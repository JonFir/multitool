# You

Rust workspace project with CLI application and libraries.

## Structure

```
you/
├── cli/              # CLI application
├── services/         # Service libraries
└── utilities/        # Utility libraries
```

## Getting Started

```bash
# Build the entire workspace
cargo build

# Run the CLI application
cargo run -p cli

# Run tests
cargo test

# Add a new service
cargo new --lib services/your-service-name

# Add a new utility
cargo new --lib utilities/your-util-name
```

## Adding Libraries

When you add a new library to `services/` or `utilities/`, it will automatically be included in the workspace thanks to the wildcard pattern in the root `Cargo.toml`.

To use a library in the CLI or another library, add it to the dependencies:

```toml
[dependencies]
your-service = { path = "../services/your-service-name" }
```
