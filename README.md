# dioxus_style

**Scoped CSS styling for Dioxus components** - Write CSS that's automatically scoped to your components, preventing style conflicts and maintaining clean, modular code.

[![Crates.io](https://img.shields.io/crates/v/dioxus_style.svg)](https://crates.io/crates/dioxus_style)
[![Documentation](https://docs.rs/dioxus_style/badge.svg)](https://docs.rs/dioxus_style)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

## Features

- 🎯 **Automatic CSS Scoping** - Classes are automatically prefixed with unique hashes
- 📦 **File or Inline CSS** - Load from `.css` files or write inline
- ⚡ **Zero Runtime Overhead** - All processing happens at compile time
- 🔥 **Hot Reload Support** - CSS changes are tracked via `include_str!`
- 🎨 **Multiple Macro Options** - Choose the syntax that fits your style
- 🚀 **Performance Optimized** - Fast hashing (xxHash), efficient parsing, and minification in release builds
- 💾 **Global Style Registry** - Automatic deduplication and insertion order preservation

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
dioxus_style = "0.1.0"
```

## Usage Examples

### 1. Attribute Macro with Auto-Injection (Recommended)

The simplest way - styles are automatically injected:

```rust
use dioxus::prelude::*;
use dioxus_style::with_css;

#[with_css("button.css")]
fn Button() -> Element {
    rsx! {
        button { class: "{css}_btn", "Click me!" }
    }
}
```

**button.css:**
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
```

### 2. Manual Style Management

For more control over when styles are injected:

```rust
use dioxus::prelude::*;
use dioxus_style::{scoped_style, inject_styles};

#[component]
fn Card() -> Element {
    let css = scoped_style!("card.css");
    
    rsx! {
        style { dangerous_inner_html: "{inject_styles()}" }
        div { class: "{css}_card",
            h2 { class: "{css}_title", "Hello" }
            p { class: "{css}_content", "World" }
        }
    }
}
```

### 3. Inline CSS

No external file needed:

```rust
use dioxus::prelude::*;
use dioxus_style::css;

#[component]
fn Badge() -> Element {
    let css = css!("background: red; color: white; padding: 4px 8px;");
    
    rsx! {
        span { class: "{css}", "New" }
    }
}
```

### 4. Function-like Component Macro

Alternative syntax for defining styled components:

```rust
use dioxus::prelude::*;
use dioxus_style::component_with_css;

component_with_css! {
    css: "card.css",
    fn Card() -> Element {
        rsx! {
            div { class: "{css}_card", "Content" }
        }
    }
}
```

## How It Works

### Compile-Time Processing

```rust
let css = scoped_style!("button.css");
// Generates: "sc_a1b2c3d4"
```

**Input CSS:**
```css
.btn { color: red; }
.btn:hover { color: blue; }
```

**Output (scoped):**
```css
.sc_a1b2c3d4_btn { color: red; }
.sc_a1b2c3d4_btn:hover { color: blue; }
```

### Usage in Components

```rust
// Use the scoped class name
button { class: "{css}_btn", "Click" }
// Renders: <button class="sc_a1b2c3d4_btn">Click</button>
```

## Style Injection Strategies

### Auto-Injection (Recommended for Simple Cases)

```rust
#[with_css("styles.css")]
fn MyComponent() -> Element {
    // Styles automatically injected - no manual inject_styles() needed
    rsx! { /* your JSX */ }
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

The library searches for CSS files in multiple locations:

```rust
scoped_style!("button.css")
// Searches:
// 1. ./button.css
// 2. ../button.css
// 3. ../../button.css
// 4. src/button.css
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

- **Compile-time processing**: Zero runtime CSS parsing
- **O(1) style lookups**: HashMap-based registry
- **Deduplication**: Identical styles registered only once
- **Fast hashing**: xxHash3 is one of the fastest non-cryptographic hashes
- **Efficient scoping**: Single-pass CSS transformation

## Architecture

```
┌─────────────────────────────────────┐
│  Your Component (compile time)       │
│  scoped_style!("button.css")        │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│  Procedural Macro                    │
│  • Read CSS file                     │
│  • Generate hash (xxHash3)           │
│  • Scope selectors (.btn → .sc_xxx_btn)│
│  • Minify (release builds)           │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│  Runtime Registry (lazy_static)      │
│  • Store scoped CSS                  │
│  • Deduplicate by hash               │
│  • Preserve insertion order          │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│  inject_styles() → <style> tag       │
│  • Inject into DOM                   │
│  • All styles in single tag          │
└─────────────────────────────────────┘
```

## Examples

### Complete App Structure

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

#[with_css("header.css")]
fn Header() -> Element {
    rsx! {
        header { class: "{css}_header",
            h1 { "My App" }
        }
    }
}

#[with_css("main.css")]
fn Main() -> Element {
    rsx! {
        main { class: "{css}_container",
            Card { title: "Welcome" }
        }
    }
}

#[with_css("card.css")]
fn Card(title: String) -> Element {
    rsx! {
        div { class: "{css}_card",
            h2 { class: "{css}_title", "{title}" }
        }
    }
}
```

## Limitations

- Only class selectors are scoped (`.class`)
- ID selectors (`#id`) and element selectors (`div`) remain global
- Pseudo-classes (`:hover`, `:focus`) are supported
- Complex selectors work: `.btn > .icon`, `.card + .card`

## Troubleshooting

### CSS file not found

```rust
// ❌ Error: Failed to find CSS file 'button.css'
scoped_style!("button.css")

// ✅ Solution: Use relative path from Cargo.toml location
scoped_style!("src/components/button.css")
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

### Class name doesn't match

```rust
// CSS file
.button { color: red; }

// ❌ Wrong class name
button { class: "{css}_btn" }

// ✅ Match the class name exactly
button { class: "{css}_button" }
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Credits

Built for the [Dioxus](https://dioxuslabs.com/) framework.

---

**Made with ❤️ for the Dioxus community**