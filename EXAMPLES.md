# 🎨 Dioxus Style - Complete Examples with SCSS Support

## 📁 Project Structure

```
project/
├── src/
│   └── main.rs                          # Main application with all examples
├── examples/
│   ├── class_selectors.css              # .class examples
│   ├── id_selectors.css                 # #id examples
│   ├── element_selectors.css            # div, span, p examples
│   ├── complex_selectors.css            # >, +, ~, space combinators
│   ├── pseudo_classes.css               # :hover, :focus, :active
│   ├── multiple_selectors.css           # Comma-separated selectors
│   ├── mixed_complex.css                # Everything combined
│   │
│   │   🆕 SCSS Examples (v0.3.0+)
│   ├── variables.scss                   # SCSS variables
│   ├── nesting.scss                     # SCSS nesting
│   ├── mixins.scss                      # SCSS mixins
│   ├── functions.scss                   # SCSS functions
│   └── advanced.scss                    # Advanced SCSS features
└── Cargo.toml
```

---

## 🚀 Quick Start

```bash
# Run the complete examples
cargo run

# Build with SCSS support (enabled by default)
cargo build --features scss

# Build without SCSS (smaller dependencies)
cargo build --no-default-features
```

---

## 📋 Feature Support Table

| Feature Type | Input | Output | Status |
|--------------|-------|--------|--------|
| **Class** | `.button` | `.sc_abc_button` | ✅ Full Support |
| **ID** | `#header` | `#sc_abc_header` | ✅ Full Support |
| **Element** | `div` | `div[data-scope="sc_abc"]` | ✅ Full Support |
| **Pseudo-class** | `.btn:hover` | `.sc_abc_btn:hover` | ✅ Full Support |
| **Child** | `.parent > .child` | `.sc_abc_parent > .sc_abc_child` | ✅ Full Support |
| **Adjacent** | `.card + .card` | `.sc_abc_card + .sc_abc_card` | ✅ Full Support |
| **Sibling** | `.box ~ .box` | `.sc_abc_box ~ .sc_abc_box` | ✅ Full Support |
| **Descendant** | `.parent .child` | `.sc_abc_parent .sc_abc_child` | ✅ Full Support |
| **Multiple** | `.a, .b, #c` | `.sc_abc_a, .sc_abc_b, #sc_abc_c` | ✅ Full Support |
| **SCSS Variables** | `$var: value` | Compiled to CSS | ✅ Full Support (v0.3.0+) |
| **SCSS Nesting** | `.a { .b {} }` | Compiled to CSS | ✅ Full Support (v0.3.0+) |
| **SCSS Mixins** | `@mixin`, `@include` | Compiled to CSS | ✅ Full Support (v0.3.0+) |

---

## 🆕 SCSS Examples (v0.3.0+)

### 1️⃣ SCSS Variables

**SCSS File:** `examples/variables.scss`

```scss
// Define reusable variables
$primary-color: #3498db;
$secondary-color: #2ecc71;
$danger-color: #e74c3c;

$spacing-unit: 1rem;
$border-radius: 8px;

$font-stack: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;

// Use in styles
.button {
    background: $primary-color;
    color: white;
    padding: $spacing-unit ($spacing-unit * 2);
    border-radius: $border-radius;
    font-family: $font-stack;
    
    &.secondary {
        background: $secondary-color;
    }
    
    &.danger {
        background: $danger-color;
    }
}

.card {
    padding: $spacing-unit * 2;
    border-radius: $border-radius;
    margin-bottom: $spacing-unit;
}
```

**Usage in Component:**

```rust
#[with_css("examples/variables.scss")]
fn VariablesExample() -> Element {
    rsx! {
        div {
            button { 
                "data-scope": "{css}",
                class: "{css}_button", 
                "Primary" 
            }
            button { 
                "data-scope": "{css}",
                class: "{css}_button {css}_secondary", 
                "Secondary" 
            }
            div { 
                "data-scope": "{css}",
                class: "{css}_card", 
                "Card with consistent spacing" 
            }
        }
    }
}
```

---

### 2️⃣ SCSS Nesting

**SCSS File:** `examples/nesting.scss`

```scss
.navigation {
    background: #2c3e50;
    padding: 1rem;
    
    .nav-list {
        list-style: none;
        display: flex;
        gap: 2rem;
        
        .nav-item {
            position: relative;
            
            .nav-link {
                color: white;
                text-decoration: none;
                padding: 0.5rem 1rem;
                
                &:hover {
                    background: rgba(255, 255, 255, 0.1);
                    border-radius: 4px;
                }
                
                &.active {
                    background: #3498db;
                    border-radius: 4px;
                }
            }
        }
    }
}

// Parent selector (&)
.card {
    background: white;
    border: 1px solid #ddd;
    
    &:hover {
        box-shadow: 0 4px 8px rgba(0,0,0,0.1);
    }
    
    &--featured {
        border: 2px solid #3498db;
    }
    
    &__title {
        font-size: 1.5rem;
        margin-bottom: 0.5rem;
    }
    
    &__content {
        color: #666;
    }
}
```

**Usage:**

```rust
#[with_css("examples/nesting.scss")]
fn NestingExample() -> Element {
    rsx! {
        nav { 
            "data-scope": "{css}",
            class: "{css}_navigation",
            ul { 
                "data-scope": "{css}",
                class: "{css}_nav-list",
                li { 
                    "data-scope": "{css}",
                    class: "{css}_nav-item",
                    a { 
                        "data-scope": "{css}",
                        class: "{css}_nav-link {css}_active",
                        href: "#",
                        "Home"
                    }
                }
            }
        }
    }
}
```

---

### 3️⃣ SCSS Mixins

**SCSS File:** `examples/mixins.scss`

```scss
// Define reusable mixins
@mixin flex-center {
    display: flex;
    justify-content: center;
    align-items: center;
}

@mixin button-variant($bg-color, $text-color: white) {
    background: $bg-color;
    color: $text-color;
    border: none;
    padding: 0.75rem 1.5rem;
    border-radius: 6px;
    cursor: pointer;
    
    &:hover {
        background: darken($bg-color, 10%);
    }
    
    &:active {
        transform: scale(0.98);
    }
}

@mixin card-shadow($level: 1) {
    @if $level == 1 {
        box-shadow: 0 2px 4px rgba(0,0,0,0.1);
    } @else if $level == 2 {
        box-shadow: 0 4px 8px rgba(0,0,0,0.15);
    } @else {
        box-shadow: 0 8px 16px rgba(0,0,0,0.2);
    }
}

// Use mixins
.container {
    @include flex-center;
    min-height: 100vh;
}

.btn-primary {
    @include button-variant(#3498db);
}

.btn-success {
    @include button-variant(#2ecc71);
}

.btn-danger {
    @include button-variant(#e74c3c);
}

.card {
    @include card-shadow(1);
    padding: 2rem;
    
    &:hover {
        @include card-shadow(2);
    }
}

.modal {
    @include flex-center;
    @include card-shadow(3);
}
```

**Usage:**

```rust
#[with_css("examples/mixins.scss")]
fn MixinsExample() -> Element {
    rsx! {
        div { 
            "data-scope": "{css}",
            class: "{css}_container",
            button { 
                "data-scope": "{css}",
                class: "{css}_btn-primary", 
                "Primary" 
            }
            button { 
                "data-scope": "{css}",
                class: "{css}_btn-success", 
                "Success" 
            }
            div { 
                "data-scope": "{css}",
                class: "{css}_card", 
                "Shadowed Card" 
            }
        }
    }
}
```

---

### 4️⃣ SCSS Functions & Operations

**SCSS File:** `examples/functions.scss`

```scss
// Color functions
$base-color: #3498db;

.primary {
    background: $base-color;
    border: 1px solid darken($base-color, 10%);
}

.light {
    background: lighten($base-color, 30%);
}

.dark {
    background: darken($base-color, 20%);
}

.transparent {
    background: rgba($base-color, 0.5);
}

// Math operations
$base-spacing: 1rem;

.spacing-sm {
    padding: $base-spacing / 2;
}

.spacing-md {
    padding: $base-spacing;
}

.spacing-lg {
    padding: $base-spacing * 2;
}

.spacing-xl {
    padding: $base-spacing * 3;
}

// String interpolation
$image-path: "../images";

.background {
    background-image: url("#{$image-path}/hero.jpg");
}

// Custom functions (using built-in functions)
@function calculate-rem($pixels) {
    @return #{$pixels / 16}rem;
}

.text-small {
    font-size: calculate-rem(12);
}

.text-medium {
    font-size: calculate-rem(16);
}

.text-large {
    font-size: calculate-rem(24);
}
```

**Usage:**

```rust
#[with_css("examples/functions.scss")]
fn FunctionsExample() -> Element {
    rsx! {
        div { 
            "data-scope": "{css}",
            class: "{css}_primary", 
            "Primary Color" 
        }
        div { 
            "data-scope": "{css}",
            class: "{css}_spacing-lg", 
            "Large Spacing" 
        }
        p { 
            "data-scope": "{css}",
            class: "{css}_text-large", 
            "Large Text" 
        }
    }
}
```

---

### 5️⃣ Advanced SCSS Features

**SCSS File:** `examples/advanced.scss`

```scss
// Lists and loops
$sizes: (sm: 0.75rem, md: 1rem, lg: 1.5rem, xl: 2rem);

@each $name, $size in $sizes {
    .text-#{$name} {
        font-size: $size;
    }
}

// Conditionals
$theme: light;

.container {
    @if $theme == light {
        background: white;
        color: black;
    } @else {
        background: black;
        color: white;
    }
}

// Maps
$colors: (
    primary: #3498db,
    success: #2ecc71,
    danger: #e74c3c,
    warning: #f39c12
);

@each $name, $color in $colors {
    .bg-#{$name} {
        background: $color;
    }
    
    .text-#{$name} {
        color: $color;
    }
    
    .border-#{$name} {
        border: 2px solid $color;
    }
}

// Extend
%button-base {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-weight: 600;
}

.btn {
    @extend %button-base;
}

.btn-large {
    @extend %button-base;
    padding: 1rem 2rem;
    font-size: 1.25rem;
}

// Nested media queries
.responsive-box {
    padding: 1rem;
    
    @media (min-width: 768px) {
        padding: 2rem;
    }
    
    @media (min-width: 1024px) {
        padding: 3rem;
    }
}
```

---

### 6️⃣ Inline SCSS

No file needed! Write SCSS directly in your components:

```rust
fn InlineScssExample() -> Element {
    let css = scoped_style!("
        $primary: #667eea;
        $secondary: #764ba2;
        
        .gradient-box {
            background: linear-gradient(135deg, $primary 0%, $secondary 100%);
            color: white;
            padding: 2rem;
            border-radius: 10px;
            
            .title {
                font-size: 2rem;
                margin-bottom: 1rem;
                
                &::before {
                    content: '✨ ';
                }
            }
            
            .content {
                opacity: 0.9;
            }
        }
    ");
    
    rsx! {
        div { 
            "data-scope": "{css}",
            class: "{css}_gradient-box",
            div { 
                "data-scope": "{css}",
                class: "{css}_title", 
                "Inline SCSS!" 
            }
            p { 
                "data-scope": "{css}",
                class: "{css}_content", 
                "Compiled at build time" 
            }
        }
    }
}
```

---

## 📖 CSS Examples (Still Supported!)

### 1️⃣ Class Selectors (`.class`)

**CSS File:** `examples/class_selectors.css`

```css
.card {
    background: white;
    border-radius: 12px;
    padding: 24px;
}

.card-title {
    font-size: 24px;
    font-weight: bold;
}

.btn-primary {
    background: #4299e1;
    color: white;
}
```

**Usage in Component:**

```rust
#[with_css("examples/class_selectors.css")]
fn ClassSelectorExample() -> Element {
    rsx! {
        div { 
            "data-scope": "{css}",
            class: "{css}_card",
            h3 { 
                "data-scope": "{css}",
                class: "{css}_card-title", 
                "Title" 
            }
            button { 
                "data-scope": "{css}",
                class: "{css}_btn {css}_btn-primary", 
                "Click" 
            }
        }
    }
}
```

---

## ⚠️ Important Notes

### 1. Element Selectors Require `data-scope`

```rust
// ❌ Wrong - won't work
div { "Content" }

// ✅ Correct
div { 
    "data-scope": "{css}",
    "Content"
}
```

### 2. SCSS Files Auto-Detected

```rust
// Automatically compiles SCSS to CSS
#[with_css("button.scss")]  // ✅ SCSS file
#[with_css("button.css")]   // ✅ CSS file
```

### 3. SCSS Compilation is Compile-Time

```rust
// All SCSS compilation happens at build time
// Zero runtime overhead!
let css = scoped_style!("button.scss");
```

### 4. Feature Flag for SCSS

```toml
# Cargo.toml

# SCSS enabled by default
[dependencies]
dioxus_style = "0.3.0"

# Explicitly enable SCSS
dioxus_style = { version = "0.3.0", features = ["scss"] }

# Disable SCSS (smaller dependencies)
dioxus_style = { version = "0.3.0", default-features = false }
```

---

## 🎯 Best Practices

### CSS vs SCSS: When to Use What?

**Use Plain CSS when:**
- Simple, one-off styles
- No variables or nesting needed
- Maximum build speed is critical
- Smaller dependency tree desired

**Use SCSS when:**
- Using design systems with variables
- Need style reusability (mixins)
- Complex component hierarchies (nesting)
- Working with themes or color schemes
- Want better organization and maintainability

### SCSS Best Practices

1. **Use Variables for Design Tokens**
   ```scss
   $primary: #3498db;
   $spacing: 1rem;
   ```

2. **Leverage Nesting (But Don't Overdo It)**
   ```scss
   // Good: 2-3 levels
   .card {
       .title { }
       .content { }
   }
   
   // Bad: Too deep (hard to debug)
   .nav { .list { .item { .link { .icon { } } } } }
   ```

3. **Create Reusable Mixins**
   ```scss
   @mixin flex-center {
       display: flex;
       justify-content: center;
       align-items: center;
   }
   ```

4. **Use Color Functions**
   ```scss
   .button {
       background: $primary;
       &:hover { background: darken($primary, 10%); }
   }
   ```

---

## 🐛 Troubleshooting

### Issue: SCSS compilation error

**Error:**
```
error: SCSS compilation error in 'button.scss': Undefined variable
```

**Solution:**
Check that all SCSS variables are defined before use:
```scss
// ❌ Wrong
.button { color: $undefined; }

// ✅ Correct
$primary: blue;
.button { color: $primary; }
```

### Issue: SCSS feature not available

**Error:**
```
error: SCSS support is not enabled
```

**Solution:**
Enable SCSS feature in Cargo.toml:
```toml
[dependencies]
dioxus_style = { version = "0.3.0", features = ["scss"] }
```

### Issue: Styles not applying

**Check:**
1. ✅ CSS/SCSS file path is correct
2. ✅ Using `{css}_` prefix for classes
3. ✅ Using `{css}_` prefix for IDs
4. ✅ Added `data-scope` for element selectors
5. ✅ `inject_styles()` in root App component
6. ✅ SCSS feature enabled if using `.scss` files

---

## 📚 More Resources

- **Dioxus Docs:** https://dioxuslabs.com
- **SCSS Guide:** https://sass-lang.com/guide
- **CSS Selectors:** https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors

---

## 🎉 Ready to Style with SCSS!

Ab aapke paas **CSS aur SCSS dono** ke complete examples hain. Variables, nesting, mixins - sab kuch use karo aur amazing styles banao! 🚀✨