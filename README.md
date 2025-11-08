# should-i

Ask the universe for guidance on your decisions! 🎲

A simple CLI tool that consults the [yesno.wtf](https://yesno.wtf) API to help you make yes/no decisions.

## Installation

```bash
cargo install should-i
```

Or from source:

```bash
git clone https://github.com/yourusername/should-i
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
