complete -c rcurl -l input-file -d 'Read URLs from a local input text file (Wget --input-file)' -r -F
complete -c rcurl -l execute -d 'Execute command as if part of .wgetrc (Wget --execute)' -r
complete -c rcurl -s g -l output-file -d 'Log messages to logfile (Wget -o / --output-file)' -r -F
complete -c rcurl -s a -l append-output -d 'Append messages to logfile (Wget -a / --append-output)' -r -F
complete -c rcurl -l base -d 'Resolve relative links using URL as base (Wget -B / --base)' -r
complete -c rcurl -l bind-address -d 'Bind to local TCP/IP address (Wget --bind-address)' -r
complete -c rcurl -l tries -d 'Set number of retries (Wget -t / --tries)' -r
complete -c rcurl -l progress -d 'Select progress indicator type (dot or bar) (Wget --progress)' -r
complete -c rcurl -l wget-timeout -d 'Network timeout in seconds (Wget --timeout)' -r
complete -c rcurl -l dns-timeout -d 'DNS lookup timeout in seconds (Wget --dns-timeout)' -r
complete -c rcurl -l read-timeout -d 'Read timeout in seconds (Wget --read-timeout)' -r
complete -c rcurl -l wait -d 'Wait N seconds between retrievals (Wget -w / --wait)' -r
complete -c rcurl -l waitretry -d 'Wait N seconds between retries (Wget --waitretry)' -r
complete -c rcurl -l quota -d 'Download quota limit (Wget -Q / --quota)' -r
complete -c rcurl -l restrict-file-names -d 'Restrict URL file name modes (unix, windows, ascii) (Wget --restrict-file-names)' -r
complete -c rcurl -l prefer-family -d 'Preferred IP family (IPv4, IPv6, none) (Wget --prefer-family)' -r
complete -c rcurl -l user-name -d 'Username for FTP/HTTP (Wget --user)' -r
complete -c rcurl -l password -d 'Password for FTP/HTTP (Wget --password)' -r
complete -c rcurl -l local-encoding -d 'Local locale encoding (Wget --local-encoding)' -r
complete -c rcurl -l remote-encoding -d 'Remote server encoding (Wget --remote-encoding)' -r
complete -c rcurl -l cut-dirs -d 'Ignore N remote directory components (Wget --cut-dirs)' -r
complete -c rcurl -s P -l directory-prefix -d 'Set output directory prefix (Wget -P / --directory-prefix)' -r -F
complete -c rcurl -l http-user -d 'HTTP username (Wget --http-user)' -r
complete -c rcurl -l http-passwd -d 'HTTP password (Wget --http-passwd)' -r
complete -c rcurl -l load-cookies -d 'Load Netscape cookies file (Wget --load-cookies)' -r -F
complete -c rcurl -l save-cookies -d 'Save cookies file before exit (Wget --save-cookies)' -r -F
complete -c rcurl -l max-redirect -d 'Max redirections (Wget --max-redirect)' -r
complete -c rcurl -l proxy-user-wget -d 'Proxy username (Wget --proxy-user)' -r
complete -c rcurl -l proxy-password -d 'Proxy password (Wget --proxy-password)' -r
complete -c rcurl -l post-data -d 'Send POST data string (Wget --post-data)' -r
complete -c rcurl -l post-file -d 'Send POST payload from file (Wget --post-file)' -r -F
complete -c rcurl -l secure-protocol -d 'Choose SSL/TLS protocol (auto, SSLv2, SSLv3, TLSv1) (Wget --secure-protocol)' -r
complete -c rcurl -l certificate -d 'Client certificate file (Wget --certificate)' -r -F
complete -c rcurl -l certificate-type -d 'Certificate type (PEM, DER) (Wget --certificate-type)' -r
complete -c rcurl -l private-key -d 'Private key file (Wget --private-key)' -r -F
complete -c rcurl -l private-key-type -d 'Private key type (PEM, DER) (Wget --private-key-type)' -r
complete -c rcurl -l ca-certificate -d 'Bundle CA certificate (Wget --ca-certificate)' -r -F
complete -c rcurl -l ca-directory -d 'Directory of CA certificates (Wget --ca-directory)' -r -F
complete -c rcurl -s l -l level -d 'Maximum recursion depth level for web crawling (Wget -l / --level)' -r
complete -c rcurl -l accept -d 'Comma-separated list of accepted file extensions (Wget --accept)' -r
complete -c rcurl -s R -l reject -d 'Comma-separated list of rejected file extensions (Wget -R / --reject)' -r
complete -c rcurl -s D -l domains -d 'Comma-separated list of accepted domains (Wget -D / --domains)' -r
complete -c rcurl -l exclude-domains -d 'Comma-separated list of excluded domains (Wget --exclude-domains)' -r
complete -c rcurl -l follow-tags -d 'Follow HTML tags list (Wget --follow-tags)' -r
complete -c rcurl -l ignore-tags -d 'Ignore HTML tags list (Wget --ignore-tags)' -r
complete -c rcurl -l bwlimit -d 'Rsync Bandwidth limit in KB/s (Rsync --bwlimit)' -r
complete -c rcurl -l backup-dir -d 'Backup directory for destination files (Rsync --backup-dir)' -r
complete -c rcurl -l suffix -d 'Backup file suffix (Rsync --suffix)' -r
complete -c rcurl -l partial-dir -d 'Directory for partially transferred files (Rsync --partial-dir)' -r
complete -c rcurl -l chmod -d 'Custom permissions chmod mode (Rsync --chmod)' -r
complete -c rcurl -l chown -d 'Force ownership user:group mapping (Rsync --chown)' -r
complete -c rcurl -l type -d 'Rsync SSL helper connection type: openssl, stunnel, gnutls (Rsync-ssl --type)' -r
complete -c rcurl -l rsyncd-config -d 'Path to rsyncd.conf daemon configuration file (Rsync --rsyncd-config)' -r
complete -c rcurl -l dparam -d 'Override global daemon config parameter (Rsync -M / --dparam)' -r
complete -c rcurl -l rrsync-dir -d 'Restricted root directory path for rrsync (rrsync DIR)' -r
complete -c rcurl -l max-days -d 'Maximum days to store file on transfer server (transfer.sh Max-Days)' -r
complete -c rcurl -l max-downloads -d 'Maximum download count before automatic file deletion (transfer.sh Max-Downloads)' -r
complete -c rcurl -l encrypt-password -d 'Encryption password for transfer payload (--encrypt-password)' -r
complete -c rcurl -l send -d 'Send file to peer over P2P mesh' -r
complete -c rcurl -l receive -d 'Receive file from peer using pairing PIN code' -r
complete -c rcurl -l json-rpc -d 'Transmit JSON-RPC 2.0 payload' -r
complete -c rcurl -l xml-rpc -d 'Transmit XML-RPC payload' -r
complete -c rcurl -l zstd-dict -d 'Load pre-trained Zstandard Shared Dictionary file' -r
complete -c rcurl -l train-dict -d 'Train 32 KB Zstandard Shared Dictionary on sample payload directory' -r
complete -c rcurl -l multicast-send -d 'Broadcast file via Omni-Multicast stream' -r
complete -c rcurl -l multicast-listen -d 'Listen and receive Omni-Multicast broadcast stream' -r
complete -c rcurl -l include-directories -d 'Comma-separated list of included directories (Wget -I / --include-directories)' -r
complete -c rcurl -l exclude-directories -d 'Comma-separated list of excluded directories (Wget --exclude-directories)' -r
complete -c rcurl -s o -l output -d 'Write response output to target file instead of stdout' -r -F
complete -c rcurl -l output-dir -d 'Directory to save output files' -r -F
complete -c rcurl -s X -l request -d 'Custom HTTP request method (GET, POST, PUT, DELETE, PATCH, HEAD)' -r
complete -c rcurl -s H -l header -d 'Pass custom header(s) to server (e.g. -H "Content-Type: application/json")' -r
complete -c rcurl -l proxy-header -d 'Pass custom header to proxy' -r
complete -c rcurl -s d -l data -d 'HTTP POST / PUT data payload' -r
complete -c rcurl -l data-raw -d 'HTTP POST raw data payload' -r
complete -c rcurl -l data-binary -d 'HTTP POST binary data payload' -r
complete -c rcurl -l data-urlencode -d 'HTTP POST URL-encoded data payload' -r
complete -c rcurl -l json -d 'Send JSON payload and automatically set Content-Type & Accept to application/json' -r
complete -c rcurl -s F -l form -d 'Send multipart/form-data form fields (e.g. -F "file=@photo.jpg")' -r
complete -c rcurl -l form-string -d 'Send multipart/form-data string field' -r
complete -c rcurl -s T -l upload-file -d 'Transfer local file to remote server via PUT (-T file.tar.gz)' -r -F
complete -c rcurl -l cacert -d 'Specify CA certificate file to verify peer' -r -F
complete -c rcurl -s E -l cert -d 'Specify client certificate file' -r -F
complete -c rcurl -l key -d 'Specify private key file' -r -F
complete -c rcurl -l pass -d 'Certificate key passphrase' -r
complete -c rcurl -l dump-header -d 'Dump raw response headers to a separate file' -r -F
complete -c rcurl -l max-redirs -d 'Maximum number of redirects to follow (default: 50)' -r
complete -c rcurl -l connect-timeout -d 'Maximum time allowed for connection phase in seconds' -r
complete -c rcurl -s t -l threads -d 'Number of parallel worker threads / range chunk streams (default: 16)' -r
complete -c rcurl -l parallel-max -d 'Maximum parallel transfers' -r
complete -c rcurl -s C -l continue-at -d 'Resume transfer from byte offset or automatically detect existing file size ("auto")' -r
complete -c rcurl -s r -l range -d 'Request byte range (e.g. -r 0-1024)' -r
complete -c rcurl -s A -l user-agent -d 'Specify User-Agent header string' -r
complete -c rcurl -s e -l referer -d 'Specify HTTP Referer header' -r
complete -c rcurl -s b -l cookie -d 'Pass HTTP Cookie string (e.g. -b "name=val")' -r
complete -c rcurl -s c -l cookie-jar -d 'Save HTTP cookies to cookie jar file' -r -F
complete -c rcurl -s u -l user -d 'Set HTTP Basic authentication username:password' -r
complete -c rcurl -s U -l proxy-user -d 'Set proxy authentication username:password' -r
complete -c rcurl -s m -l max-time -d 'Maximum request timeout in seconds' -r
complete -c rcurl -l retry -d 'Maximum number of automatic retries on connection failure' -r
complete -c rcurl -l rate-limit -d 'Restrict maximum download speed (e.g. --rate-limit 5M, 500K)' -r
complete -c rcurl -l limit-rate -d 'Restrict maximum download speed (curl standard flag)' -r
complete -c rcurl -l sha256 -d 'Expected SHA-256 hash to verify file integrity after streaming download' -r
complete -c rcurl -l md5 -d 'Expected MD5 hash to verify file integrity after streaming download' -r
complete -c rcurl -s x -l proxy -d 'Route requests through HTTP, HTTPS, or SOCKS5 proxy (e.g. -x socks5://127.0.0.1:9050)' -r
complete -c rcurl -l socks5 -d 'SOCKS5 proxy server' -r
complete -c rcurl -l socks5-hostname -d 'SOCKS5 hostname proxy server' -r
complete -c rcurl -l doh-url -d 'DNS Over HTTPS (DoH) URL' -r
complete -c rcurl -l resolve -d 'Custom host resolution mapping (--resolve host:port:address)' -r
complete -c rcurl -s K -l config -d 'Custom path for .rcurlrc configuration file' -r -F
complete -c rcurl -l netrc-file -d 'Netrc file path' -r -F
complete -c rcurl -s w -l watch -d 'Periodically poll/watch URL every N seconds (e.g. -w 2s, -w 500ms)' -r
complete -c rcurl -l watch-file -d 'Automatically re-send request whenever specified file changes on disk' -r -F
complete -c rcurl -l write-out -d 'Output format string (curl -w / --write-out)' -r
complete -c rcurl -l background -d 'Run in background immediately (Wget --background)'
complete -c rcurl -l debug -d 'Turn on debug output (Wget -d / --debug)'
complete -c rcurl -s q -l quiet -d 'Turn off output / quiet mode (Wget -q / --quiet)'
complete -c rcurl -l no-verbose -d 'Non-verbose output (Wget -nv / --no-verbose)'
complete -c rcurl -l force-html -d 'Force input file to be treated as HTML (Wget -F / --force-html)'
complete -c rcurl -l no-clobber -d 'Prevent clobbering existing files (Wget -nc / --no-clobber)'
complete -c rcurl -l continue -d 'Continue getting partially-downloaded file (Wget -c / --continue)'
complete -c rcurl -l no-use-server-timestamps -d 'Don\'t set local file timestamp from server (Wget --no-use-server-timestamps)'
complete -c rcurl -l server-response -d 'Print headers sent by HTTP/FTP servers (Wget -S / --server-response)'
complete -c rcurl -l spider -d 'Web spider mode: check pages without downloading (Wget --spider)'
complete -c rcurl -l random-wait -d 'Randomize wait times between requests (Wget --random-wait)'
complete -c rcurl -l no-proxy -d 'Don\'t use proxies (Wget --no-proxy)'
complete -c rcurl -l no-dns-cache -d 'Turn off DNS caching (Wget --no-dns-cache)'
complete -c rcurl -s 4 -l inet4-only -d 'Force IPv4 only (Wget -4 / --inet4-only)'
complete -c rcurl -s 6 -l inet6-only -d 'Force IPv6 only (Wget -6 / --inet6-only)'
complete -c rcurl -l retry-connrefused -d 'Retry transient connection refused errors (Wget --retry-connrefused)'
complete -c rcurl -l ask-password -d 'Prompt for password (Wget --ask-password)'
complete -c rcurl -l no-iri -d 'Turn off IRI support (Wget --no-iri)'
complete -c rcurl -l unlink -d 'Force unlink existing file before writing (Wget --unlink)'
complete -c rcurl -l no-directories -d 'Disable directory creation hierarchy (Wget -nd / --no-directories)'
complete -c rcurl -l force-directories -d 'Force local directory hierarchy creation (Wget -x / --force-directories)'
complete -c rcurl -l no-host-directories -d 'Disable host-prefixed directory (Wget -nH / --no-host-directories)'
complete -c rcurl -l protocol-directories -d 'Use protocol names as directory components (Wget --protocol-directories)'
complete -c rcurl -l html-extension -d 'Append .html extension to HTML responses (Wget -E / --html-extension)'
complete -c rcurl -l no-cache -d 'Disable server-side caching (Wget --no-cache)'
complete -c rcurl -l no-cookies -d 'Disable cookies (Wget --no-cookies)'
complete -c rcurl -l keep-session-cookies -d 'Save session cookies (Wget --keep-session-cookies)'
complete -c rcurl -l ignore-length -d 'Ignore Content-Length header (Wget --ignore-length)'
complete -c rcurl -l save-headers -d 'Save response headers to output file (Wget --save-headers)'
complete -c rcurl -l content-disposition -d 'Respect Content-Disposition header filename (Wget --content-disposition)'
complete -c rcurl -l trust-server-names -d 'Use redirection URL last component as filename (Wget --trust-server-names)'
complete -c rcurl -l auth-no-challenge -d 'Send Basic Auth without server challenge (Wget --auth-no-challenge)'
complete -c rcurl -l no-check-certificate -d 'Skip SSL certificate verification (Wget --no-check-certificate)'
complete -c rcurl -l recursive -d 'Enable recursive web crawling & downloading (Wget --recursive)'
complete -c rcurl -l delete-after -d 'Delete downloaded files after retrieval (Wget --delete-after)'
complete -c rcurl -l convert-links -d 'Convert links for local offline viewing (Wget -k / --convert-links)'
complete -c rcurl -l backup-converted -d 'Backup original files with .orig suffix before converting (Wget -K / --backup-converted)'
complete -c rcurl -l mirror -d 'Mirror website recursively with timestamping (Wget --mirror)'
complete -c rcurl -s p -l page-requisites -d 'Download all page requisites (CSS, JS, images) for offline viewing (Wget -p)'
complete -c rcurl -l strict-comments -d 'Strict SGML HTML comment parsing (Wget --strict-comments)'
complete -c rcurl -l follow-ftp -d 'Follow FTP links from HTML pages (Wget --follow-ftp)'
complete -c rcurl -l ignore-case -d 'Ignore case when matching files (Wget --ignore-case)'
complete -c rcurl -l archive -d 'Rsync Archive mode (preserve permissions, times, symlinks) (Rsync --archive)'
complete -c rcurl -s z -l compress -d 'Rsync Compress file data during transfer (Rsync -z / --compress)'
complete -c rcurl -l delete -d 'Rsync Delete extraneous files from destination dir (Rsync --delete)'
complete -c rcurl -l dry-run -d 'Perform trial run without making changes (Rsync --dry-run)'
complete -c rcurl -l whole-file -d 'Copy files whole without delta transfer (Rsync --whole-file)'
complete -c rcurl -l inplace -d 'Update destination files in-place (Rsync --inplace)'
complete -c rcurl -l backup -d 'Make backup copies of destination files (Rsync --backup)'
complete -c rcurl -l checksum -d 'Force checksum comparison before transfer (Rsync --checksum)'
complete -c rcurl -l itemize-changes -d 'Itemize change summary for all updates (Rsync --itemize-changes)'
complete -c rcurl -l stats -d 'Output verbose transfer statistics (Rsync --stats)'
complete -c rcurl -l delay-updates -d 'Put updated files into place at end (Rsync --delay-updates)'
complete -c rcurl -l partial -d 'Keep partially transferred files (Rsync --partial)'
complete -c rcurl -l prune-empty-dirs -d 'Prune empty directory chains from file list (Rsync --prune-empty-dirs)'
complete -c rcurl -l remove-source-files -d 'Sender removes synchronized files (Rsync --remove-source-files)'
complete -c rcurl -l numeric-ids -d 'Transfer numeric UID/GID values (Rsync --numeric-ids)'
complete -c rcurl -l list-only -d 'List files instead of transferring (Rsync --list-only)'
complete -c rcurl -l mkpath -d 'Create missing path components of destination (Rsync --mkpath)'
complete -c rcurl -l rsync-ssl -d 'Enable SSL/TLS encryption for Rsync daemon transfer (Rsync-ssl)'
complete -c rcurl -l daemon -d 'Run as an rsync daemon (Rsync --daemon)'
complete -c rcurl -l no-detach -d 'Do not detach from the parent process (Rsync --no-detach)'
complete -c rcurl -l rrsync -d 'Enable restricted rsync SSH mode (rrsync)'
complete -c rcurl -l rrsync-ro -d 'Allow only reading from restricted DIR (rrsync -ro)'
complete -c rcurl -l rrsync-wo -d 'Allow only writing to restricted DIR (rrsync -wo)'
complete -c rcurl -l rrsync-munge -d 'Enable symlink munging on server side (rrsync -munge)'
complete -c rcurl -l rrsync-no-del -d 'Disable delete and remove options in rrsync (rrsync -no-del)'
complete -c rcurl -l rrsync-no-overwrite -d 'Prevent overwriting existing files in rrsync (rrsync -no-overwrite)'
complete -c rcurl -l path-containment -d 'Enforce strict path traversal containment check within target directory'
complete -c rcurl -l fastcdc -d 'Enable FastCDC Content-Defined Variable Chunking Engine (Next-Gen Delta Sync)'
complete -c rcurl -l ultracdc -d 'Enable UltraCDC Normalized Dual-Mask & Merkle-DAG Tree Sync Engine'
complete -c rcurl -l turboquant -d 'Enable TurboQuant Vector Quantization Chunk Compression Engine'
complete -c rcurl -l mcts-router -d 'Enable MCTS (Monte Carlo Tree Search) Intelligent Multi-Path Chunk Router'
complete -c rcurl -l subq -d 'Enable SubQ (Sub-Vector Quantization) Delta Compression Engine'
complete -c rcurl -l polarquant -d 'Enable PolarQuant (Polar Coordinate Angle-Magnitude Quantization) Engine'
complete -c rcurl -l gdrive-upload -d 'Enable Google Drive Resumable API-keyless upload engine'
complete -c rcurl -l resumable -d 'Force Resumable Chunked Upload Protocol'
complete -c rcurl -l ultraheavy -d 'Combine all Next-Gen CDC & AI Compression Engines (UltraCDC + TurboQuant + MCTS + SubQ + PolarQuant)'
complete -c rcurl -l no-ultraheavy -d 'Disable Ultraheavy engine and force standard 16-thread HTTP Range streaming'
complete -c rcurl -l torrent -d 'Force BitTorrent P2P & Magnet Client Engine'
complete -c rcurl -l no-share -d 'Enable Private Leech Mode (Set upload rate to 0 and choke peers on 100% completion)'
complete -c rcurl -l p2p-mesh -d 'Enable Universal Open Device-to-Device Mesh Transfer Engine'
complete -c rcurl -l tailscale-mesh -d 'Enable Tailscale & WireGuard private mesh VPN compatibility'
complete -c rcurl -l grpc -d 'Force gRPC / gRPC-Web binary Protobuf streaming engine'
complete -c rcurl -l ebpf-accelerator -d 'Enable Kernel eBPF XDP Socket Acceleration (Linux zero-copy socket bypass)'
complete -c rcurl -l tui -d 'Launch Interactive Terminal TUI Dashboard'
complete -c rcurl -l tor -d 'Force Tor SOCKS5 Onion circuit routing for .onion URLs'
complete -c rcurl -l i2p -d 'Force I2P SAM v3 bridge routing for .i2p URLs'
complete -c rcurl -l omni-multicast -d 'Enable Omni-Multicast dual-stack IPv4/IPv6 SSM & PGM FEC repair engine'
complete -c rcurl -l mitm-proxy -d 'Launch TLS MITM Proxy Interceptor & Traffic Inspector Daemon'
complete -c rcurl -l micro-ram -d 'Force Sub-Megabyte (< 1 MB) Micro-RAM Engine Mode'
complete -c rcurl -l transfer-server -d 'Enable embedded Transfer.sh Server Daemon'
complete -c rcurl -l adler-md5 -d 'Force classic Adler-32 & MD5 Delta Engine algorithm'
complete -c rcurl -l span-hosts -d 'Span across hosts during recursive download (Wget -H / --span-hosts)'
complete -c rcurl -l relative -d 'Follow relative links only (Wget --relative)'
complete -c rcurl -l no-parent -d 'Do not ascend to parent directory (Wget -np / --no-parent)'
complete -c rcurl -s O -l remote-name -d 'Write output to a file named after the remote file name'
complete -c rcurl -l remote-name-all -d 'Write all remote files to remote names in multi-URL mode'
complete -c rcurl -l compressed -d 'Request compressed response (gzip, brotli, deflate, zstd)'
complete -c rcurl -s k -l insecure -d 'Allow insecure SSL/TLS connections (skip certificate verification)'
complete -c rcurl -s N -l no-buffer -d 'Disable stdout buffering for real-time streaming data'
complete -c rcurl -s L -l location -d 'Follow HTTP redirects'
complete -c rcurl -l location-trusted -d 'Follow redirects with trusted credentials across hosts'
complete -c rcurl -s v -l verbose -d 'Verbose output (show request and response headers)'
complete -c rcurl -s i -l include -d 'Include HTTP response headers in the output stream'
complete -c rcurl -s I -l head -d 'Fetch HTTP headers only (HEAD request)'
complete -c rcurl -s s -l silent -d 'Silent mode (suppress progress bar and status messages)'
complete -c rcurl -s S -l show-error -d 'Show error message even when silent'
complete -c rcurl -s f -l fail -d 'Fail fast on HTTP errors (4xx / 5xx)'
complete -c rcurl -s Z -l parallel -d 'Parallel multi-URL transfer mode'
complete -c rcurl -l json-output -d 'Output telemetry metrics and transfer results in JSON format'
complete -c rcurl -l json-metrics -d 'Legacy / shortcut json flag'
complete -c rcurl -s 0 -l http1.0 -d 'Force HTTP/1.0 protocol'
complete -c rcurl -l http1.1 -d 'Force HTTP/1.1 protocol'
complete -c rcurl -l http2 -d 'Prioritize HTTP/2 multiplexing protocol'
complete -c rcurl -l http2-prior-knowledge -d 'Force HTTP/2 prior knowledge'
complete -c rcurl -l http3 -d 'Prioritize HTTP/3 QUIC transport'
complete -c rcurl -s n -l netrc -d 'Read credentials from ~/.netrc file'
complete -c rcurl -s h -l help -d 'Print help'
complete -c rcurl -s V -l version -d 'Print version'
