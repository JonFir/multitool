# Project Overview

## About
This is a CLI utility (with potential future HTTP interface) designed to handle various work-related tasks. The tool provides unified access to:

- Task trackers
- Code repositories
- Personal knowledge base
- Corporate wiki
- And other work management systems

## Task Complexity Levels

### Simple Tasks (Utility Functions)
- Retrieve a specific task
- Create a wiki page
- Fetch repository information
- Query knowledge base entries

### Complex Tasks (Intelligent Workflows)
- Get task list and generate daily work plan
- Prepare meeting agenda from calendar events
- Cross-reference tasks with code changes
- Generate status reports from multiple sources

## Architecture

### Domain-Driven Design
The project follows a domain-driven architecture with clear separation of concerns:

- **Domains are organized as libraries (`lib`)**: Each domain represents a specific business capability
- **Modular structure**: Domains can be composed to build complex workflows
- **Clear boundaries**: Each domain has its own models, services, and interfaces

Example domains might include:
- `tasks`: Task tracker integration
- `git`: Repository operations
- `wiki`: Wiki management
- `calendar`: Calendar and scheduling
- `knowledge`: Personal knowledge base

---

# Development Guidelines

## Code Quality Requirements

### Testing
**CRITICAL**: After completing any task, you MUST:
1. Run the test suite: `cargo test`
2. Ensure all tests pass
3. If tests fail, investigate and fix the issues before considering the task complete
4. For new features, write appropriate unit tests and integration tests

### Linting
**CRITICAL**: After completing any task, you MUST:
1. Run the linter: `cargo clippy -- -D warnings`
2. Fix all linter errors and warnings
3. Run formatter: `cargo fmt`
4. Ensure code adheres to Rust best practices

## Tracing and Observability

### Tracing Requirements
**MANDATORY**: All code must include proper tracing instrumentation.

#### Guidelines:
- Use the `tracing` crate for all logging and instrumentation
- Instrument functions with `#[tracing::instrument]` attribute where appropriate
- Use appropriate log levels:
  - `error!`: Critical failures
  - `warn!`: Recoverable issues or deprecated usage
  - `info!`: Significant events (startup, shutdown, major operations)
  - `debug!`: Detailed operational information
  - `trace!`: Very detailed diagnostic information

#### Example:
```rust
use tracing::{info, debug, instrument};

#[instrument(skip(client))]
async fn fetch_tasks(client: &TaskClient, filter: &TaskFilter) -> Result<Vec<Task>> {
    info!("Fetching tasks with filter: {:?}", filter);

    let tasks = client.get_tasks(filter).await?;
    debug!("Retrieved {} tasks", tasks.len());

    Ok(tasks)
}
```

#### Span Context:
- Create spans for logical operations
- Include relevant context (user_id, task_id, etc.)
- Use structured fields for better query capabilities

---

## Development Workflow

### Before Implementation
1. Review existing domain structure
2. Identify which domain(s) the feature belongs to
3. Check for existing patterns and utilities
4. Consider reusability and composability

### During Implementation
1. Write code with tracing instrumentation
2. Follow domain boundaries
3. Keep functions focused and testable
4. Document public APIs

### After Implementation
1. **Run tests**: `cargo test`
2. **Fix any test failures**
3. **Run linter**: `cargo clippy -- -D warnings`
4. **Fix all warnings**
5. **Format code**: `cargo fmt`
6. **Verify tracing output** (if applicable)

### Task Completion Checklist
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Code is formatted
- [ ] Tracing is properly instrumented
- [ ] Documentation is updated (if needed)

---

## Best Practices

### Error Handling
- Use `anyhow` for error handling
- Provide context with errors using `.context()` or `.with_context()`
- Log errors at appropriate levels

### Async Code
- Use `tokio` runtime for async operations
- Instrument async functions properly
- Handle cancellation gracefully

### Configuration
- Keep configuration separate from logic
- Use environment variables or config files
- Validate configuration on startup

### Dependencies
- Prefer well-maintained crates
- Keep dependencies minimal
- Document why specific crates are chosen

---

## Common Commands

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run linter
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting
cargo fmt -- --check

# Build release
cargo build --release

# Run the CLI
cargo run -- [args]

# Run with tracing enabled
RUST_LOG=debug cargo run -- [args]
```

---

## Notes for AI Assistant (Claude)

- Always run tests after making changes
- Always check and fix linter warnings
- Ensure tracing is properly instrumented in all new code
- Follow the domain-driven architecture
- Don't create new files unless absolutely necessary
- Keep code simple and focused - avoid over-engineering
- When in doubt, ask questions before implementing
