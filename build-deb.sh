#!/bin/bash

set -e

echo "Construindo pacote DEB para Extra Cosmic XKill..."

if ! command -v cargo &> /dev/null; then
    echo "Erro: cargo não encontrado. Instale o Rust primeiro."
    exit 1
fi

if ! cargo install --list | grep -q "cargo-deb"; then
    echo "Instalando cargo-deb..."
    cargo install cargo-deb
fi

echo "Compilando release..."
cargo build --release

echo "Gerando pacote DEB..."
cargo deb

echo ""
echo "Pacote DEB criado com sucesso!"
echo "Localização: target/debian/"
ls -lh target/debian/*.deb

echo ""
echo "Para instalar:"
echo "  sudo dpkg -i target/debian/*.deb"
echo ""
echo "Para remover:"
echo "  sudo apt remove extra-cosmic-xkill"
