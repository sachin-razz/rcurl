#!/usr/bin/env bash
set -e

echo "🚀 Installing rcurl CLI..."

REPO="sachin-razz/rcurl"
INSTALL_DIR="/usr/local/bin"

if command -v cargo >/dev/null 2>&1; then
    echo "📦 Building rcurl from source via cargo..."
    cargo build --release
    cargo run --bin gen_completions
    BINARY_PATH="target/release/rcurl"
else
    echo "📦 Fetching pre-built binary release from GitHub..."
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    ARCH="$(uname -m)"

    if [ "$OS" = "darwin" ]; then
        if [ "$ARCH" = "arm64" ]; then
            TAG="rcurl-macos-arm64.tar.gz"
        else
            TAG="rcurl-macos-x86_64.tar.gz"
        fi
    elif [ "$OS" = "linux" ]; then
        TAG="rcurl-linux-x86_64.tar.gz"
    else
        echo "❌ Unsupported OS for automated curl install: $OS"
        exit 1
    fi

    DOWNLOAD_URL="https://github.com/${REPO}/releases/latest/download/${TAG}"
    echo "📥 Downloading pre-compiled binary: $DOWNLOAD_URL"
    curl -sSL "$DOWNLOAD_URL" | tar -xz
    BINARY_PATH="rcurl"
fi

# Install Binary
if [ -w "$INSTALL_DIR" ]; then
    cp "$BINARY_PATH" "$INSTALL_DIR/rcurl"
else
    sudo cp "$BINARY_PATH" "$INSTALL_DIR/rcurl"
fi

echo "✅ rcurl binary installed to $INSTALL_DIR/rcurl"

# Install Zsh Completions
ZSH_COMP_DIR="$HOME/.zsh/completions"
mkdir -p "$ZSH_COMP_DIR"
if [ -f "completions/_rcurl" ]; then
    cp completions/_rcurl "$ZSH_COMP_DIR/_rcurl"

    if ! grep -q "fpath=(~/.zsh/completions" "$HOME/.zshrc" 2>/dev/null; then
        echo '' >> "$HOME/.zshrc"
        echo '# rcurl Zsh completions' >> "$HOME/.zshrc"
        echo 'fpath=(~/.zsh/completions $fpath)' >> "$HOME/.zshrc"
        echo 'autoload -U compinit && compinit' >> "$HOME/.zshrc"
    fi
    echo "✅ Zsh auto-completions installed to $ZSH_COMP_DIR/_rcurl"
fi

echo "🎉 rcurl installation complete! Run 'rcurl --help' to use."
