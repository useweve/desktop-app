# Weve Desktop

Aplicativo desktop oficial da [Weve](https://useweve.com) para macOS e Windows.

## Download

Baixe a versão mais recente na página de [Releases](https://github.com/useweve/desktop-app/releases).

| Sistema | Arquivo |
|---------|---------|
| macOS (Apple Silicon) | `Weve_x.x.x_aarch64.dmg` |
| macOS (Intel) | `Weve_x.x.x_x64.dmg` |
| Windows | `Weve_x.x.x_x64-setup.exe` |

## Instalação

### macOS

1. Baixe o arquivo `.dmg` correspondente ao seu Mac
2. Abra o arquivo `.dmg`
3. Arraste o app **Weve** para a pasta **Aplicativos**
4. Abra o **Terminal** e execute:
   ```bash
   xattr -cr /Applications/Weve.app
   ```
5. Abra o app normalmente

> **Nota:** O comando acima remove o atributo de quarentena que o macOS aplica a apps baixados da internet. Isso é necessário porque o app ainda não possui assinatura da Apple. O app é open source e você pode verificar o código.

### Windows

1. Baixe o arquivo `.exe`
2. Execute o instalador
3. Se aparecer o aviso do SmartScreen:
   - Clique em **Mais informações**
   - Clique em **Executar assim mesmo**
4. Siga as instruções do instalador

> **Nota:** O aviso do SmartScreen aparece porque o app ainda não possui assinatura digital. Isso é seguro.

## Recursos

- Acesso rápido ao Weve direto do desktop
- Notificações nativas do sistema operacional
- Atualização automática
- Links externos abrem no navegador

## Suporte

Encontrou um problema? [Abra uma issue](https://github.com/useweve/desktop-app/issues).
