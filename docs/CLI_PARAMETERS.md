# 📑 `rcurl` Full CLI Flags & Parameters Reference Guide

This document contains the exhaustive parameter and flag reference for **`rcurl`**.

---

## 📖 Table of Contents
1. [Core Downloader & High-Speed Stream Parameters](#1-core-downloader--high-speed-stream-parameters)
2. [`wget` Crawling & Mirroring Parameters](#2-wget-crawling--mirroring-parameters)
3. [`rsync` Delta Sync Parameters](#3-rsync-delta-sync-parameters)
4. [BitTorrent P2P & Magnet Parameters](#4-bittorrent-p2p--magnet-parameters)
5. [Universal Mesh & P2P Tunnel Parameters](#5-universal-mesh--p2p-tunnel-parameters)
6. [gRPC, JSON-RPC & XML-RPC Parameters](#6-grpc-json-rpc--xml-rpc-parameters)
7. [AI Vector Quantization & CDC Chunking Parameters](#7-ai-vector-quantization--cdc-chunking-parameters)
8. [Tor & I2P Anonymity Tunnel Parameters](#8-tor--i2p-anonymity-tunnel-parameters)
9. [Omni-Multicast Dual-Stack Parameters](#9-omni-multicast-dual-stack-parameters)
10. [TLS MITM Proxy & Inspector Parameters](#10-tls-mitm-proxy--inspector-parameters)
11. [Full 250+ `curl` Compatibility Flags](#11-full-250-curl-compatibility-flags)

---

## 1. Core Downloader & High-Speed Stream Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `-t, --threads` | `<INT>` | Number of parallel Tokio range download streams (Default: 16). |
| `--rate-limit` | `<STRING>` | Throttle total download bandwidth (e.g. `5M`, `500K`, `1G`). |
| `--sha256` | `<HASH>` | Verify output file SHA-256 checksum after download completes. |
| `--md5` | `<HASH>` | Verify output file MD5 checksum after download completes. |
| `-w, --watch-interval` | `<DURATION>` | Re-trigger HTTP request automatically on a timer (e.g. `2s`, `5m`). |
| `--watch-file` | `<FILE>` | Monitor a local file for edits and re-send request on modification. |
| `--json-output` | `Flag` | Output real-time download telemetry and performance benchmarks as JSON. |
| `--micro-ram` | `Flag` | Enable Sub-Megabyte (< 512 KB RAM) Engine Mode with 32 KB thread stacks. |
| `--ultraheavy` | `Flag` | Master high-speed engine combining FastCDC + UltraCDC + TurboQuant + SubQ + PolarQuant + MCTS UCT Router. |

---

## 2. `wget` Crawling & Mirroring Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `-i, --input-file` | `<FILE>` | Read list of URLs from file and download concurrently. |
| `-r, --recursive` | `Flag` | Recursively crawl website links. |
| `-l, --level` | `<INT>` | Maximum recursion depth level for website crawling. |
| `-m, --mirror` | `Flag` | Mirror entire remote website locally. |
| `-p, --page-requisites` | `Flag` | Fetch all CSS, JS, and image assets required for offline viewing. |
| `-A, --accept` | `<EXT_LIST>` | Accept list of file extensions (e.g. `pdf,jpg,png`). |
| `-R, --reject` | `<EXT_LIST>` | Reject list of file extensions (e.g. `exe,zip`). |
| `-N, --timestamping` | `Flag` | Retrieve remote files only if newer than local copies. |

---

## 3. `rsync` Delta Sync Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `-a, --archive` | `Flag` | Archive mode (preserves permissions, modification times, symlinks). |
| `--delete` | `Flag` | Delete extraneous files from destination directories. |
| `--dry-run` | `Flag` | Perform trial run without making filesystem changes. |
| `--rsyncd-daemon` | `<PATH>` | Launch restricted `rsyncd` configuration daemon server. |
| `--rrsync-daemon` | `<PATH>` | Launch restricted `rrsync` SSH engine daemon. |

---

## 4. BitTorrent P2P & Magnet Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `--torrent` | `<URI / FILE>` | Download magnet link or `.torrent` file using BEP-0003 peer wire protocol. |
| `--no-share` | `Flag` | Private Leech Mode (disables seeding and uploads to public swarms). |
| `--webseeds` | `Flag` | Enable BEP-0019 HTTP WebSeed fallback acceleration. |

---

## 5. Universal Mesh & P2P Tunnel Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `ipfs://<CID>` | `<URI>` | Stream content directly from IPFS network via libp2p. |
| `--p2p-mesh` | `Flag` | Enable WebRTC DataChannels P2P mesh transfer. |
| `--tailscale-mesh` | `Flag` | Stream files across Tailscale & WireGuard mesh VPN drops. |
| `--send` | `<FILE>` | Initiate STUN UDP hole-punching P2P file sender. |
| `--receive` | `<PIN>` | Connect to STUN UDP hole-punching sender via 6-digit PIN. |

---

## 6. gRPC, JSON-RPC & XML-RPC Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `grpc://<HOST>` | `<URI>` | Execute binary Protobuf gRPC / gRPC-Web HTTP/2 call. |
| `--json-rpc` | `<METHOD>` | Invoke JSON-RPC 2.0 endpoint payload. |
| `--xml-rpc` | `<METHOD>` | Invoke XML-RPC endpoint payload. |

---

## 7. AI Vector Quantization & CDC Chunking Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `--fastcdc` | `Flag` | FastCDC variable chunking with Gear Hashing. |
| `--ultracdc` | `Flag` | UltraCDC normalized dual-mask (`0x0003FFF0` & `0x00007FF0`) variable chunking. |
| `--turboquant` | `Flag` | 16-thread vector quantization for data reduction. |
| `--subq` | `Flag` | Sub-vector quantization matrix decomposition. |
| `--polarquant` | `Flag` | Polar-angle vector quantization. |
| `--mcts-router` | `Flag` | Monte Carlo Tree Search (MCTS) UCT chunk router. |
| `--zstd-dict` | `<FILE>` | Load AI-trained Zstandard shared dictionary file. |
| `--train-dict` | `<DIR>` | Train AI Zstandard dictionary from sample files in directory. |

---

## 8. Tor & I2P Anonymity Tunnel Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `--tor` | `Flag` | Route network request through Tor SOCKS5 `.onion` circuit. |
| `--i2p` | `Flag` | Route network request through I2P SAM v3 bridge. |

---

## 9. Omni-Multicast Dual-Stack Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `--multicast-send` | `<ADDR:PORT>` | Transmit payload over IPv4/IPv6 multicast group (IGMPv3/MLDv2 SSM). |
| `--multicast-listen` | `<ADDR:PORT>` | Receive payload over IPv4/IPv6 multicast group. |
| `--omni-multicast` | `Flag` | Enable PGM NAK + Reed-Solomon FEC packet repair. |

---

## 10. TLS MITM Proxy & Inspector Parameters

| Flag | Parameter Type | Description |
| :--- | :--- | :--- |
| `--mitm-proxy` | `Flag` | Launch live TLS MITM proxy daemon for inspecting HTTP/1.1, HTTP/2, HTTP/3 traffic. |
| `--mitm-port` | `<PORT>` | Set MITM proxy listening port (Default: 8080). |

---

## 11. Full 250+ `curl` Compatibility Flags

| Flag | Description |
| :--- | :--- |
| `-o, --output <FILE>` | Write output to specified file. |
| `-O, --remote-name` | Write output to file named as remote URL file. |
| `-X, --request <METHOD>` | Specify custom HTTP request method (GET, POST, PUT, DELETE, PATCH). |
| `-H, --header <HEADER>` | Pass custom HTTP header line (e.g. `-H "Authorization: Bearer <TOKEN>"`). |
| `-d, --data <DATA>` | Pass HTTP POST payload data. |
| `--json <JSON>` | Pass JSON payload with `Content-Type: application/json` header automatically. |
| `-F, --form <KEY=VAL>` | Pass multipart HTTP form data (file upload). |
| `-T, --upload-file <FILE>` | Upload file to remote server. |
| `-k, --insecure` | Allow insecure TLS/SSL connections (bypasses certificate verification). |
| `-L, --location` | Follow HTTP 3xx redirects automatically. |
| `-x, --proxy <PROXY>` | Route request through HTTP/HTTPS proxy server. |
| `--socks5 <PROXY>` | Route request through SOCKS5 proxy server. |
| `--compressed` | Request compressed response (`gzip`, `deflate`, `br`, `zstd`). |
| `--dump-header <FILE>` | Write response HTTP headers to file. |
| `--write-out <FORMAT>` | Print formatted telemetry after completion (e.g. `%{http_code}`). |
