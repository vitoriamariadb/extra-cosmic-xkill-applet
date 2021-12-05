# cosmic-xkill-applet
Um applet para COSMIC desktop que simula o xkill, permitindo fechar janelas com um clique.

## Instalação
1. Instale Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Instale dependências: `sudo apt install libxkbcommon-dev libwayland-dev`
3. Clone o repositório: `git clone https://github.com/seuusuario/cosmic-xkill-applet`
4. Build: `cargo build --release`
5. Instale: `sudo cp target/release/cosmic-xkill-applet /usr/local/bin/`

## Limitações
Funciona apenas para janelas X11/XWayland no COSMIC (Wayland não suporta xkill nativo).

## Licença
GPL-3.0

