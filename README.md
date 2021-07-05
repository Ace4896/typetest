# TypeTest

An offline typing test, written using Rust and Iced. Inspired by [typetest.io](https://typetest.io/) and [10fastfingers.com](https://10fastfingers.com/).

![Demo](docs/demo.gif)

## Requirements

On all OSes, Rust 1.53 is required.

On Linux, `iced` has additional dependencies that need to be present when installing from crates.io or from source - see [this GitHub issue](https://github.com/hecrj/iced/issues/256) for more information.

## Usage

### Installation

Install from crates.io:

```
cargo install typetest
```

Or alternatively, clone this repository and run from the source code:

```
git clone https://github.com/Ace4896/typetest.git
cd typetest
cargo run --release
```

### Shortcuts

- Redo Current Test: `Ctrl/Cmd + R` or `F5`
