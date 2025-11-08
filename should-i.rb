class ShouldI < Formula
  desc "CLI tool to help you make decisions by consulting the yesno.wtf API"
  homepage "https://github.com/Justhiro55/should-i"
  url "https://github.com/Justhiro55/should-i/archive/refs/tags/v0.1.2.tar.gz"
  sha256 "b67868c36b54592287c7cfbcfa08b44b2cff12560043805638394d3a7ff36900"
  license "MIT OR Apache-2.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    output = shell_output("#{bin}/should-i --help")
    assert_match "Ask the universe for guidance", output
  end
end
