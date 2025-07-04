# Project Guidelines

## Project Overview

This is a Rust-based Solana blockchain data processing system with a modular architecture for indexing and processing
Star Atlas game data.

## Project Structure

- **`/database`** - Database schema, migrations, and database-related utilities
- **`/decoder`** - Solana transaction and account data decoders
- **`/indexer`** - Solana blockchain data indexing service
- **`/processor`** - Solana Data processing logic
- **Root** - Workspace configuration and shared dependencies

## Development Guidelines

### Building and Testing

- Always run `cargo build` to verify compilation before submitting changes
- Run `cargo test` to execute all tests across the workspace
- Use `cargo clippy` to check for linting issues and code quality
- Format code with `cargo fmt` before committing

### Database Operations

- Database migrations are located in `/database/migrations/`

### Code Style

- Follow standard Rust formatting (rustfmt)
- Use meaningful variable and function names
- Add comprehensive error handling using `anyhow` or `thiserror`
- Include proper documentation for public APIs
- Use `async/await` patterns consistently with tokio runtime

### Solana Integration

- When working with Solana data, use the provided `solana-sdk` and related crates
- Ensure proper handling of blockchain data serialization/deserialization with `borsh`
- Use appropriate commitment levels for blockchain queries

### Environment and Configuration

- Use `.env` files for environment-specific configuration
- Leverage `dotenv` for loading environment variables
- Include proper logging with `log` and `env_logger` crates

### Testing Requirements

- Write unit tests for all business logic
- Include integration tests for database operations
- Test error handling paths
- Verify async operations work correctly

### Dependencies

- Stick to the established dependency versions unless upgrading is necessary
- New dependencies should be added to the workspace root `Cargo.toml`
- Consider using workspace features for optional functionality