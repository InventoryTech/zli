# zli 🎮

<a href="https://crates.io/crates/zli">
  <img alt="Crates.io" src="https://img.shields.io/crates/v/zli.svg">
</a>

zli is a ZMQ command line tool.

## Installation

```bash
cargo install zli
```

## Usage

```bash
zli sink "*" 5555
```

```bash
zli talk push localhost 5555 "what it do"
```