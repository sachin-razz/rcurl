# 📦 `rcurl` Universal Installation & Package Management Guide

This document provides complete installation instructions for `rcurl` across **all Linux GNU distributions, macOS, and Windows**, alongside an explanation of the underlying Rust compilation directives and memory architecture.

---

## 📖 Table of Contents
1. [Why Multi-Distribution Package Managers are Essential](#1-why-multi-distribution-package-managers-are-essential)
2. [GNU/Linux Installation Matrix](#2-gnulinux-installation-matrix)
3. [macOS & Windows Installation Matrix](#3-macos--windows-installation-matrix)
4. [Universal 1-Line Install Script](#4-universal-1-line-install-script)
5. [Rust `#[allow(dead_code)]` Directive Explained](#5-rust-allowdead_code-directive-explained)

---

## 1. Why Multi-Distribution Package Managers Are Essential

`rcurl` is not just a standard HTTP client—it is a **16-Thread Tokio Parallel Streaming Engine** equipped with server daemons (`--transfer-server`, `--mitm-proxy`, `--rsyncd`), P2P swarms (`--torrent`, `--p2p-mesh`), and kernel socket accelerators (`--ebpf-accelerator`).

Because Linux servers run across diverse GNU distributions (Debian, Ubuntu, Fedora, RHEL, CentOS, Arch, Alpine), providing native package manifests (`.deb`, `.rpm`, `PKGBUILD`, `APKBUILD`, `Snap`) ensures:
- **Zero-Dependency Setup**: Users don't need to install Rust or Cargo to run `rcurl`.
- **System Service Integration**: Pre-compiled binaries run with native `systemd` and OS security profiles.
- **Automated Updates**: Package managers (`apt update`, `dnf upgrade`, `brew upgrade`) automatically pull new `rcurl` releases.

---

## 2. GNU/Linux Installation Matrix

### 🐧 Debian & Ubuntu (`.deb` / `apt`)
```bash
# Download and install the pre-compiled Debian package
curl -LO https://github.com/sachin-razz/rcurl/releases/latest/download/rcurl-linux-x86_64.tar.gz
tar -xzf rcurl-linux-x86_64.tar.gz
sudo cp rcurl /usr/local/bin/
```
*(Package Control Spec: [`packaging/debian/control`](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/packaging/debian/control))*

---

### 🎩 Fedora, RHEL, CentOS (`.rpm` / `dnf` / `yum`)
```bash
# Install via DNF package manager
sudo dnf install rcurl
```
*(RPM Spec File: [`packaging/rcurl.spec`](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/packaging/rcurl.spec))*

---

### 🏹 Arch Linux, Manjaro, EndeavourOS (`AUR`)
```bash
# Install via AUR helper (yay or paru)
yay -S rcurl-bin
```
*(PKGBUILD Spec: [`packaging/PKGBUILD`](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/packaging/PKGBUILD))*

---

### 🏔️ Alpine Linux (`apk`)
```bash
# Install via Alpine Package Keeper
sudo apk add rcurl
```
*(APKBUILD Spec: [`packaging/APKBUILD`](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/packaging/APKBUILD))*

---

### ⚡ Universal Linux Snapcraft (`snap`)
```bash
# Install via Ubuntu Snap
sudo snap install rcurl
```
*(Snapcraft Spec: [`snap/snapcraft.yaml`](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/snap/snapcraft.yaml))*

---

## 3. macOS & Windows Installation Matrix

### 🍏 macOS & Linuxbrew (`Homebrew`)
```bash
brew install sachin-razz/tap/rcurl
```
*(Formula: [`Formula/rcurl.rb`](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/Formula/rcurl.rb))*

---

### 🪟 Windows (`WinGet` / `Chocolatey` / `Scoop`)
```powershell
# WinGet (Native Windows Package Manager)
winget install rcurl

# Chocolatey
choco install rcurl

# Scoop
scoop install rcurl
```

---

## 4. Universal 1-Line Install Script

For any Linux or macOS machine without Cargo or package managers:

```bash
curl -sSL https://raw.githubusercontent.com/sachin-razz/rcurl/master/install.sh | bash
```

This installer automatically detects your operating system and CPU architecture (x86_64, ARM64), downloads the pre-compiled binary from GitHub Releases, installs it to `/usr/local/bin/rcurl`, and configures Zsh tab completions!

---

## 5. Rust `#[allow(dead_code)]` Directive Explained

### ❓ What does `#[allow(dead_code)]` mean?

In Rust, the compiler enforces strict code quality lints. By default, if a function, struct, or variable is written but not directly called in the main binary entry point, Rust emits a compiler warning: `warning: function is never used (dead_code)`.

`#[allow(dead_code)]` is an explicit attribute placed above functions or module structs (like `parse_rate_limit()` or `parse_interval()`) that tells the Rust compiler:

> *"This code symbol is deliberately created as part of the library API, test suite, or future CLI options. Do not emit a warning."*

### 💡 Why is it useful in `rcurl`?
1. **Public Library API**: `rcurl` is both a CLI binary and a Rust crate library (`src/lib.rs`). Functions like `parse_rate_limit()` are exported for third-party Rust applications calling `rcurl` as a dependency.
2. **Modular Architecture**: Prevents compiler noise while maintaining helper utilities for protocols, CDC chunking, and memory engines.
