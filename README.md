# Extra Cosmic XKill Applet

Applet sistema em Rust para gerenciamento de janelas X11/Wayland.

## Funcionalidades

- Fechar janelas através de clique (similar ao xkill)
- Suporte a X11 e detecção de Wayland
- Interface gráfica GTK
- Modo CLI disponível
- Configuração via arquivo TOML
- Notificações do sistema
- Atalhos de teclado configuráveis

## Requisitos

- Rust 1.57 ou superior
- Bibliotecas de desenvolvimento:
  - libx11-dev
  - libgtk-3-dev
  - libwayland-dev

### Instalação de Dependências

**Debian/Ubuntu:**
```bash
sudo apt install libx11-dev libgtk-3-dev libwayland-dev pkg-config
```

**Fedora:**
```bash
sudo dnf install libX11-devel gtk3-devel wayland-devel pkg-config
```

**Arch Linux:**
```bash
sudo pacman -S libx11 gtk3 wayland pkg-config
```

## Compilação

```bash
cargo build --release
```

O binário será gerado em `target/release/xkill-applet`.

## Instalação

```bash
cargo install --path .
```

Ou copiar manualmente:
```bash
sudo cp target/release/xkill-applet /usr/local/bin/
```

## Uso

### Modo Gráfico (padrão)

```bash
xkill-applet
```

### Modo CLI

```bash
xkill-applet --cli
```

No modo CLI, clique na janela que deseja fechar. Pressione qualquer tecla para cancelar.

## Configuração

O arquivo de configuração está localizado em:
- Linux: `~/.config/extra-cosmic-xkill/config.toml`

Veja `config.example.toml` para exemplo de configuração.

### Opções de Configuração

```toml
[general]
auto_start = false              # Iniciar com o sistema
show_notifications = true       # Exibir notificações
prefer_wayland = false          # Preferir Wayland quando disponível

[ui]
theme = "default"               # Tema da interface
show_window_names = true        # Mostrar nomes das janelas
window_width = 350              # Largura da janela
window_height = 200             # Altura da janela

[hotkey]
enabled = true                  # Ativar atalho de teclado
modifiers = ["Ctrl", "Alt"]     # Teclas modificadoras
key = "K"                       # Tecla principal
```

## Testes

```bash
cargo test
```

## Licença

MIT

## Autor

Vitória Maria
