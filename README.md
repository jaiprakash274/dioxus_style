# dioxus_style

**Scoped CSS/SCSS styling for Dioxus components** - Write CSS or SCSS that's automatically scoped to your components, preventing style conflicts and maintaining clean, modular code.

[![Crates.io](https://img.shields.io/crates/v/dioxus_style.svg)](https://crates.io/crates/dioxus_style)
[![Documentation](https://docs.rs/dioxus_style/badge.svg)](https://docs.rs/dioxus_style)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## ✨ Features

- 🎨 **SCSS/SASS Support** - Variables, nesting, mixins, and all SCSS features (NEW in v0.3.0!)
- 🎯 **Automatic CSS Scoping** - Classes, IDs, and elements are automatically scoped
- 🏷️ **Element Scoping** - Element selectors (`div`, `span`) use data attributes for isolation
- 📦 **File or Inline Styles** - Load from `.css`/`.scss` files or write inline
- ⚡ **Zero Runtime Overhead** - All processing happens at compile time
- 🔥 **Hot Reload Support** - CSS/SCSS changes are tracked via `include_str!`
- 🎨 **Multiple Macro Options** - Choose the syntax that fits your style
- 🚀 **Performance Optimized** - Fast hashing (xxHash), efficient parsing, and minification in release builds
- 💾 **Global Style Registry** - Automatic deduplication and insertion order preservation

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus_style = "0.3.0"  # SCSS enabled by default

# Or explicitly enable SCSS
dioxus_style = { version = "0.3.0", features = ["scss"] }

# Or use without SCSS (smaller dependencies)
dioxus_style = { version = "0.3.0", default-features = false }
```

## 🆕 SCSS Support (v0.3.0+)

Write modern, maintainable styles with SCSS! All compilation happens at build time.

### SCSS Example

**button.scss:**
```scss
$primary-color: #3498db;
$padding: 10px 20px;

.btn {
    background: $primary-color;
    color: white;
    padding: $padding;
    border-radius: 5px;
    
    &:hover {
        background: darken($primary-color, 10%);
        transform: scale(1.05);
    }
    
    &:active {
        transform: scale(0.98);
    }
}
```

**Component:**
```rust
use dioxus::prelude::*;
use dioxus_style::with_css;

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

### Inline SCSS

```rust
use dioxus_style::scoped_style;

fn Badge() -> Element {
    let css = scoped_style!("
        $danger: #e74c3c;
        
        .badge {
            background: $danger;
            color: white;
            padding: 4px 8px;
            
            &:hover {
                background: darken($danger, 10%);
            }
        }
    ");
    
    rsx! {
        span { 
            "data-scope": "{css}",
            class: "{css}_badge", 
            "New" 
        }
    }
}
```

## Usage Examples

### 1. Attribute Macro with Auto-Injection (Recommended)

The simplest way - styles are automatically injected:

```rust
use dioxus::prelude::*;
use dioxus_style::with_css;

#[with_css("button.css")]  // or "button.scss"
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

**button.css (or button.scss):**
```css
.btn {
    background: blue;
    color: white;
    padding: 10px 20px;
    border-radius: 5px;
}

.btn:hover {
    background: darkblue;
}

button {
    cursor: pointer;
    border: none;
}
```

### 2. Manual Style Management

For more control over when styles are injected:

```rust
use dioxus::prelude::*;
use dioxus_style::{scoped_style, inject_styles};

#[component]
fn Card() -> Element {
    let css = scoped_style!("card.scss");  // Supports both .css and .scss
    
    rsx! {
        style { dangerous_inner_html: "{inject_styles()}" }
        div { 
            "data-scope": "{css}",
            class: "{css}_card",
            h2 { 
                "data-scope": "{css}",
                class: "{css}_title", 
                "Hello" 
            }
            p { 
                "data-scope": "{css}",
                class: "{css}_content", 
                "World" 
            }
        }
    }
}
```

### 3. Inline CSS/SCSS

No external file needed:

```rust
use dioxus::prelude::*;
use dioxus_style::css;

#[component]
fn Badge() -> Element {
    // Plain CSS
    let css = css!("background: red; color: white; padding: 4px 8px;");
    
    rsx! {
        span { 
            "data-scope": "{css}",
            class: "{css}", 
            "New" 
        }
    }
}

#[component]
fn ScssCard() -> Element {
    // Inline SCSS
    let css = css!("
        $spacing: 1rem;
        
        background: white;
        padding: $spacing;
        
        &:hover {
            box-shadow: 0 2px 8px rgba(0,0,0,0.1);
        }
    ");
    
    rsx! {
        div { 
            "data-scope": "{css}",
            class: "{css}", 
            "Hover me" 
        }
    }
}
```

### 4. Function-like Component Macro

Alternative syntax for defining styled components:

```rust
use dioxus::prelude::*;
use dioxus_style::component_with_css;

component_with_css! {
    css: "card.scss",  // Supports both .css and .scss
    fn Card() -> Element {
        rsx! {
            div { 
                "data-scope": "{css}",
                class: "{css}_card", 
                "Content" 
            }
        }
    }
}
```

## How It Works

### Compile-Time Processing

```rust
let css = scoped_style!("button.scss");
// 1. Compiles SCSS to CSS (if .scss file)
// 2. Scopes all selectors
// 3. Generates: "sc_a1b2c3d4"
```

**Input SCSS:**
```scss
$primary: red;

.btn { 
    color: $primary;
    
    &:hover { 
        color: blue; 
    }
}

div { 
    margin: 10px; 
}

#header { 
    font-size: 24px; 
}
```

**After SCSS Compilation:**
```css
.btn { color: red; }
.btn:hover { color: blue; }
div { margin: 10px; }
#header { font-size: 24px; }
```

**After Scoping:**
```css
.sc_a1b2c3d4_btn { color: red; }
.sc_a1b2c3d4_btn:hover { color: blue; }
div[data-scope="sc_a1b2c3d4"] { margin: 10px; }
#sc_a1b2c3d4_header { font-size: 24px; }
```

### Usage in Components

```rust
// Use the scoped class name and data-scope attribute
button { 
    "data-scope": "{css}",
    class: "{css}_btn", 
    "Click" 
}
// Renders: <button data-scope="sc_a1b2c3d4" class="sc_a1b2c3d4_btn">Click</button>
```

## SCSS Features Supported

All standard SCSS features are supported:

- ✅ **Variables**: `$primary: #3498db;`
- ✅ **Nesting**: Parent-child selector relationships
- ✅ **Parent Selector**: `&` for modifiers and pseudo-classes
- ✅ **Mixins**: `@mixin` and `@include`
- ✅ **Functions**: `darken()`, `lighten()`, `rgba()`, etc.
- ✅ **Imports**: `@import` and `@use`
- ✅ **Extends**: `@extend`
- ✅ **Operators**: `+`, `-`, `*`, `/`, `%`
- ✅ **Interpolation**: `#{$variable}`
- ✅ **Control Directives**: `@if`, `@for`, `@each`, `@while`
- ✅ **Maps and Lists**: Complex data structures
- ✅ **Built-in Functions**: All SCSS built-in functions

### SCSS Examples

**Variables and Operations:**
```scss
$base-spacing: 1rem;

.container {
    padding: $base-spacing * 2;
    margin: $base-spacing / 2;
}
```

**Nesting:**
```scss
.card {
    background: white;
    
    .title {
        font-size: 1.5rem;
    }
    
    .content {
        color: #666;
    }
}
```

**Mixins:**
```scss
@mixin flex-center {
    display: flex;
    justify-content: center;
    align-items: center;
}

.container {
    @include flex-center;
}
```

**Parent Selector:**
```scss
.button {
    background: blue;
    
    &:hover {
        background: darkblue;
    }
    
    &--large {
        padding: 1rem 2rem;
    }
}
```

## Scoping Behavior

### What Gets Scoped

| Selector Type | Input | Output | Usage |
|--------------|-------|---------|-------|
| **Class** | `.btn` | `.sc_xxx_btn` | `class: "{css}_btn"` |
| **ID** | `#header` | `#sc_xxx_header` | `id: "{css}_header"` |
| **Element** | `div` | `div[data-scope="sc_xxx"]` | `"data-scope": "{css}"` |
| **Pseudo-class** | `.btn:hover` | `.sc_xxx_btn:hover` | (automatic) |
| **Complex** | `.card > .title` | `.sc_xxx_card > .sc_xxx_title` | (automatic) |

### Element Scoping

Elements are scoped using `data-scope` attributes:

```rust
// SCSS
$spacing: 20px;

div { 
    padding: $spacing; 
}

span.highlight { 
    color: yellow; 
}

// Component
rsx! {
    div { 
        "data-scope": "{css}",
        class: "{css}_container",
        span { 
            "data-scope": "{css}",
            class: "{css}_highlight",
            "Text"
        }
    }
}
```

## Style Injection Strategies

### Auto-Injection (Recommended for Simple Cases)

```rust
#[with_css("styles.scss")]
fn MyComponent() -> Element {
    // Styles automatically injected - no manual inject_styles() needed
    rsx! { 
        div { 
            "data-scope": "{css}",
            /* your content */ 
        } 
    }
}
```

### Manual Injection (Recommended for Root Component)

```rust
#[component]
fn App() -> Element {
    rsx! {
        // Inject ALL registered styles once at the root
        style { dangerous_inner_html: "{inject_styles()}" }
        
        // Your components
        MyComponent {}
        AnotherComponent {}
    }
}
```

## Advanced Features

### CSS File Path Resolution

The library searches for CSS/SCSS files in multiple locations:

```rust
scoped_style!("button.scss")
// Searches:
// 1. ./button.scss
// 2. ../button.scss
// 3. ../../button.scss
// 4. src/button.scss
```

### Automatic File Type Detection

```rust
#[with_css("button.scss")]  // ✅ SCSS file - automatically compiled
#[with_css("button.css")]   // ✅ CSS file - scoped only
```

### Complex Selectors

All complex selectors are fully supported in both CSS and SCSS:

```scss
// SCSS with nesting
.parent {
    > .child { 
        color: blue; 
    }
}

// Compiles to: .sc_xxx_parent > .sc_xxx_child { color: blue; }

// Adjacent sibling
.card + .card { 
    margin-top: 20px; 
}
// Output: .sc_xxx_card + .sc_xxx_card { margin-top: 20px; }

// Mixed selectors with SCSS
div.container {
    > span#label { 
        font-weight: bold; 
    }
}
// Output: div[data-scope="sc_xxx"].sc_xxx_container > span[data-scope="sc_xxx"]#sc_xxx_label
```

### Minification

In release builds, CSS is automatically minified:

```rust
// Debug: Preserves formatting for readability
// Release: Removes whitespace and comments for smaller bundles
```

### Hash Generation

Uses xxHash (XXH3) for fast, collision-resistant hashing:

```rust
// Hash includes file path + content for uniqueness
// Format: "sc_" + base62(hash)
// Example: "sc_3xK9mP2"
```

## Performance Characteristics

- **Compile-time SCSS compilation**: Zero runtime SCSS parsing
- **Compile-time CSS scoping**: Zero runtime CSS transformation
- **O(1) style lookups**: HashMap-based registry
- **Deduplication**: Identical styles registered only once
- **Fast hashing**: xxHash3 is one of the fastest non-cryptographic hashes
- **Efficient scoping**: Single-pass CSS transformation with optimized state machine

## Architecture

```
┌───────────────────────────────────┐
│  Your Component (compile time)    │
│  scoped_style!("button.scss")     │
└──────────────┬────────────────────┘
               ↓
┌───────────────────────────────────┐
│  SCSS Compiler (if .scss)         │
│  • Compile SCSS to CSS            │
│  • Variables, nesting, mixins     │
│  • Functions and operations       │
└──────────────┬────────────────────┘
               ↓
┌───────────────────────────────────┐
│  Procedural Macro                 │
│  • Read CSS/SCSS file             │
│  • Generate hash (xxHash3)        │
│  • Scope selectors:               │
│    - .btn → .sc_xxx_btn           │
│    - #id → #sc_xxx_id             │
│    - div → div[data-scope="..."]  │
│  • Minify (release builds)        │
└──────────────┬────────────────────┘
               ↓
┌───────────────────────────────────┐
│  Runtime Registry (lazy_static)   │
│  • Store scoped CSS               │
│  • Deduplicate by hash            │
│  • Preserve insertion order       │
└──────────────┬────────────────────┘
               ↓
┌───────────────────────────────────┐
│  inject_styles() → <style> tag    │
│  • Inject into DOM                │
│  • All styles in single tag       │
└───────────────────────────────────┘
```

## Examples

### Complete App Structure with SCSS

```rust
use dioxus::prelude::*;
use dioxus_style::{with_css, inject_styles};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        style { dangerous_inner_html: "{inject_styles()}" }
        Header {}
        Main {}
        Footer {}
    }
}

#[with_css("header.scss")]
fn Header() -> Element {
    rsx! {
        header { 
            "data-scope": "{css}",
            class: "{css}_header",
            h1 { 
                "data-scope": "{css}",
                "My App" 
            }
        }
    }
}

#[with_css("main.scss")]
fn Main() -> Element {
    rsx! {
        main { 
            "data-scope": "{css}",
            class: "{css}_container",
            Card { title: "Welcome" }
        }
    }
}

#[with_css("card.scss")]
fn Card(title: String) -> Element {
    rsx! {
        div { 
            "data-scope": "{css}",
            class: "{css}_card",
            h2 { 
                "data-scope": "{css}",
                class: "{css}_title", 
                "{title}" 
            }
        }
    }
}
```

**header.scss:**
```scss
$header-bg: #2c3e50;

.header {
    background: $header-bg;
    padding: 1rem;
    
    h1 {
        color: white;
        margin: 0;
    }
}
```

## Scoping Rules Summary

### ✅ Automatically Scoped

- **Classes**: `.button` → `.sc_xxx_button`
- **IDs**: `#header` → `#sc_xxx_header`
- **Elements**: `div` → `div[data-scope="sc_xxx"]` (requires `data-scope` attribute)
- **Pseudo-classes**: `:hover`, `:focus`, `:active`, etc.
- **Pseudo-elements**: `::before`, `::after`
- **Attribute selectors**: `[type="text"]` (passed through, element gets scoped)
- **Complex selectors**: All combinators (`>`, `+`, `~`, space)

### ❌ Not Scoped (Global)

- **Universal selector**: `*`
- **:root**: CSS variables at root level
- **@keyframes**: Animation definitions (use unique names)
- **@media, @supports**: Query blocks (contents are scoped)

## Migration Guides

### From v0.2.0 to v0.3.0 (SCSS Support)

**Good News**: This is a NON-BREAKING release! All v0.2.0 code works without changes.

**To Use SCSS:**
1. Rename `.css` files to `.scss` (optional)
2. Start using SCSS features
3. Or keep using plain CSS - both work!

**Cargo.toml:**
```toml
# Before (v0.2.0)
[dependencies]
dioxus_style = "0.2.0"

# After (v0.3.0) - SCSS enabled by default
[dependencies]
dioxus_style = "0.3.0"

# Or without SCSS
dioxus_style = { version = "0.3.0", default-features = false }
```

### From v0.1.0 to v0.2.0+ (Element Scoping)

**Breaking Changes:**

1. **Element selectors now require `data-scope`:**
   ```rust
   // OLD (v0.1.0)
   rsx! { div { class: "{css}_container", "Content" } }
   
   // NEW (v0.2.0+)
   rsx! { 
       div { 
           "data-scope": "{css}",
           class: "{css}_container", 
           "Content" 
       } 
   }
   ```

2. **Class selector output format changed:**
   - Old: `.sc_xxx.button`
   - New: `.sc_xxx_button`

3. **ID selector output format changed:**
   - Old: `#sc_xxx.header`
   - New: `#sc_xxx_header`

## Troubleshooting

### SCSS compilation error

```
error: SCSS compilation error in 'button.scss': Undefined variable
```

**Solution:** Check that all SCSS variables are defined:
```scss
// ❌ Wrong
.button { color: $undefined; }

// ✅ Correct
$primary: blue;
.button { color: $primary; }
```

### SCSS feature not available

```
error: SCSS support is not enabled
```

**Solution:** Enable SCSS in Cargo.toml:
```toml
[dependencies]
dioxus_style = { version = "0.3.0", features = ["scss"] }
```

### CSS/SCSS file not found

```rust
// ❌ Error: Failed to find file 'button.scss'
scoped_style!("button.scss")

// ✅ Solution: Use relative path from Cargo.toml location
scoped_style!("src/components/button.scss")
```

### Styles not appearing

```rust
// ❌ Forgot to inject styles
#[component]
fn App() -> Element {
    rsx! { MyComponent {} }
}

// ✅ Add inject_styles() to root component
#[component]
fn App() -> Element {
    rsx! {
        style { dangerous_inner_html: "{inject_styles()}" }
        MyComponent {}
    }
}
```

### Element styles not working

```rust
// ❌ Missing data-scope attribute
div { class: "{css}_container", "Content" }

// ✅ Add data-scope for element scoping
div { 
    "data-scope": "{css}",
    class: "{css}_container", 
    "Content" 
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Credits

Built for the [Dioxus](https://dioxuslabs.com/) framework.

SCSS compilation powered by [grass](https://github.com/connorskees/grass).

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for detailed version history.

---

**Made with ❤️ for the Dioxus community**