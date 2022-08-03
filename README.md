# zli 🎮

<a href="https://crates.io/crates/zli">
  <img alt="Crates.io" src="https://img.shields.io/crates/v/zli.svg">
</a>

This is a fork of David Richard Holtz's zmq command line tool. It was forked to add extra options.

## Installation

Clone the repo and build using cargo `cargo build`

## Usage

`zli talk <ACTION> <HOST> <PORT> <TEXT> [TOPIC]`

`<ACTION>` for `talk`:

* `push`
* `req`
* `pub`

`<TEXT>` can be a file, contents of the file will be used
`[TOPIC]` is an optional topic for the `pub` action.

`zli sink <ACTION> <HOST> <PORT> [TOPIC]`

`<ACTION>` for `sink`:

* `pull`
* `rep`
* `sub`

`[TOPIC]` is an optional topic for the `sub` action.

### Examples

```bash
zli sink sub localhost 5555 "temperature"
zli sink rep "*" 5555
zli sink pull "*" 5555
```

```bash
zli talk pub "*" 5555 "25" "temperature"
zli talk req localhost 5555 "what it does"
zli talk push localhost 5555 "what it does"
```
