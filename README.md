# should-i

Ask the universe for guidance on your life decisions! 🎲

A simple CLI tool that helps you make decisions by consulting the [yesno.wtf](https://yesno.wtf) API. Get instant yes/no/maybe answers with fun GIF animations.

## Installation

### Using Cargo

```bash
cargo install should-i
```

### Using Homebrew

```bash
brew install should-i
```

### From Source

```bash
git clone https://github.com/yourusername/should-i
cd should-i
cargo install --path .
```

## Usage

Ask any yes/no question:

```bash
should-i "go to the gym"
should-i "eat pizza tonight"
should-i "勉強する"
```

### Example Output

```
🎲 Asking the universe...

✅ YES! Do it! 🎉

🖼️  https://yesno.wtf/assets/yes/2.gif
```

### Options

- `-o, --open`: Open the GIF image in your browser
- `-h, --help`: Show help information

```bash
should-i "take a break" --open
```

## Features

- 🎯 Simple and intuitive CLI interface
- 🌐 Powered by the yesno.wtf API
- 🎨 Colorful emoji-based output
- 🖼️ Optional browser integration to view GIFs
- 🌍 Supports any language for questions
- ⚡ Fast and lightweight

## How It Works

`should-i` sends a request to the [yesno.wtf API](https://yesno.wtf/api) and displays:
- The answer (Yes, No, or Maybe)
- A URL to an animated GIF representing the answer
- Optional: Opens the GIF directly in your browser with the `--open` flag

## Requirements

- Rust 1.81.0 or later (for building from source)

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
