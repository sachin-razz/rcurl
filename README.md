# `rcurl` 🚀

> **Ultra-Fast 16-Thread Tokio Parallel Streaming CLI HTTP Client & Monitor**

`rcurl` is a high-performance, memory-efficient command-line HTTP client built with **Rust** and powered by a **16-Thread Tokio Async Streaming Engine**. It acts as a multi-part parallel streaming accelerator and endpoint monitor with low RAM usage (< 5 MB), rate-limiting, checksum verification, proxy routing, and real-time watching.

---

## ✨ Core Features

- **⚡ 16-Thread Parallel Range Downloader**: Automatically splits files into 16 byte range chunks and streams them in parallel using OS offset writes.
- **👀 Live Watch & Polling**:
  - **URL Watch (`-w / --watch 2s`)**: Continuously poll an endpoint every $N$ seconds.
  - **File Change Watch (`--watch-file <FILE>`)**: Re-trigger HTTP requests automatically whenever a local file is modified on disk.
- **⚡ Bandwidth Throttling (`--rate-limit 5M`)**: Restrict max speed (e.g. `5M`, `500K`) to avoid network saturation.
- **🔐 On-the-Fly Hash Check (`--sha256` / `--md5`)**: Compute checksums on the fly during streaming without extra disk reads.
- **🌐 SOCKS5 & HTTP Proxy Support (`-x, --proxy`)**: Route parallel streams through SOCKS5, HTTP, or HTTPS proxies.
- **📊 JSON Telemetry (`--json`)**: Output structured transfer metrics (speed, status code, elapsed time, hashes) for scripts and CI/CD pipelines.
- **📝 Dotfile Config (`~/.rcurlrc`)**: Store default headers, Bearer tokens, thread count, and proxies.
- **🛡️ Low Memory Footprint**: Strictly < 5 MB RAM usage regardless of file size.

---

## 🛠️ Usage Examples

### 1. High-Concurrency 16-Thread File Download
```bash
rcurl https://speed.hetzner.de/100MB.bin -o 100MB.bin -t 16
```

### 2. Live URL Polling / Endpoint Monitoring (-w)
```bash
# Poll endpoint every 2 seconds
rcurl https://httpbin.org/get -w 2s
```

### 3. Automatically Re-send Request When File Changes (--watch-file)
```bash
# Watch payload.json and re-send POST request whenever payload.json is edited & saved
rcurl https://api.example.com/v1/update -X POST -d @payload.json --watch-file payload.json
```

### 4. Bandwidth Rate Limiter (--rate-limit)
```bash
rcurl https://example.com/large.iso -o large.iso --rate-limit 5M
```

### 5. On-the-Fly SHA-256 Hash Verification (--sha256)
```bash
rcurl https://example.com/release.tar.gz -o release.tar.gz --sha256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

### 6. JSON Telemetry Output for Scripts (--json)
```bash
rcurl https://postman-echo.com/get --json
```

---

## 📋 Options Reference

| Option | Long Flag | Description |
|---|---|---|
| `-w` | `--watch <INTERVAL>` | Periodically poll URL every $N$ seconds (`2s`, `500ms`, `1m`) |
| `--watch-file` | `--watch-file <FILE>` | Re-trigger HTTP request whenever target file changes on disk |
| `--rate-limit` | `--rate-limit <SPEED>`| Bandwidth rate limiter (`5M`, `500K`) |
| `--sha256` | `--sha256 <HASH>` | Verify SHA-256 hash on completion |
| `--md5` | `--md5 <HASH>` | Verify MD5 hash on completion |
| `-x` | `--proxy <URL>` | Proxy server (`socks5://127.0.0.1:9050`, `http://proxy:8080`) |
| `--json` | `--json` | Output benchmark metrics as structured JSON |
| `-t` | `--threads <NUM>` | Number of Tokio worker threads / streams (default: `16`) |
| `-o` | `--output <FILE>` | Write response to destination file |
| `-O` | `--remote-name` | Save file using remote filename |
| `-X` | `--request <METHOD>` | HTTP method (`GET`, `POST`, `PUT`, `DELETE`, etc.) |
| `-H` | `--header <HEADER>` | Custom header (e.g. `-H "Authorization: Bearer token"`) |
| `-d` | `--data <DATA>` | POST / PUT body data payload |
| `-L` | `--location` | Follow HTTP redirects |
| `-v` | `--verbose` | Print request/response headers & trace |
| `-C` | `--continue-at <OFFSET>`| Resume offset (`auto` or byte index) |

---

## 🔗 Repository
[**github.com/sachin-razz/rcurl**](https://github.com/sachin-razz/rcurl)
