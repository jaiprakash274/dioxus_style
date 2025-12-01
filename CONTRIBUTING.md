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
git clone https://github.com/jaiprakash274/dioxus_style.git
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

#### Testing Scoping Behavior (v0.2.0+)

When testing selector scoping, ensure you cover:

```rust
#[test]
fn test_element_scoping() {
    let css = "div { margin: 10px; }";
    let scoped = parse_and_scope(css, "sc_test", false);
    assert!(scoped.scoped.contains("div[data-scope=\"sc_test\"]"));
}

#[test]
fn test_mixed_selector() {
    let css = "div.container > span#label { color: red; }";
    let scoped = parse_and_scope(css, "sc_test", false);
    // Test element scoping
    assert!(scoped.scoped.contains("div[data-scope=\"sc_test\"]"));
    // Test class scoping
    assert!(scoped.scoped.contains(".sc_test_container"));
    // Test ID scoping
    assert!(scoped.scoped.contains("#sc_test_label"));
}
```

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
cargo test test_element_scoping

# With output
cargo test -- --nocapture

# Run only style_parser tests
cargo test -p dioxus_style_macro style_parser
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

### Testing Selector Scoping

When modifying `style_parser.rs`, ensure all selector types are tested:

```rust
// Test class selectors
#[test]
fn test_class_selector() {
    let css = ".button { color: red; }";
    let scoped = parse_and_scope(css, "sc_test", false);
    assert!(scoped.scoped.contains(".sc_test_button"));
}

// Test element selectors
#[test]
fn test_element_selector() {
    let css = "div { padding: 10px; }";
    let scoped = parse_and_scope(css, "sc_test", false);
    assert!(scoped.scoped.contains("div[data-scope=\"sc_test\"]"));
}

// Test ID selectors
#[test]
fn test_id_selector() {
    let css = "#header { font-size: 24px; }";
    let scoped = parse_and_scope(css, "sc_test", false);
    assert!(scoped.scoped.contains("#sc_test_header"));
}

// Test complex selectors
#[test]
fn test_complex_selector() {
    let css = "div.parent > span + .child { margin: 5px; }";
    let scoped = parse_and_scope(css, "sc_test", false);
    // Verify all parts are correctly scoped
    assert!(scoped.scoped.contains("div[data-scope=\"sc_test\"]"));
    assert!(scoped.scoped.contains(".sc_test_parent"));
    assert!(scoped.scoped.contains("span[data-scope=\"sc_test\"]"));
    assert!(scoped.scoped.contains(".sc_test_child"));
}
```

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

### Example Documentation Format

```rust
/// Scopes a CSS selector with a unique prefix.
///
/// # Arguments
///
/// * `selector` - The CSS selector to scope (e.g., ".button")
/// * `scope` - The unique scope prefix (e.g., "sc_abc123")
/// * `class_names` - HashSet to track discovered class names
///
/// # Returns
///
/// A scoped CSS selector string
///
/// # Examples
///
/// ```
/// use dioxus_style_macro::style_parser::scope_selector;
/// use std::collections::HashSet;
///
/// let mut classes = HashSet::new();
/// let scoped = scope_selector(".btn", "sc_test", &mut classes);
/// assert_eq!(scoped, ".sc_test_btn");
/// ```
pub fn scope_selector(
    selector: &str, 
    scope: &str, 
    class_names: &mut HashSet<String>
) -> String {
    // Implementation
}
```

## Release Process

(For maintainers)

1. Update version numbers in all `Cargo.toml` files
2. Update `CHANGELOG.md` with new version
3. Update documentation and examples
4. Run full test suite: `cargo test --all`
5. Check formatting: `cargo fmt --all -- --check`
6. Run clippy: `cargo clippy --all-targets --all-features -- -D warnings`
7. Create git tag: `git tag -a v0.2.0 -m "Release v0.2.0"`
8. Push tag: `git push origin v0.2.0`
9. Publish to crates.io:
   ```bash
   cd dioxus_style_macro && cargo publish
   # Wait a few minutes
   cd ../dioxus_style && cargo publish
   ```

## Areas for Contribution

Here are some areas where contributions would be especially welcome:

### High Priority
- [ ] Support for CSS nesting syntax
- [ ] Better pseudo-element scoping
- [ ] Universal selector (`*`) scoping strategy
- [ ] Performance benchmarks
- [ ] More comprehensive examples

### Medium Priority
- [ ] CSS preprocessor integration (SCSS, LESS)
- [ ] Source maps for debugging
- [ ] Animation keyframe scoping
- [ ] Media query optimization
- [ ] CSS variables scoping

### Low Priority
- [ ] Plugin system for custom transformations
- [ ] IDE integration helpers
- [ ] Visual regression testing
- [ ] Performance profiling tools

## Getting Help

- **GitHub Issues**: For bugs and feature requests
- **Discussions**: For questions and general discussion
- **Discord**: Join the Dioxus community server

## Code Review Process

All contributions go through code review. Reviewers will check for:

- **Correctness**: Does the code work as intended?
- **Tests**: Are there adequate tests?
- **Documentation**: Is the code well-documented?
- **Style**: Does it follow Rust conventions?
- **Performance**: Are there any performance concerns?
- **Breaking Changes**: Are they necessary and well-documented?

## Version Policy

We follow [Semantic Versioning](https://semver.org/):

- **Patch (0.2.x)**: Bug fixes, documentation
- **Minor (0.x.0)**: New features, non-breaking changes
- **Major (x.0.0)**: Breaking API changes

## License

By contributing to dioxus_style, you agree that your contributions will be licensed under both the MIT License and Apache License 2.0.

## Recognition

Contributors will be acknowledged in:
- GitHub contributors list
- Release notes
- Project documentation

## Questions?

If you have questions about contributing, feel free to:
- Open a GitHub Discussion
- Ask in the Dioxus Discord server
- Email the maintainers

Thank you for contributing to dioxus_style! 🎉