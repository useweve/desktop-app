const INIT_SCRIPT: &str = r#"
(function() {
    // === NOTIFICATION BRIDGE ===
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
                console.warn('[Weve Desktop] Erro de permissão de notificação:', e);
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
                await sendNotification({ title: this.title, body: this.body });
                if (this.onshow) this.onshow();
            } catch (e) {
                console.warn('[Weve Desktop] Falha ao enviar notificação:', e);
                if (this.onerror) this.onerror(e);
            }
        }

        close() {
            if (this.onclose) this.onclose();
        }
    }

    // === INTERCEPTOR DE NOVA JANELA ===
    // Intercepta window.open
    const originalOpen = window.open;
    window.open = function(url, target, features) {
        if (url && window.__TAURI__) {
            window.__TAURI__.core.invoke('open_new_window', { url: url.toString() });
            return null;
        }
        return originalOpen.call(window, url, target, features);
    };

    // Intercepta cliques em links target="_blank"
    document.addEventListener('click', function(e) {
        const link = e.target.closest('a[target="_blank"]');
        if (link && link.href && window.__TAURI__) {
            e.preventDefault();
            window.__TAURI__.core.invoke('open_new_window', { url: link.href });
        }
    }, true);

    // Aplica as customizações
    if (window.__TAURI__) {
        window.Notification = TauriNotification;
        console.log('[Weve Desktop] Inicializado com sucesso');
    }
})();
"#;

use tauri_plugin_updater::UpdaterExt;
use std::sync::atomic::{AtomicUsize, Ordering};

static WINDOW_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn create_window(app: &tauri::AppHandle, url: &str) -> tauri::Result<tauri::WebviewWindow> {
    let count = WINDOW_COUNTER.fetch_add(1, Ordering::SeqCst);
    let label = if count == 0 {
        "main".to_string()
    } else {
        format!("window-{}", count)
    };

    tauri::WebviewWindowBuilder::new(
        app,
        &label,
        tauri::WebviewUrl::External(url.parse().unwrap()),
    )
    .title("Weve")
    .inner_size(1280.0, 800.0)
    .initialization_script(INIT_SCRIPT)
    .on_navigation(|url| {
        // Permite navegação apenas para domínios da Weve
        url.host_str().map_or(false, |host| {
            host == "app.useweve.com" || host.ends_with(".useweve.com")
        })
    })
    .build()
}

#[tauri::command]
fn open_new_window(app: tauri::AppHandle, url: String) {
    // Verifica se é um domínio permitido
    if let Ok(parsed) = url::Url::parse(&url) {
        let is_weve = parsed.host_str().map_or(false, |host| {
            host == "app.useweve.com" || host.ends_with(".useweve.com")
        });

        if is_weve {
            // Abre nova janela do app
            let _ = create_window(&app, &url);
        } else {
            // Abre no navegador do sistema
            let _ = tauri_plugin_opener::open_url(&url, None::<&str>);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![open_new_window])
        .setup(|app| {
            // Cria a janela principal
            let _window = create_window(app.handle(), "https://app.useweve.com")?;

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
