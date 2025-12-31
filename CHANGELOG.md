# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2025-12-31

### Added
- **Universal Selector (`*`) Support** - The universal selector now passes through without scoping
  - Example: `* { margin: 0; }` remains unchanged (global reset styles)
- **`:root` and `:host` Pseudo-class Support** - Global pseudo-classes are now properly recognized
  - `:root` and `:host` selectors pass through without element scoping
  - CSS custom properties defined in `:root` work correctly
- **Improved At-Rule Handling** - Better handling of CSS at-rules
  - `@keyframes` and `@font-face` pass through unchanged (no scoping)
  - `@media`, `@supports`, `@layer`, `@container` properly scope nested rules
  - Animation names in `@keyframes` are preserved

### Changed
- Removed debug `eprintln!` statements from macro compilation output
- Cleaner compile-time output for better developer experience

### Fixed
- Edge case handling for CSS selectors with universal selector
- Proper handling of `:root { --custom-property: value; }` patterns

### Examples
```rust
// Universal selector (global resets) - unchanged
let css = scoped_style!("* { margin: 0; padding: 0; }");

// :root variables - unchanged
let css = scoped_style!(":root { --primary-color: #3498db; }");

// @keyframes - animation names preserved
let css = scoped_style!("
    @keyframes fadeIn {
        from { opacity: 0; }
        to { opacity: 1; }
    }
");

// @media with scoped nested rules
let css = scoped_style!("
    @media (max-width: 768px) {
        .container { padding: 10px; }
    }
");
```

## [0.3.0] - 2025-12-28

### Added
- **SCSS/SASS Support** - Full SCSS and SASS compilation support at compile time
  - Automatic detection of `.scss` and `.sass` files
  - Inline SCSS compilation for `scoped_style!` and `css!` macros
  - Variables, nesting, mixins, and all SCSS features supported
  - Uses the `grass` crate for fast, standards-compliant compilation
- **Optional SCSS Feature** - Enable with `features = ["scss"]` (enabled by default)
- Automatic SCSS minification in release builds
- Compile-time SCSS error reporting with file context
- Smart detection of SCSS syntax in inline styles

### Changed
- Default features now include SCSS support
- File path resolution now supports `.scss` and `.sass` extensions
- Enhanced error messages for SCSS compilation failures
- Improved inline style detection to differentiate CSS from SCSS

### Performance
- SCSS compilation happens at compile time (zero runtime overhead)
- Compiled CSS is cached during build
- Minified output in release builds reduces bundle size

### Examples
```rust
// Use SCSS file
#[with_css("button.scss")]
fn Button() -> Element { ... }

// Inline SCSS
let css = scoped_style!("
    $primary: #3498db;
    
    .button {
        background: $primary;
        
        &:hover {
            background: darken($primary, 10%);
        }
    }
");
```

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
- CSS preprocessor integration (LESS)
- Source maps for debugging
- CSS variables scoping
- Media query optimization
- Plugin system for custom transformations
- Pseudo-element (`::`-based) scoping improvements

---

## Version History

### Release Notes

#### v0.4.0 - Enhanced CSS Selector Support Release
This release improves CSS selector handling with support for universal selectors, global pseudo-classes, and better at-rule processing.

**Key Highlights:**
- Universal selector (`*`) passes through unchanged for global resets
- `:root` and `:host` pseudo-classes properly recognized
- `@keyframes` and `@font-face` pass through without scoping
- `@media`, `@supports`, `@layer`, `@container` properly scope nested rules
- Removed debug output for cleaner compilation

**Migration Guide from v0.3.0:**

**Non-Breaking Changes:**
This is a fully backward compatible release. Existing CSS/SCSS files work without any changes.

**Cargo.toml Configuration:**
```toml
[dependencies]
dioxus_style = "0.4.0"

# With SCSS support
dioxus_style = { version = "0.4.0", features = ["scss"] }
```

#### v0.3.0 - SCSS Support Release
This release adds full SCSS/SASS support, allowing you to write modern, maintainable styles with variables, nesting, mixins, and more. All SCSS compilation happens at compile time for zero runtime overhead.

**Key Highlights:**
- Full SCSS/SASS support via the `grass` crate
- Automatic `.scss`/`.sass` file detection
- Inline SCSS compilation
- Variables, nesting, mixins, functions, and all SCSS features
- Compile-time error reporting
- Optional feature flag for projects that don't need SCSS

**Migration Guide from v0.2.0:**

**Non-Breaking Changes:**
SCSS support is fully backward compatible. Existing CSS files work without any changes.

**To Use SCSS:**
1. Rename `.css` files to `.scss` (optional)
2. Start using SCSS features like variables and nesting
3. Or keep using plain CSS - both work!

**Cargo.toml Configuration:**
```toml
# SCSS enabled by default
[dependencies]
dioxus_style = "0.3.0"

# Or explicitly enable
dioxus_style = { version = "0.3.0", features = ["scss"] }

# Disable SCSS if not needed (smaller dependency tree)
dioxus_style = { version = "0.3.0", default-features = false }
```

**SCSS Examples:**

File-based SCSS:
```rust
#[with_css("button.scss")]
fn Button() -> Element {
    rsx! {
        button { 
            "data-scope": "{css}",
            class: "{css}_btn", 
            "Click me!" 
        }
    }
}
```

**button.scss:**
```scss
$primary: #3498db;
$padding: 10px 20px;

.btn {
    background: $primary;
    color: white;
    padding: $padding;
    border-radius: 5px;
    
    &:hover {
        background: darken($primary, 10%);
    }
    
    &:active {
        transform: scale(0.98);
    }
}
```

Inline SCSS:
```rust
let css = scoped_style!("
    $spacing: 1rem;
    
    .card {
        padding: $spacing;
        
        .title {
            margin-bottom: $spacing / 2;
        }
    }
");
```

**SCSS Features Supported:**
- ✅ Variables (`$variable`)
- ✅ Nesting (parent-child selectors)
- ✅ Parent selector (`&`)
- ✅ Mixins (`@mixin`, `@include`)
- ✅ Functions (`darken()`, `lighten()`, etc.)
- ✅ Imports (`@import`, `@use`)
- ✅ Extends (`@extend`)
- ✅ Operators (`+`, `-`, `*`, `/`)
- ✅ Interpolation (`#{$var}`)
- ✅ Control directives (`@if`, `@for`, `@each`, `@while`)

**Compile-Time Errors:**
SCSS errors are caught at compile time with helpful messages:
```
error: SCSS compilation error in 'button.scss': 
  Undefined variable: "$undefined-color"
  --> button.scss:5:15
```

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
   
   // v0.2.0+ (elements need data-scope)
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
   
   /* v0.2.0+ output */
   .sc_abc_button { color: red; }
   ```

3. **ID selector format standardized:**
   ```css
   /* v0.1.0 output */
   #sc_abc.header { color: blue; }
   
   /* v0.2.0+ output */
   #sc_abc_header { color: blue; }
   ```

**Upgrade Steps:**
1. Update `dioxus_style` dependency to `0.2.0` or later
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