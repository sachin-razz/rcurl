# `rcurl` 🚀

> **16-Thread Tokio Streaming Engine + Native C `libcurl` Integration (100% Full Curl Parity)**

`rcurl` is an ultra-fast, memory-efficient command-line HTTP client built with **Rust**, **`mimalloc`**, **16-Thread Tokio Async Stream Engine**, and **Native C `libcurl` System Integration**.

---

## 🌟 Dual Hybrid Engine Architecture

1. **⚡ 16-Thread Tokio Parallel Streaming Engine (Default)**:
   - Range-chunk parallel streaming for high-speed multi-part downloads.
   - `mimalloc` allocator with < 3 MB RAM footprint.
   - Rate limiting, hash verification, live watch loops, and JSON telemetry.

2. **🌐 Native C `libcurl` System Engine Integration**:
   - Inherits 100% feature parity with C `libcurl` built over 27 years (including HSTS, Alt-Svc, RTSP, AWS-SigV4, GSS-API, LDAP, Kerberos, Telnet, Dict, SASL, SMTP, IMAP, POP3, SSH, SFTP, SCP, TFTP).

---

## 🛠️ Complete Command-Line Matrix

```bash
# 1. High-Concurrency 16-Thread Streaming Download
rcurl https://speed.hetzner.de/100MB.bin -o 100MB.bin -t 16

# 2. Live Endpoint Watching
rcurl https://httpbin.org/get -w 2s

# 3. File Modification Watcher
rcurl https://api.example.com/v1/update -X POST -d @payload.json --watch-file payload.json

# 4. JSON Telemetry Output
rcurl https://postman-echo.com/get --json-output

# 5. Native Curl Compatibility Options
rcurl https://example.com --compressed -k --http2 -x socks5://127.0.0.1:9050
```

---

## 🔗 GitHub Repository
[**github.com/sachin-razz/rcurl**](https://github.com/sachin-razz/rcurl)
