const NOTIFICATION_BRIDGE_SCRIPT: &str = r#"
(function() {
    // Guarda a referência original
    const OriginalNotification = window.Notification;

    // Cria uma classe que imita a API de Notification
    class TauriNotification {
        static permission = 'granted';

        static async requestPermission() {
            try {
                const { isPermissionGranted, requestPermission } = await window.__TAURI__.notification;
                let granted = await isPermissionGranted();
                if (!granted) {
                    const result = await requestPermission();
                    granted = result === 'granted';
                }
                TauriNotification.permission = granted ? 'granted' : 'denied';
                return TauriNotification.permission;
            } catch (e) {
                console.warn('[Weve Desktop] Notification permission error:', e);
                return 'denied';
            }
        }

        constructor(title, options = {}) {
            this.title = title;
            this.body = options.body || '';
            this.icon = options.icon || '';
            this.tag = options.tag || '';
            this.onclick = null;
            this.onclose = null;
            this.onerror = null;
            this.onshow = null;

            this._send();
        }

        async _send() {
            try {
                const { sendNotification } = await window.__TAURI__.notification;
                await sendNotification({
                    title: this.title,
                    body: this.body,
                });
                if (this.onshow) this.onshow();
            } catch (e) {
                console.warn('[Weve Desktop] Failed to send notification:', e);
                if (this.onerror) this.onerror(e);
            }
        }

        close() {
            if (this.onclose) this.onclose();
        }
    }

    // Substitui a API de Notification
    if (window.__TAURI__) {
        window.Notification = TauriNotification;
        console.log('[Weve Desktop] Notification API bridged to native notifications');
    }
})();
"#;

use tauri_plugin_updater::UpdaterExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Cria a janela manualmente com o script de inicialização
            let _window = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://app.useweve.com".parse().unwrap()),
            )
            .title("Weve")
            .inner_size(1280.0, 800.0)
            .initialization_script(NOTIFICATION_BRIDGE_SCRIPT)
            .build()?;

            // Verifica atualizações em background
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = check_for_updates(handle).await {
                    eprintln!("Erro ao verificar atualizações: {}", e);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Erro ao iniciar a aplicação");
}

async fn check_for_updates(app: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(update) = app.updater()?.check().await? {
        println!("Atualização disponível: {}", update.version);

        // Baixa e instala a atualização
        let mut downloaded = 0;
        update.download_and_install(
            |chunk_length, content_length| {
                downloaded += chunk_length;
                println!("Baixando {} de {:?} bytes", downloaded, content_length);
            },
            || {
                println!("Download concluído, preparando instalação...");
            },
        ).await?;

        println!("Atualização instalada, reiniciando...");
        app.restart();
    }

    Ok(())
}
