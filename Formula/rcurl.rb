class Rcurl < Formula
  desc "16-Thread Tokio Streaming CLI Downloader (Pure Rust Native Engine)"
  homepage "https://github.com/sachin-razz/rcurl"
  url "https://github.com/sachin-razz/rcurl.git", branch: "master"
  version "1.0.0"
  license "MIT"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    system "#{bin}/rcurl", "--version"
  end
end
