# xgrep

Minimal grep. If you're looking for a fully featured grep, use [ripgrep](https://github.com/BurntSushi/ripgrep) instead.

## Installation

Install from source ([Rust](https://www.rust-lang.org/tools/install) installation required):

```bash
cargo install --git https://github.com/drewxs/xgrep
```

## Usage

```bash
xgrep <query> <file_path>
```

## Developing

```bash
git clone https://github.com/drewxs/xgrep
cd xgrep

# Running
cargo run <query> <file_path>

# Testing
cargo test

# Building
cargo build [--release]

# Installing locally
cargo install --path .
```
