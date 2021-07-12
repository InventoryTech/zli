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
zli sink sub localhost 5555
zli sink rep "*" 5555
zli sink pull "*" 5555
```

```bash
zli talk pub "*" 5555 "what it do"
zli talk req localhost 5555 "what it do"
zli talk push localhost 5555 "what it do"
```