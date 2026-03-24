class Dkit < Formula
  desc "Swiss army knife for data format conversion and querying"
  homepage "https://github.com/syangkkim/dkit"
  license "MIT"
  version "0.9.0"

  on_macos do
    on_intel do
      url "https://github.com/syangkkim/dkit/releases/download/v#{version}/dkit-v#{version}-x86_64-apple-darwin.tar.gz"
      # sha256 will be updated on release
    end

    on_arm do
      url "https://github.com/syangkkim/dkit/releases/download/v#{version}/dkit-v#{version}-aarch64-apple-darwin.tar.gz"
      # sha256 will be updated on release
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/syangkkim/dkit/releases/download/v#{version}/dkit-v#{version}-x86_64-unknown-linux-musl.tar.gz"
      # sha256 will be updated on release
    end
  end

  def install
    bin.install "dkit"
  end

  test do
    assert_match "dkit", shell_output("#{bin}/dkit --version")
  end
end
