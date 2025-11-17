# Contributing to dioxus_style

Thank you for your interest in contributing to dioxus_style! This document provides guidelines and instructions for contributing.

## Code of Conduct

We are committed to providing a welcoming and inspiring community for all. Please be respectful and constructive in all interactions.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue on GitHub with:

- A clear, descriptive title
- Steps to reproduce the issue
- Expected behavior
- Actual behavior
- Your environment (OS, Rust version, Dioxus version)
- Code samples if applicable

### Suggesting Features

We welcome feature suggestions! Please create an issue with:

- A clear description of the feature
- Use cases and examples
- Why this would be beneficial to users
- Potential implementation approach (optional)

### Pull Requests

1. **Fork the repository** and create a new branch from `main`
2. **Make your changes** with clear, descriptive commit messages
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Ensure all tests pass**: `cargo test --all-features`
6. **Check formatting**: `cargo fmt --all -- --check`
7. **Run clippy**: `cargo clippy --all-targets --all-features -- -D warnings`
8. **Submit your pull request**

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Cargo

### Building the Project

```bash
# Clone the repository
git clone https://github.com/yourusername/dioxus_style.git
cd dioxus_style

# Build all workspace members
cargo build --all

# Run tests
cargo test --all

# Run specific workspace tests
cargo test -p dioxus_style
cargo test -p dioxus_style_macro
```

### Project Structure

```
dioxus_style/
├── dioxus_style/          # Runtime library
│   ├── src/
│   │   ├── lib.rs         # Public API exports
│   │   └── runtime_injector.rs  # Style registry
│   └── Cargo.toml
├── dioxus_style_macro/    # Procedural macros
│   ├── src/
│   │   ├── lib.rs         # Macro entry points
│   │   ├── macros.rs      # Macro implementations
│   │   ├── hash.rs        # Hash generation
│   │   └── style_parser.rs  # CSS parsing/scoping
│   └── Cargo.toml
└── Cargo.toml             # Workspace root
```

## Coding Guidelines

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for consistent formatting
- Address all `clippy` warnings
- Write idiomatic Rust code

### Documentation

- Add doc comments (`///`) for all public items
- Include examples in doc comments when helpful
- Use `#[doc(hidden)]` for internal APIs
- Keep documentation up-to-date with code changes

### Testing

- Write unit tests for new functionality
- Add integration tests for macro behavior
- Test edge cases and error conditions
- Maintain existing test coverage

### Performance

- Profile changes that affect performance
- Use benchmarks for performance-critical code
- Prefer zero-cost abstractions
- Document performance characteristics

## Testing Guidelines

### Running Tests

```bash
# All tests
cargo test --all

# Specific package
cargo test -p dioxus_style_macro

# Specific test
cargo test test_generate_hash

# With output
cargo test -- --nocapture
```

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Arrange
        let input = "test";
        
        // Act
        let result = function(input);
        
        // Assert
        assert_eq!(result, expected);
    }
}
```

## Macro Development

### Testing Procedural Macros

```bash
# Expand macros to see output
cargo expand --package dioxus_style_macro

# Test with trybuild for compile-time errors
# (requires trybuild in dev-dependencies)
```

### Debugging Tips

- Use `eprintln!` for debug output during macro expansion
- Check `target/` for intermediate files
- Use `quote!` carefully with proper escaping
- Test both success and error cases

## Documentation

### Building Docs

```bash
# Build documentation
cargo doc --no-deps --all-features

# Build and open in browser
cargo doc --no-deps --all-features --open

# Check for broken links
cargo doc --no-deps --all-features 2>&1 | grep warning
```

### Documentation Standards

- Complete, accurate descriptions
- Working code examples
- Clear parameter documentation
- Return value documentation
- Error condition documentation

## Release Process

(For maintainers)

1. Update version numbers in all `Cargo.toml` files
2. Update `CHANGELOG.md`
3. Update documentation
4. Run full test suite
5. Create git tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
6. Push tag: `git push origin v0.1.0`
7. Publish to crates.io:
   ```bash
   cd dioxus_style_macro && cargo publish
   cd ../dioxus_style && cargo publish
   ```

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Discord**: Join the Dioxus community server

## License

By contributing to dioxus_style, you agree that your contributions will be licensed under both the MIT License and Apache License 2.0.

## Recognition

Contributors will be acknowledged in:
- GitHub contributors list
- Release notes
- Project documentation

Thank you for contributing to dioxus_style! 🎉