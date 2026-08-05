_rcurl() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="rcurl"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        rcurl)
            opts="-g -a -q -4 -6 -P -l -p -R -D -z -o -O -X -H -d -F -T -k -E -N -L -v -i -I -s -S -f -t -Z -C -r -A -e -b -c -u -U -m -x -0 -K -n -w -h -V --input-file --background --execute --output-file --append-output --debug --quiet --no-verbose --force-html --base --bind-address --tries --no-clobber --continue --progress --no-use-server-timestamps --server-response --spider --wget-timeout --dns-timeout --read-timeout --wait --waitretry --random-wait --no-proxy --quota --no-dns-cache --restrict-file-names --inet4-only --inet6-only --prefer-family --retry-connrefused --user-name --password --ask-password --no-iri --local-encoding --remote-encoding --unlink --no-directories --force-directories --no-host-directories --protocol-directories --cut-dirs --directory-prefix --html-extension --http-user --http-passwd --no-cache --no-cookies --load-cookies --save-cookies --keep-session-cookies --ignore-length --max-redirect --proxy-user-wget --proxy-password --save-headers --post-data --post-file --content-disposition --trust-server-names --auth-no-challenge --secure-protocol --no-check-certificate --certificate --certificate-type --private-key --private-key-type --ca-certificate --ca-directory --recursive --level --delete-after --convert-links --backup-converted --mirror --page-requisites --strict-comments --accept --reject --domains --exclude-domains --follow-ftp --follow-tags --ignore-tags --ignore-case --archive --compress --delete --bwlimit --dry-run --whole-file --inplace --backup --backup-dir --suffix --checksum --itemize-changes --stats --delay-updates --partial --partial-dir --prune-empty-dirs --remove-source-files --chmod --chown --numeric-ids --list-only --mkpath --type --rsync-ssl --daemon --rsyncd-config --no-detach --dparam --rrsync --rrsync-dir --rrsync-ro --rrsync-wo --rrsync-munge --rrsync-no-del --rrsync-no-overwrite --path-containment --fastcdc --ultracdc --turboquant --mcts-router --subq --polarquant --gdrive-upload --resumable --max-days --max-downloads --encrypt-password --ultraheavy --no-ultraheavy --torrent --no-share --p2p-mesh --send --receive --tailscale-mesh --grpc --json-rpc --xml-rpc --zstd-dict --train-dict --ebpf-accelerator --tui --tor --i2p --multicast-send --multicast-listen --omni-multicast --mitm-proxy --micro-ram --transfer-server --adler-md5 --span-hosts --relative --include-directories --exclude-directories --no-parent --output --remote-name --remote-name-all --output-dir --request --header --proxy-header --data --data-raw --data-binary --data-urlencode --json --form --form-string --upload-file --compressed --insecure --cacert --cert --key --pass --dump-header --max-redirs --connect-timeout --no-buffer --location --location-trusted --verbose --include --head --silent --show-error --fail --threads --parallel --parallel-max --continue-at --range --user-agent --referer --cookie --cookie-jar --user --proxy-user --max-time --retry --rate-limit --limit-rate --sha256 --md5 --proxy --socks5 --socks5-hostname --json-output --json-metrics --http1.0 --http1.1 --http2 --http2-prior-knowledge --http3 --doh-url --resolve --config --netrc --netrc-file --watch --watch-file --write-out --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --input-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --execute)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -g)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --append-output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -a)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --base)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --bind-address)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --tries)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --progress)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wget-timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --dns-timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --read-timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --wait)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --waitretry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --quota)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --restrict-file-names)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --prefer-family)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --user-name)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --password)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --local-encoding)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --remote-encoding)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cut-dirs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --directory-prefix)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -P)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --http-user)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --http-passwd)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --load-cookies)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --save-cookies)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-redirect)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --proxy-user-wget)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --proxy-password)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --post-data)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --post-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --secure-protocol)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --certificate)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --certificate-type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --private-key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --private-key-type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --ca-certificate)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --ca-directory)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --level)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -l)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --accept)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --reject)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -R)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --domains)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -D)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --exclude-domains)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --follow-tags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --ignore-tags)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --bwlimit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --backup-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --suffix)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --partial-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --chmod)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --chown)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --type)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rsyncd-config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --dparam)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rrsync-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-days)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-downloads)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --encrypt-password)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --send)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --receive)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --json-rpc)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --xml-rpc)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --zstd-dict)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --train-dict)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --multicast-send)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --multicast-listen)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --include-directories)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --exclude-directories)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -o)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --output-dir)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --request)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -X)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --header)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -H)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --proxy-header)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --data)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -d)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --data-raw)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --data-binary)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --data-urlencode)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --json)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --form)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -F)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --form-string)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --upload-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -T)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cacert)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cert)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -E)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --key)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --pass)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --dump-header)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-redirs)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --connect-timeout)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --threads)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --parallel-max)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --continue-at)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --range)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --user-agent)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -A)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --referer)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -e)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cookie)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -b)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cookie-jar)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --user)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -u)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --proxy-user)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -U)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --max-time)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --retry)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --rate-limit)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --limit-rate)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --sha256)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --md5)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --proxy)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -x)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --socks5)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --socks5-hostname)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --doh-url)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --resolve)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --config)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -K)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --netrc-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --watch)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -w)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --watch-file)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --write-out)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _rcurl -o nosort -o bashdefault -o default rcurl
else
    complete -F _rcurl -o bashdefault -o default rcurl
fi
