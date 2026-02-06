# You

Rust workspace project with CLI application and libraries.


## Getting Started

```bash
# Build the entire workspace
cargo build

# Run the CLI application
cargo run -p cli

# Run tests
cargo test

cargo new --lib your-lib-name
```

## Adding Libraries

When you add a new library , it will automatically be included in the workspace thanks to the wildcard pattern in the root `Cargo.toml`.

To use a library in the CLI or another library, add it to the dependencies:

```toml
[dependencies]
your-service = { path = "../your-lib-name" }
```
