# should-i

[![Crates.io](https://img.shields.io/crates/v/should-i.svg)](https://crates.io/crates/should-i)
[![CI](https://github.com/Justhiro55/should-i/workflows/CI/badge.svg)](https://github.com/Justhiro55/should-i/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.81%2B-orange.svg)](https://www.rust-lang.org/)

> A CLI tool to help you make decisions by asking the universe 🎲

Consult the [yesno.wtf](https://yesno.wtf) API for instant yes/no/maybe answers to life's toughest questions.

## Installation

### Using Cargo

```bash
cargo install should-i
```

### Using Homebrew

```bash
brew tap Justhiro55/tap
brew install should-i
```

### From source

```bash
git clone https://github.com/Justhiro55/should-i
cd should-i
cargo install --path .
```

## Usage

```bash
$ should-i "buy a new laptop"

🎲 Asking the universe...

✅ YES! Do it! 🎉

🖼️  https://yesno.wtf/assets/yes/2.gif
```

```bash
$ should-i "eat pizza tonight"

🎲 Asking the universe...

❌ NO! Don't do it! 🚫

🖼️  https://yesno.wtf/assets/no/0.gif
```

### Options

- `-o, --open`: Open the GIF image in your browser
- `-h, --help`: Show help

```bash
should-i "take a break" --open
```

## License

MIT OR Apache-2.0
