# 🎨 Dioxus Style - Complete Selector Examples

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
│   └── mixed_complex.css                # Everything combined
└── Cargo.toml
```

---

## 🚀 Quick Start

```bash
# Run the complete examples
cargo run

# The app will show all selector types in action!
```

---

## 📋 Selector Support Table

| Selector Type | Input | Output | Status |
|--------------|-------|--------|--------|
| **Class** | `.button` | `.sc_abc.button` | ✅ Full Support |
| **ID** | `#header` | `#sc_abc_header` | ✅ Full Support |
| **Element** | `div` | `div[data-scope="sc_abc"]` | ✅ Full Support |
| **Pseudo-class** | `.btn:hover` | `.sc_abc.btn:hover` | ✅ Full Support |
| **Child** | `.parent > .child` | `.sc_abc.parent > .sc_abc.child` | ✅ Full Support |
| **Adjacent** | `.card + .card` | `.sc_abc.card + .sc_abc.card` | ✅ Full Support |
| **Sibling** | `.box ~ .box` | `.sc_abc.box ~ .sc_abc.box` | ✅ Full Support |
| **Descendant** | `.parent .child` | `.sc_abc.parent .sc_abc.child` | ✅ Full Support |
| **Multiple** | `.a, .b, #c` | `.sc_abc.a, .sc_abc.b, #sc_abc_c` | ✅ Full Support |

---

## 📖 Example Breakdown

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
        div { class: "{css}_card",
            h3 { class: "{css}_card-title", "Title" }
            button { class: "{css}_btn {css}_btn-primary", "Click" }
        }
    }
}
```

**Scoped Output:**
```css
.sc_abc123.card { background: white; ... }
.sc_abc123.card-title { font-size: 24px; ... }
.sc_abc123.btn-primary { background: #4299e1; ... }
```

---

### 2️⃣ ID Selectors (`#id`)

**CSS File:** `examples/id_selectors.css`

```css
#header {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 20px;
}

#main-content {
    border-left: 4px solid #4299e1;
}
```

**Usage:**

```rust
#[with_css("examples/id_selectors.css")]
fn IdSelectorExample() -> Element {
    rsx! {
        div { id: "{css}_header", "Header" }
        div { id: "{css}_main-content", "Content" }
    }
}
```

**Scoped Output:**
```css
#sc_abc123_header { background: linear-gradient(...); }
#sc_abc123_main-content { border-left: 4px solid #4299e1; }
```

---

### 3️⃣ Element Selectors (`div`, `span`, `p`)

**CSS File:** `examples/element_selectors.css`

```css
div {
    background: #edf2f7;
    padding: 15px;
}

p {
    color: #2c5282;
    line-height: 1.5;
}

span {
    background: #fbd38d;
    padding: 4px 8px;
}
```

**Usage (⚠️ Important: Add `data-scope`):**

```rust
#[with_css("examples/element_selectors.css")]
fn ElementSelectorExample() -> Element {
    rsx! {
        div { 
            "data-scope": "{css.scope()}",  // ✅ Required!
            "Styled div"
            
            p { 
                "data-scope": "{css.scope()}",
                "Styled paragraph"
            }
        }
    }
}
```

**Scoped Output:**
```css
div[data-scope="sc_abc123"] { background: #edf2f7; ... }
p[data-scope="sc_abc123"] { color: #2c5282; ... }
span[data-scope="sc_abc123"] { background: #fbd38d; ... }
```

---

### 4️⃣ Complex Selectors (Combinators)

**CSS File:** `examples/complex_selectors.css`

```css
/* Child combinator */
.parent > .child {
    background: #81e6d9;
}

/* Adjacent sibling */
.card + .card {
    margin-top: 20px;
}

/* Descendant */
.container .item {
    background: #feb2b2;
}

/* General sibling */
.box ~ .box {
    border-left: 4px solid #ecc94b;
}
```

**Usage:**

```rust
#[with_css("examples/complex_selectors.css")]
fn ComplexSelectorExample() -> Element {
    rsx! {
        // Child: parent > child
        div { class: "{css}_parent",
            div { class: "{css}_child", "Direct child" }
        }
        
        // Adjacent sibling: .card + .card
        div { class: "{css}_card", "Card 1" }
        div { class: "{css}_card", "Card 2 (has margin)" }
        
        // Descendant: .container .item
        div { class: "{css}_container",
            div { class: "{css}_item", "Nested" }
        }
    }
}
```

**Scoped Output:**
```css
.sc_abc123.parent > .sc_abc123.child { ... }
.sc_abc123.card + .sc_abc123.card { ... }
.sc_abc123.container .sc_abc123.item { ... }
.sc_abc123.box ~ .sc_abc123.box { ... }
```

---

### 5️⃣ Pseudo-classes (`:hover`, `:focus`, `:active`)

**CSS File:** `examples/pseudo_classes.css`

```css
.hover-btn {
    background: #9f7aea;
}

.hover-btn:hover {
    background: #7c3aed;
    transform: scale(1.05);
}

.focus-input:focus {
    border-color: #4299e1;
    box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.2);
}

.link:visited {
    color: #805ad5;
}
```

**Usage:**

```rust
#[with_css("examples/pseudo_classes.css")]
fn PseudoClassExample() -> Element {
    rsx! {
        button { class: "{css}_hover-btn", "Hover me!" }
        
        input { 
            class: "{css}_focus-input",
            placeholder: "Focus me"
        }
        
        a { 
            class: "{css}_link",
            href: "#",
            "Link"
        }
    }
}
```

---

### 6️⃣ Multiple Selectors (Comma-separated)

**CSS File:** `examples/multiple_selectors.css`

```css
/* Same style for multiple selectors */
.btn,
.button,
.action {
    background: #ed8936;
    padding: 10px 20px;
}

#header,
#footer {
    background: #2d3748;
    color: white;
}
```

**Scoped Output:**
```css
.sc_abc123.btn, .sc_abc123.button, .sc_abc123.action { ... }
#sc_abc123_header, #sc_abc123_footer { ... }
```

---

### 7️⃣ Mixed Complex Example

**CSS File:** `examples/mixed_complex.css`

```css
/* Super complex selector */
div.container > .item#special {
    background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
    border: 3px solid #e53e3e;
}

/* Hover with combinators */
.card:hover > .icon + span {
    color: #4299e1;
    font-weight: bold;
}

/* Element with class and pseudo */
ul > li.active {
    background: #4299e1;
    color: white;
}
```

**Usage:**

```rust
#[with_css("examples/mixed_complex.css")]
fn MixedComplexExample() -> Element {
    rsx! {
        div { 
            "data-scope": "{css.scope()}",
            class: "{css}_container",
            
            div { 
                class: "{css}_item",
                id: "{css}_special",
                "Special item!"
            }
        }
    }
}
```

---

### 8️⃣ Inline CSS (No file needed!)

```rust
fn InlineCssExample() -> Element {
    let style = css!("
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        color: white;
        padding: 20px;
        border-radius: 10px;
    ");
    
    rsx! {
        div { class: "{style}", "Inline styled!" }
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
    "data-scope": "{css.scope()}",
    "Content"
}
```

### 2. Class Names Need Scope Prefix

```rust
// ❌ Wrong
div { class: "button", "Click" }

// ✅ Correct
div { class: "{css}_button", "Click" }
```

### 3. ID Names Need Scope Prefix

```rust
// ❌ Wrong
div { id: "header", "Header" }

// ✅ Correct
div { id: "{css}_header", "Header" }
```

---

## 🎯 Best Practices

1. **Use Classes for Styling** (most common, most flexible)
2. **Use IDs for Unique Elements** (single instance per scope)
3. **Avoid Element Selectors** (require data-scope, less flexible)
4. **Leverage Pseudo-classes** (great for interactions)
5. **Combine Wisely** (`.parent > .child` is powerful!)

---

## 🐛 Troubleshooting

### Issue: Styles not applying

**Check:**
1. ✅ CSS file path is correct
2. ✅ Using `{css}_` prefix for classes
3. ✅ Using `{css}_` prefix for IDs
4. ✅ Added `data-scope` for element selectors
5. ✅ `inject_styles()` in root App component

### Issue: Element selector not working

```rust
// Add data-scope attribute!
div { 
    "data-scope": "{css.scope()}",
    // ...
}
```

---

## 📚 More Resources

- **Dioxus Docs:** https://dioxuslabs.com
- **CSS Selectors Reference:** https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Selectors

---

## 🎉 Ready to Style!

Ab aapke paas **sabhi selector types** ke complete examples hain. Copy-paste karo aur enjoy karo! 🚀