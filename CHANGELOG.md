# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-12-01

### Added
- **Element Selector Scoping** - Elements like `div`, `span`, `p` are now automatically scoped using `data-scope` attributes
  - Example: `div { margin: 10px; }` becomes `div[data-scope="sc_xxx"] { margin: 10px; }`
  - Works with complex selectors: `div.container > span` becomes `div[data-scope="sc_xxx"].sc_xxx_container > span[data-scope="sc_xxx"]`
- ID selector scoping now uses underscore prefix (e.g., `#header` becomes `#sc_xxx_header`)
- Enhanced selector parsing for better handling of mixed selectors (classes, IDs, and elements)

### Changed
- **Breaking**: Element selectors are now scoped by default - components must include `data-scope` attribute on element tags
- Improved scoping algorithm to handle element + class combinations (e.g., `div.container`)
- Better handling of combinator spacing in complex selectors
- Class selectors now use dot notation (`.sc_xxx_class` instead of `.sc_xxx.class`)

### Fixed
- Fixed selector parsing for adjacent sibling combinators (`+`)
- Fixed handling of element selectors at the start of complex selectors
- Improved whitespace handling in minified CSS
- Better attribute selector pass-through logic

### Performance
- Optimized selector parsing with improved state machine
- Reduced string allocations in scoping operations
- More efficient combinator detection

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
- Universal selector (`*`) scoping
- Pseudo-element (`::`-based) scoping improvements

---

## Version History

### Release Notes

#### v0.2.0 - Element Scoping Release
This release adds comprehensive element selector scoping, making dioxus_style even more powerful for preventing style conflicts. Element selectors are now automatically scoped using data attributes.

**Key Highlights:**
- Full element selector scoping support
- Enhanced complex selector handling
- Improved ID selector scoping format
- Better handling of mixed selector types

**Migration Guide from v0.1.0:**

**Breaking Changes:**
1. **Element selectors now require `data-scope` attribute:**
   ```rust
   // v0.1.0 (elements were not scoped)
   rsx! { div { class: "{css}_container", "Content" } }
   
   // v0.2.0 (elements need data-scope)
   rsx! { 
       div { 
           "data-scope": "{css}",
           class: "{css}_container", 
           "Content" 
       } 
   }
   ```

2. **Class selector format changed:**
   ```css
   /* v0.1.0 output */
   .sc_abc.button { color: red; }
   
   /* v0.2.0 output */
   .sc_abc_button { color: red; }
   ```

3. **ID selector format standardized:**
   ```css
   /* v0.1.0 output */
   #sc_abc.header { color: blue; }
   
   /* v0.2.0 output */
   #sc_abc_header { color: blue; }
   ```

**Upgrade Steps:**
1. Update `dioxus_style` dependency to `0.2.0`
2. Add `data-scope` attribute to all element tags that need scoping:
   ```rust
   div { "data-scope": "{css}", class: "{css}_myclass", ... }
   ```
3. Review CSS that uses element selectors - they will now be scoped
4. Test thoroughly to ensure styling still works as expected

#### v0.1.0 - Initial Public Release
This is the first stable release of dioxus_style, providing a complete solution for scoped CSS in Dioxus applications.

**Key Highlights:**
- Production-ready scoped CSS system
- Multiple ergonomic APIs for different use cases
- Optimized for both development and production builds
- Comprehensive documentation with examples
- Full test coverage

---

For more details, see the [GitHub Releases](https://github.com/jaiprakash274/dioxus_style/releases) page.