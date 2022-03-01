# Guia de Instalação - Extra Cosmic XKill

## Pré-requisitos

### Sistema Operacional

- Linux com suporte a X11 ou Wayland
- Testado em: Ubuntu 20.04+, Fedora 34+, Arch Linux

### Ferramentas de Build

- Rust 1.57 ou superior
- Cargo (gerenciador de pacotes Rust)
- pkg-config
- gcc/g++

## Instalação do Rust

Se você ainda não tem o Rust instalado:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verifique a instalação:
```bash
rustc --version
cargo --version
```

## Dependências do Sistema

### Ubuntu/Debian

```bash
sudo apt update
sudo apt install -y \
    libx11-dev \
    libgtk-3-dev \
    libwayland-dev \
    pkg-config \
    build-essential
```

### Fedora

```bash
sudo dnf install -y \
    libX11-devel \
    gtk3-devel \
    wayland-devel \
    pkg-config \
    gcc \
    gcc-c++
```

### Arch Linux

```bash
sudo pacman -S --needed \
    libx11 \
    gtk3 \
    wayland \
    pkg-config \
    base-devel
```

## Compilação e Instalação

### Método 1: Instalação via Cargo

```bash
git clone https://github.com/vitoriamaria/extra-cosmic-xkill-applet.git
cd extra-cosmic-xkill-applet
cargo install --path .
```

O binário será instalado em `~/.cargo/bin/xkill-applet`.

Certifique-se de que `~/.cargo/bin` está no seu PATH:
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Método 2: Instalação Manual

```bash
git clone https://github.com/vitoriamaria/extra-cosmic-xkill-applet.git
cd extra-cosmic-xkill-applet
cargo build --release
sudo cp target/release/xkill-applet /usr/local/bin/
```

### Método 3: Instalação a partir do Pacote DEB

```bash
cargo install cargo-deb
cargo deb
sudo dpkg -i target/debian/*.deb
```

## Verificação da Instalação

```bash
xkill-applet --version
```

## Configuração Inicial

Criar arquivo de configuração:
```bash
mkdir -p ~/.config/extra-cosmic-xkill
cp config.example.toml ~/.config/extra-cosmic-xkill/config.toml
```

Edite o arquivo conforme necessário:
```bash
nano ~/.config/extra-cosmic-xkill/config.toml
```

## Executando

### Modo Gráfico
```bash
xkill-applet
```

### Modo CLI
```bash
xkill-applet --cli
```

## Auto-inicialização (Opcional)

Para iniciar automaticamente com o sistema:

### GNOME
```bash
cat > ~/.config/autostart/xkill-applet.desktop << 'AUTOSTART'
[Desktop Entry]
Type=Application
Name=Extra Cosmic XKill
Exec=/usr/local/bin/xkill-applet
Hidden=false
NoDisplay=false
X-GNOME-Autostart-enabled=true
AUTOSTART
```

### systemd (usuário)
```bash
mkdir -p ~/.config/systemd/user
cat > ~/.config/systemd/user/xkill-applet.service << 'SERVICE'
[Unit]
Description=Extra Cosmic XKill Applet
After=graphical-session.target

[Service]
ExecStart=/usr/local/bin/xkill-applet
Restart=on-failure

[Install]
WantedBy=default.target
SERVICE

systemctl --user enable xkill-applet.service
systemctl --user start xkill-applet.service
```

## Desinstalação

### Cargo
```bash
cargo uninstall xkill-applet
```

### Manual
```bash
sudo rm /usr/local/bin/xkill-applet
rm -rf ~/.config/extra-cosmic-xkill
```

### Pacote DEB
```bash
sudo apt remove extra-cosmic-xkill
```

## Troubleshooting

### Erro: failed to connect to X11 server

Certifique-se de que está executando em uma sessão gráfica X11:
```bash
echo $DISPLAY
```

### Erro: cannot open display

Execute com permissões adequadas ou verifique DISPLAY:
```bash
export DISPLAY=:0
xkill-applet
```

### Erro de compilação relacionado a GTK

Certifique-se de que todas as bibliotecas de desenvolvimento estão instaladas:
```bash
pkg-config --modversion gtk+-3.0
```

## Suporte

Para problemas, abra uma issue no GitHub:
https://github.com/vitoriamaria/extra-cosmic-xkill-applet/issues
