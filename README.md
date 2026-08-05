# `rcurl` 🚀

> **High-Performance 16-Thread Tokio Streaming HTTP CLI Client**

`rcurl` is an ultra-fast, memory-efficient command-line HTTP client built with **Rust** and powered by a **16-Thread Tokio Async Streaming Engine**. It acts as a high-speed replacement for traditional `curl` with low memory usage, parallel stream processing, and interactive progress bars.

---

## ✨ Features

- **⚡ 16-Thread Tokio Runtime**: Configured by default with 16 multi-threaded Tokio workers for high-concurrency stream operations.
- **🛡️ Memory Leak Free**: Uses zero-allocation chunk streaming via `tokio::io::AsyncWrite` directly to disk/stdout. RAM usage remains flat (< 5 MB) even when streaming multi-gigabyte payloads.
- **🔄 Auto-Resume & Range Transfers**: Resume interrupted transfers automatically with `-C / --continue-at auto`.
- **📊 Real-time Progress Bar**: Multi-progress visualization powered by `indicatif` with memory-cleaned progress bar lifecycles.
- **⚙️ Full Curl Syntax**: Supports `-X`, `-H`, `-d`, `-o`, `-O`, `-L`, `-v`, `-i`, `-s`, `-u`, `-A`, `-m`, and `--retry`.
- **🔀 Multi-URL Parallel Fetching**: Stream multiple URLs concurrently across worker threads.

---

## 📦 Installation

### From Source

```bash
git clone https://github.com/YOUR_USERNAME/rcurl.git
cd rcurl
cargo install --path .
```

### Build Release Binary

```bash
cargo build --release
# Output binary location: ./target/release/rcurl
```

---

## 🛠️ Usage Examples

### 1. Simple GET Request (Output to Stdout)
```bash
rcurl https://httpbin.org/get
```

### 2. Download File to Disk with Progress Bar (-o / -O)
```bash
rcurl https://speed.hetzner.de/100MB.bin -o 100MB.bin
```

### 3. High-Concurrency Multi-Thread Streaming (-t 16)
```bash
rcurl https://example.com/large-dataset.tar.gz -o dataset.tar.gz -t 16
```

### 4. Send POST Request with JSON Body (-X POST -H -d)
```bash
rcurl https://httpbin.org/post \
  -X POST \
  -H "Content-Type: application/json" \
  -d '{"name": "rcurl", "speed": "fast"}'
```

### 5. Resume Interrupted Download (-C auto)
```bash
rcurl https://speed.hetzner.de/100MB.bin -o 100MB.bin -C auto
```

### 6. Verbose Header Trace (-v)
```bash
rcurl https://httpbin.org/headers -v
```

---

## 📋 CLI Options Reference

| Option | Long Flag | Description |
|---|---|---|
| `-t` | `--threads <NUM>` | Number of Tokio worker threads / streams (default: `16`) |
| `-o` | `--output <FILE>` | Write response to file instead of stdout |
| `-O` | `--remote-name` | Write response to file named after remote filename |
| `-X` | `--request <METHOD>` | HTTP method (`GET`, `POST`, `PUT`, `DELETE`, etc.) |
| `-H` | `--header <HEADER>` | Custom header (e.g. `-H "Authorization: Bearer token"`) |
| `-d` | `--data <DATA>` | POST / PUT body data payload |
| `-L` | `--location` | Follow HTTP redirects (default: `true`) |
| `-v` | `--verbose` | Print request/response headers & trace |
| `-i` | `--include` | Include HTTP response headers in output |
| `-s` | `--silent` | Hide progress bar and status messages |
| `-C` | `--continue-at <OFFSET>`| Resume offset (`auto` or byte index) |
| `-u` | `--user <USER:PASS>` | Basic authentication credentials |
| `-A` | `--user-agent <STRING>`| Custom User-Agent header |
| `-m` | `--max-time <SECS>` | Timeout duration in seconds |
| `--retry` | `--retry <NUM>` | Number of connection failure retries (default: `3`) |

---

## 📄 License

MIT License © 2026 Sachin Rajpurohit
