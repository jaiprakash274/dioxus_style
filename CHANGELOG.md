# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-11-15

### Added
- Initial release of dioxus_style
- `scoped_style!` macro for file-based or inline CSS scoping
- `css!` macro for inline utility styles
- `#[with_css]` attribute macro with automatic style injection
- `component_with_css!` function-like macro for styled components
- Automatic CSS scoping with xxHash-based unique identifiers
- Global style registry with deduplication
- `inject_styles()` function for manual style injection
- Compile-time CSS processing and minification
- Hot reload support via `include_str!` file tracking
- Multiple CSS file path resolution strategies
- Performance optimizations:
  - Fast xxHash3 hashing
  - Efficient single-pass CSS parsing
  - HashMap-based O(1) style lookups
  - Automatic minification in release builds
- Comprehensive test coverage
- Full documentation and examples

### Features
- Zero runtime CSS parsing overhead
- Automatic class name scoping (`.btn` → `.sc_xxx_btn`)
- Support for pseudo-classes and complex selectors
- Insertion order preservation in style registry
- Thread-safe global registry using `lazy_static` and `Mutex`

## [Unreleased]

### Planned
- Support for CSS nesting syntax
- CSS preprocessor integration (SCSS, LESS)
- Source maps for debugging
- CSS variables scoping
- Animation keyframe scoping
- Media query optimization
- Plugin system for custom transformations

---

## Version History

### Release Notes

#### v0.1.0 - Initial Public Release
This is the first stable release of dioxus_style, providing a complete solution for scoped CSS in Dioxus applications.

**Key Highlights:**
- Production-ready scoped CSS system
- Multiple ergonomic APIs for different use cases
- Optimized for both development and production builds
- Comprehensive documentation with examples
- Full test coverage

**Migration Guide:**
N/A - This is the initial release.

**Breaking Changes:**
N/A - This is the initial release.

---

For more details, see the [GitHub Releases](https://github.com/yourusername/dioxus_style/releases) page.