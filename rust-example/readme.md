# Rust

https://www.rust-lang.org/tools/install  
https://doc.rust-lang.org/cargo/getting-started/installation.html

- Speed
- Safety
- Concurrency
- Potability

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
rustup --version
#rustup 1.28.2 (e4f3ad6f8 2025-04-28)
rustc --version
# rustc 1.89.0 (29483883e 2025-08-04)
cargo --version
# cargo 1.89.0 (c24e10642 2025-06-23)

cargo new rust-example
cd rust-example
# or
cargo init # 
# cargo build
cargo run
```

# Recommended Project Structure
```text
my-rust-app/
├── Cargo.toml
├── src/
│   ├── main.rs          # Entry point (for binaries)
│   ├── lib.rs           # For shared logic (if applicable)
│   ├── bin/             # Optional: additional binaries
│   │   └── helper.rs
│   ├── models/          # Data structures, domain models
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── services/        # Business logic, use cases
│   │   ├── mod.rs
│   │   └── auth.rs
│   ├── routes/          # Web routes (if web app)
│   │   ├── mod.rs
│   │   └── user_routes.rs
│   ├── handlers/        # Request handlers (web apps)
│   │   ├── mod.rs
│   │   └── user_handler.rs
│   ├── config/          # Configuration handling
│   │   ├── mod.rs
│   │   └── env.rs
│   ├── utils/           # Helper functions, macros
│   │   ├── mod.rs
│   │   └── logging.rs
│   └── db/              # Database access, ORM models, migrations
│       ├── mod.rs
│       └── schema.rs
├── tests/               # Integration and module tests
│   ├── integration_test.rs
│   └── unit_test.rs
├── examples/            # Example binaries
│   └── demo.rs
├── migrations/          # Database migrations (if using SQLx, Diesel, etc.)
├── .env                 # Environment variables (gitignored)
├── .gitignore
└── README.md
```
