# Rust

https://www.rust-lang.org/tools/install  
https://doc.rust-lang.org/cargo/getting-started/installation.html

## Install Rust and Cargo

```shell
# On Linux and macOS systems, this is done as follows:
curl https://sh.rustup.rs -sSf | sh
# Rust is installed now. Great!

# To configure your current shell, you need to source the corresponding env file under $HOME/.cargo.
# This is usually done by running one of the following (note the leading DOT):
. "$HOME/.cargo/env"            # For sh/bash/zsh/ash/dash/pdksh
source "$HOME/.cargo/env.fish"  # For fish
source $"($nu.home-path)/.cargo/env.nu"  # For nushell

# ---------
rustc --version
# rustc 1.89.0 (29483883e 2025-08-04)
cargo --version
# cargo 1.89.0 (c24e10642 2025-06-23)

cargo new rust-example
cd rust-example
#cargo build
cargo run
```
