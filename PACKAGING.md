# Guia de Empacotamento

## Pacote DEB (Debian/Ubuntu)

### Pré-requisitos

```bash
cargo install cargo-deb
```

### Build Automático

Use o script fornecido:

```bash
./build-deb.sh
```

### Build Manual

```bash
cargo build --release
cargo deb
```

O pacote será criado em `target/debian/extra-cosmic-xkill_0.1.0_amd64.deb`.

### Instalação

```bash
sudo dpkg -i target/debian/extra-cosmic-xkill_0.1.0_amd64.deb
```

### Resolução de Dependências

Se houver dependências não satisfeitas:

```bash
sudo apt --fix-broken install
```

### Verificação

```bash
dpkg -L extra-cosmic-xkill
```

## Pacote RPM (Fedora/RHEL)

### Pré-requisitos

```bash
cargo install cargo-rpm
```

### Build

```bash
cargo build --release
cargo rpm build
```

## Pacote AUR (Arch Linux)

### PKGBUILD

Crie um arquivo `PKGBUILD`:

```bash
pkgname=extra-cosmic-xkill
pkgver=0.1.0
pkgrel=1
pkgdesc="Applet sistema para gerenciamento de janelas X11/Wayland"
arch=('x86_64')
url="https://github.com/vitoriamaria/extra-cosmic-xkill-applet"
license=('MIT')
depends=('gtk3' 'libx11' 'wayland')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$pkgname-$pkgver"
    install -Dm755 target/release/xkill-applet "$pkgdir/usr/bin/xkill-applet"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
}
```

### Build

```bash
makepkg -si
```

## AppImage

### Pré-requisitos

```bash
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x linuxdeploy-x86_64.AppImage
```

### Build

```bash
cargo build --release

mkdir -p AppDir/usr/bin
cp target/release/xkill-applet AppDir/usr/bin/

./linuxdeploy-x86_64.AppImage \
    --appdir AppDir \
    --output appimage
```

## Flatpak

### Manifest

Crie `com.vitoriamaria.XKill.yml`:

```yaml
app-id: com.vitoriamaria.XKill
runtime: org.gnome.Platform
runtime-version: '42'
sdk: org.gnome.Sdk
sdk-extensions:
  - org.freedesktop.Sdk.Extension.rust-stable
command: xkill-applet
finish-args:
  - --socket=x11
  - --socket=wayland
  - --share=ipc
modules:
  - name: xkill-applet
    buildsystem: simple
    build-commands:
      - cargo build --release
      - install -Dm755 target/release/xkill-applet /app/bin/xkill-applet
    sources:
      - type: dir
        path: .
```

### Build

```bash
flatpak-builder build com.vitoriamaria.XKill.yml
```

## Snap

### Snapcraft.yaml

```yaml
name: extra-cosmic-xkill
version: '0.1.0'
summary: Applet sistema para gerenciamento de janelas
description: |
  Extra Cosmic XKill é um applet sistema em Rust para gerenciamento
  de janelas X11/Wayland.

grade: stable
confinement: classic
base: core20

parts:
  xkill-applet:
    plugin: rust
    source: .

apps:
  xkill-applet:
    command: bin/xkill-applet
```

### Build

```bash
snapcraft
```

## Distribuição

### GitHub Releases

```bash
gh release create v0.1.0 \
    target/debian/*.deb \
    target/release/xkill-applet \
    --title "Release v0.1.0" \
    --notes "Initial release"
```

### Upload para Repositórios

#### Debian/Ubuntu PPA

1. Configure GPG key
2. Build source package
3. Upload com dput

#### AUR

1. Clone aur.git
2. Update PKGBUILD
3. Commit e push

## Verificação de Qualidade

### Lintian (Debian)

```bash
lintian target/debian/*.deb
```

### RPMLint (RPM)

```bash
rpmlint target/rpm/RPMS/x86_64/*.rpm
```

## Assinaturas

### GPG Sign DEB

```bash
dpkg-sig --sign builder target/debian/*.deb
```

### Verify

```bash
dpkg-sig --verify target/debian/*.deb
```
