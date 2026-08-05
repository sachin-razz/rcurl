# `rcurl` 🚀

> **Brutally Optimized 16-Thread Tokio Parallel Streaming CLI HTTP Client & Monitor**

`rcurl` is an ultra-fast, memory-efficient command-line HTTP client built with **Rust**, **`mimalloc`**, and powered by a **16-Thread Tokio Async Streaming Engine**. It acts as a multi-part parallel streaming accelerator and endpoint monitor with low RAM usage (< 3 MB), rate-limiting, checksum verification, proxy routing, and real-time watching.

---

## ✨ Features

- **⚡ 16-Thread Parallel Range Downloader**: Automatically splits files into 16 byte range chunks and streams them in parallel using OS offset writes.
- **🚀 `mimalloc` Memory Allocator**: Uses Microsoft `mimalloc` for sub-nanosecond thread allocations with < 3 MB RAM footprint.
- **👀 Live Watch & Polling**:
  - **URL Watch (`-w / --watch 2s`)**: Continuously poll an endpoint every $N$ seconds.
  - **File Change Watch (`--watch-file <FILE>`)**: Re-trigger HTTP requests automatically whenever a local file is modified on disk.
- **⚡ Bandwidth Throttling (`--rate-limit 5M`)**: Restrict max speed (e.g. `5M`, `500K`) to avoid network saturation.
- **🔐 On-the-Fly Hash Check (`--sha256` / `--md5`)**: Compute checksums on the fly during streaming without extra disk reads.
- **🌐 SOCKS5 & HTTP Proxy Support (`-x, --proxy`)**: Route parallel streams through SOCKS5, HTTP, or HTTPS proxies.
- **📊 JSON Telemetry (`--json`)**: Output structured transfer metrics (speed, status code, elapsed time, hashes) for scripts and CI/CD pipelines.
- **📝 Dotfile Config (`~/.rcurlrc`)**: Store default headers, Bearer tokens, thread count, and proxies.
- **🛡️ Full Curl Option Support**: `--json`, `-T / --upload-file`, `-k / --insecure`, `--compressed`, `--dump-header`, `--max-redirs`, `--connect-timeout`, `-N / --no-buffer`, `--cacert`, `--cert`, `--key`.

---

## 🛠️ Options Reference Matrix

| Option | Long Flag | Description |
|---|---|---|
| `-w` | `--watch <INTERVAL>` | Periodically poll URL every $N$ seconds (`2s`, `500ms`, `1m`) |
| `--watch-file` | `--watch-file <FILE>` | Re-trigger HTTP request whenever target file changes on disk |
| `--rate-limit` | `--rate-limit <SPEED>`| Bandwidth rate limiter (`5M`, `500K`) |
| `--sha256` | `--sha256 <HASH>` | Verify SHA-256 hash on completion |
| `--md5` | `--md5 <HASH>` | Verify MD5 hash on completion |
| `-x` | `--proxy <URL>` | Proxy server (`socks5://127.0.0.1:9050`, `http://proxy:8080`) |
| `--json` | `--json` | Output benchmark metrics as structured JSON |
| `--json` | `--json <PAYLOAD>` | Send JSON body and inject `Content-Type: application/json` |
| `-T` | `--upload-file <FILE>`| Upload local file to remote server via PUT |
| `-k` | `--insecure` | Skip SSL certificate verification |
| `--compressed` | `--compressed` | Request compressed response (`gzip`, `brotli`, `deflate`) |
| `--dump-header` | `--dump-header <FILE>`| Dump raw response headers to a file |
| `--max-redirs` | `--max-redirs <NUM>` | Limit maximum HTTP redirects (default `50`) |
| `--connect-timeout`| `--connect-timeout <SECS>`| Connection phase timeout |
| `-N` | `--no-buffer` | Disable stdout buffering for live SSE streaming |
| `-t` | `--threads <NUM>` | Number of Tokio worker threads / streams (default: `16`) |
| `-o` | `--output <FILE>` | Write response to destination file |
| `-O` | `--remote-name` | Save file using remote filename |
| `-X` | `--request <METHOD>` | HTTP method (`GET`, `POST`, `PUT`, `DELETE`, etc.) |
| `-H` | `--header <HEADER>` | Custom header (e.g. `-H "Authorization: Bearer token"`) |
| `-d` | `--data <DATA>` | POST / PUT body data payload |
| `-L` | `--location` | Follow HTTP redirects |
| `-v` | `--verbose` | Print request/response headers & trace |

---

## 🔗 GitHub Repository
[**github.com/sachin-razz/rcurl**](https://github.com/sachin-razz/rcurl)
