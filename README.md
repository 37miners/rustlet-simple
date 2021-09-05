# rustlet-simple

The goal of this project is to make the most simple rustlet possible. There are two files in the project: [Cargo.toml](https://github.com/bitcoinmw/rustlet-simple/blob/master/Cargo.toml) and [main.rs](https://github.com/bitcoinmw/rustlet-simple/blob/master/src/main.rs). Cargo.toml only has a single dependency, which is the rustlet project. The rustlet project can be [found here](https://github.com/bitcoinmw/rustlet).

# Prerequisites

1.) Git - https://git-scm.com/book/en/v2/Getting-Started-Installing-Git

2.) Rust - Most testing was done on Rust 1.54.0. The latest stable version of rust is suggested. To get rust go to https://rustup.rs

# Building/Running

To build/run, run the following commands from the terminal.

```
# git clone https://github.com/bitcoinmw/rustlet-simple

# cd rustlet-simple

# cargo run
```

After that, you can access the "Hello World" rustlet in your browser at http://localhost:8080/.
Please note that using cargo run compiles in debug mode, so to get _MUCH_ better performance, use 'cargo build --release', then ./target/release/rustlet.

