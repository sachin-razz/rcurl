class Rcurl < Formula
  desc "16-Thread Tokio Streaming CLI Downloader (Pure Rust Native Engine)"
  homepage "https://github.com/sachin-razz/rcurl"
  url "https://github.com/sachin-razz/rcurl/archive/refs/tags/v11.7.0.tar.gz"
  sha256 "0000000000000000000000000000000000000000000000000000000000000000"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
    system "cargo", "run", "--bin", "gen_completions"

    zsh_completion.install "completions/_rcurl"
    bash_completion.install "completions/rcurl.bash"
    fish_completion.install "completions/rcurl.fish"
  end

  test do
    system "#{bin}/rcurl", "--version"
  end
end
