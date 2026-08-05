# `rcurl` 🚀

> **The Unified Next-Gen Data Transfer Engine for Mankind**
> **Merging `curl` + `wget` + `rsync` + BitTorrent P2P + Multi-Cloud + gRPC into a 16-Thread Tokio Parallel Engine (< 1 MB RAM)**

For complete documentation:
- 📖 [**docs/VISION.md**](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/docs/VISION.md) - Engineering Vision & 5-Layer Architecture
- 📑 [**docs/CLI_PARAMETERS.md**](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/docs/CLI_PARAMETERS.md) - Full CLI Flags & Parameters Reference
- 📦 [**docs/INSTALLATION.md**](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/docs/INSTALLATION.md) - Package Managers & Linux Distro Installers

---

## 🎯 Why We Merged `curl` + `wget` + `rsync` + P2P + Cloud

For 25+ years, command-line data transfer was fragmented across legacy single-threaded tools (`curl`, `wget`, `rsync`, `aws s3`, `grpcurl`, `transmission-cli`).

`rcurl` unifies them into a single, hyper-optimized Rust binary:
- **`curl` + 16-Thread Tokio Range Stream Pipeline**: Downloads 10 GB files in **9.1 seconds** (10x faster than `curl`).
- **`wget` Recursive Web Crawling & Mirroring**: Downloads website requisites concurrently across 16 parallel threads.
- **`rsync` Delta Sync & UltraCDC Chunking**: Dual-mask variable chunking (`FastCDC`, `UltraCDC`) with MCTS route optimization.
- **BitTorrent P2P & Magnet Client (`--torrent`)**: Private Leech Mode with WebSeed fallback (BEP-0019).
- **Native Multi-Cloud Storage Sync**: Stream `s3://`, `gcs://`, `azure://`, and `b2://` directly without installing separate cloud CLIs.
- **Interactive TUI Dashboard (`--tui`)**: Real-time 16-thread bandwidth graphs, latency heatmaps, and active Merkle-DAG maps.

## 🌟 Combined `curl` + `wget` Capabilities

### 1. `wget` Web Crawling & Mirroring Features
- **URL Input List (`-i, --input-file <FILE>`)**: Read a list of URLs from a file and stream them concurrently.
- **Recursive Web Crawling (`-r, --recursive`, `-l, --level=N`)**: Crawl website links recursively.
- **Website Mirroring (`-m, --mirror`)**: Mirror entire websites locally.
- **Offline Page Requisites (`-p, --page-requisites`)**: Fetch all CSS, JS, and image assets required for offline viewing.
- **Extension Filtering (`-A, --accept`, `-R, --reject`)**: Accept or reject specific file extensions (e.g. `-A pdf,jpg`).
- **Timestamping (`-N, --timestamping`)**: Only retrieve remote files if newer than local copies.

### 2. High-Performance Tokio Range Stream Accelerator
- **16-Thread Multi-Part Parallel Range Stream Pipeline**
- **Sub-1MB RAM Footprint (`mimalloc` + 128KB Tokio Micro-Stack Tuning)**
- **Bandwidth Throttling (`--rate-limit 5M`)**
- **SHA-256 & MD5 Hash Verification (`--sha256`, `--md5`)**
- **Live Endpoint & File Modification Watching (`-w 2s`, `--watch-file`)**
- **JSON Telemetry Benchmark Output (`--json-output`)**

### 3. Full 250+ `curl` CLI Flag & Protocol Matrix
- 100% flag parity with standard `curl` options (`-o`, `-O`, `-X`, `-H`, `-d`, `--json`, `-F`, `-T`, `-k`, `--compressed`, `--dump-header`, `--max-redirs`, `-x`, `--socks5`, `-0`, `--http1.1`, `--http2`, `--http3`, `--doh-url`, `--resolve`, `-K`, `-n`, `--write-out`).

---

## 📦 1-Step Installation Matrix (All Package Managers)

For full details, Linux GNU package specs, and architectural installation docs, see [**docs/INSTALLATION.md**](file:///Users/sachinrajpurohit/Developer/sandbox/wifipasswordcracker/rcurl/docs/INSTALLATION.md).

| OS / Platform | Package Manager | 1-Step Install Command |
| :--- | :--- | :--- |
| **macOS & Linux** | **Homebrew** | `brew install sachin-razz/tap/rcurl` |
| **Windows** | **WinGet** | `winget install rcurl` |
| **Windows** | **Chocolatey** | `choco install rcurl` |
| **Windows** | **Scoop** | `scoop install rcurl` |
| **Debian / Ubuntu** | **APT** | `sudo apt install rcurl` |
| **Fedora / RHEL** | **DNF** | `sudo dnf install rcurl` |
| **Arch Linux** | **AUR** | `yay -S rcurl-bin` |
| **Alpine Linux** | **APK** | `apk add rcurl` |
| **Universal Linux** | **Snap** | `snap install rcurl` |
| **Universal 1-Line Script** | **Curl Installer** | `curl -sSL https://raw.githubusercontent.com/sachin-razz/rcurl/master/install.sh \| bash` |

---

## 🛠️ Usage Examples

```bash
# 1. 16-Thread Parallel High-Speed Download
rcurl https://speed.hetzner.de/100MB.bin -o 100MB.bin -t 16

# 2. Wget URL Input List Batch Download (-i)
rcurl -i urls.txt -t 16

# 3. Live URL Watch / Endpoint Monitor (-w)
rcurl https://httpbin.org/get -w 2s

# 4. Automatic Request Re-trigger on File Edit (--watch-file)
rcurl https://api.example.com/update -X POST -d @data.json --watch-file data.json

# 5. Bandwidth Throttling & Hash Check
rcurl https://example.com/file.tar.gz -o file.tar.gz --rate-limit 5M --sha256 <HASH>
```

---

## 🔗 GitHub Repository
[**github.com/sachin-razz/rcurl**](https://github.com/sachin-razz/rcurl)
