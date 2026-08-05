#!/usr/bin/env bash
set -e

echo "🚀 Installing rcurl & Shell Completions (Zsh, Bash, Fish)..."

# Build release binary and generate completions
cargo build --release
cargo run --bin gen_completions

# Install Binary
INSTALL_DIR="/usr/local/bin"
if [ -w "$INSTALL_DIR" ]; then
    cp target/release/rcurl "$INSTALL_DIR/rcurl"
else
    sudo cp target/release/rcurl "$INSTALL_DIR/rcurl"
fi

echo "✅ rcurl binary installed to $INSTALL_DIR/rcurl"

# Install Zsh Completions
ZSH_COMP_DIR="$HOME/.zsh/completions"
mkdir -p "$ZSH_COMP_DIR"
cp completions/_rcurl "$ZSH_COMP_DIR/_rcurl"

# Add Zsh fpath if not present in ~/.zshrc
if ! grep -q "fpath=(~/.zsh/completions" "$HOME/.zshrc" 2>/dev/null; then
    echo '' >> "$HOME/.zshrc"
    echo '# rcurl Zsh completions' >> "$HOME/.zshrc"
    echo 'fpath=(~/.zsh/completions $fpath)' >> "$HOME/.zshrc"
    echo 'autoload -U compinit && compinit' >> "$HOME/.zshrc"
fi

echo "✅ Zsh auto-completions installed to $ZSH_COMP_DIR/_rcurl"
echo "🎉 rcurl installation complete! Run 'rcurl --help' or 'source ~/.zshrc' to use tab completions."
