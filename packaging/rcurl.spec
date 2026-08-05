Name:           rcurl
Version:        11.7.0
Release:        1%{?dist}
Summary:        16-Thread Tokio Streaming CLI Downloader

License:        MIT
URL:            https://github.com/sachin-razz/rcurl
Source0:        https://github.com/sachin-razz/rcurl/archive/v%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust

%description
Ultra-fast 16-thread streaming CLI downloader with BitTorrent P2P,
WebDrive, gRPC, Zstd Dict, eBPF XDP, and TUI Dashboard.

%prep
%autosetup -n rcurl-%{version}

%build
cargo build --release
cargo run --bin gen_completions

%install
rm -rf $RPM_BUILD_ROOT
install -d %{buildroot}%{_bindir}
install -m 0755 target/release/rcurl %{buildroot}%{_bindir}/rcurl
install -d %{buildroot}%{_datadir}/zsh/site-functions
install -m 0644 completions/_rcurl %{buildroot}%{_datadir}/zsh/site-functions/_rcurl

%files
%{_bindir}/rcurl
%{_datadir}/zsh/site-functions/_rcurl

%changelog
* Wed Aug 05 2026 Sachin Rajpurohit <sachin-razz> - 11.7.0-1
- Initial RPM release
