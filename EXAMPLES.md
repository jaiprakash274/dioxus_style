# dioxus_style Examples

Complete examples showing different ways to use dioxus_style.

## Table of Contents

1. [Basic Usage](#basic-usage)
2. [Component Styles](#component-styles)
3. [Inline Styles](#inline-styles)
4. [Complex Application](#complex-application)
5. [Best Practices](#best-practices)

---

## Basic Usage

### Simple Button Component

**button.css:**
```css
.btn {
    padding: 10px 20px;
    border: none;
    border-radius: 5px;
    background: #007bff;
    color: white;
    cursor: pointer;
    font-size: 16px;
    transition: background 0.3s;
}

.btn:hover {
    background: #0056b3;
}

.btn:active {
    transform: scale(0.98);
}

.btn-disabled {
    opacity: 0.5;
    cursor: not-allowed;
}
```

**button.rs:**
```rust
use dioxus::prelude::*;
use dioxus_style::with_css;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    text: String,
    disabled: bool,
    on_click: EventHandler<MouseEvent>,
}

#[with_css("button.css")]
pub fn Button(props: ButtonProps) -> Element {
    let disabled_class = if props.disabled { 
        format!(" {css}_btn-disabled") 
    } else { 
        String::new() 
    };
    
    rsx! {
        button {
            class: "{css}_btn{disabled_class}",
            disabled: props.disabled,
            onclick: move |evt| props.on_click.call(evt),
            "{props.text}"
        }
    }
}
```

**Usage:**
```rust
rsx! {
    Button {
        text: "Click Me",
        disabled: false,
        on_click: |_| println!("Clicked!")
    }
}
```

---

## Component Styles

### Card Component with Multiple Elements

**card.css:**
```css
.card {
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0,0,0,0.1);
    padding: 20px;
    margin: 10px;
}

.card-header {
    font-size: 24px;
    font-weight: bold;
    margin-bottom: 10px;
    color: #333;
}

.card-body {
    font-size: 16px;
    line-height: 1.6;
    color: #666;
}

.card-footer {
    margin-top: 15px;
    padding-top: 15px;
    border-top: 1px solid #eee;
    display: flex;
    justify-content: space-between;
}

.card:hover {
    box-shadow: 0 4px 12px rgba(0,0,0,0.15);
    transform: translateY(-2px);
    transition: all 0.3s;
}
```

**card.rs:**
```rust
use dioxus::prelude::*;
use dioxus_style::with_css;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    title: String,
    content: String,
    footer: Option<String>,
}

#[with_css("card.css")]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div { class: "{css}_card",
            div { class: "{css}_card-header",
                "{props.title}"
            }
            div { class: "{css}_card-body",
                "{props.content}"
            }
            if let Some(footer_text) = props.footer {
                div { class: "{css}_card-footer",
                    "{footer_text}"
                }
            }
        }
    }
}
```

---

## Inline Styles

### Using css! Macro for Dynamic Styles

```rust
use dioxus::prelude::*;
use dioxus_style::css;

#[component]
fn Badge(text: String, color: String) -> Element {
    let badge_style = css!("
        padding: 4px 8px;
        border-radius: 12px;
        font-size: 12px;
        font-weight: bold;
    ");
    
    rsx! {
        span {
            class: "{badge_style}",
            style: "background: {color}; color: white;",
            "{text}"
        }
    }
}

#[component]
fn StatusBadges() -> Element {
    rsx! {
        div {
            Badge { text: "Active", color: "#28a745" }
            Badge { text: "Pending", color: "#ffc107" }
            Badge { text: "Inactive", color: "#dc3545" }
        }
    }
}
```

---

## Complex Application

### Complete Todo App

**app.css:**
```css
.app {
    max-width: 600px;
    margin: 50px auto;
    font-family: 'Segoe UI', sans-serif;
}

.app-title {
    text-align: center;
    color: #333;
    margin-bottom: 30px;
}
```

**todo.css:**
```css
.todo-container {
    background: white;
    border-radius: 8px;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.todo-input-section {
    padding: 20px;
    border-bottom: 1px solid #eee;
    display: flex;
    gap: 10px;
}

.todo-input {
    flex: 1;
    padding: 10px;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 16px;
}

.todo-add-btn {
    padding: 10px 20px;
    background: #007bff;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
}

.todo-add-btn:hover {
    background: #0056b3;
}

.todo-list {
    list-style: none;
    padding: 0;
    margin: 0;
}

.todo-item {
    padding: 15px 20px;
    border-bottom: 1px solid #eee;
    display: flex;
    align-items: center;
    gap: 10px;
}

.todo-item:hover {
    background: #f8f9fa;
}

.todo-checkbox {
    width: 20px;
    height: 20px;
    cursor: pointer;
}

.todo-text {
    flex: 1;
    font-size: 16px;
}

.todo-text-completed {
    text-decoration: line-through;
    color: #999;
}

.todo-delete-btn {
    padding: 5px 10px;
    background: #dc3545;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 14px;
}

.todo-delete-btn:hover {
    background: #c82333;
}
```

**main.rs:**
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
        TodoApp {}
    }
}

#[derive(Clone, PartialEq)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

#[with_css("app.css")]
fn TodoApp() -> Element {
    let mut todos = use_signal(|| Vec::<Todo>::new());
    let mut input = use_signal(|| String::new());
    let mut next_id = use_signal(|| 0);
    
    let add_todo = move |_| {
        if !input().is_empty() {
            let new_todo = Todo {
                id: next_id(),
                text: input(),
                completed: false,
            };
            todos.write().push(new_todo);
            next_id += 1;
            input.set(String::new());
        }
    };
    
    rsx! {
        div { class: "{css}_app",
            h1 { class: "{css}_app-title", "My Todo List" }
            TodoContainer { 
                todos: todos,
                input: input,
                on_add: add_todo
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TodoContainerProps {
    todos: Signal<Vec<Todo>>,
    input: Signal<String>,
    on_add: EventHandler<MouseEvent>,
}

#[with_css("todo.css")]
fn TodoContainer(props: TodoContainerProps) -> Element {
    let todos = props.todos;
    let input = props.input;
    
    let toggle_todo = move |id: usize| {
        let mut todos_write = todos.write();
        if let Some(todo) = todos_write.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    };
    
    let delete_todo = move |id: usize| {
        todos.write().retain(|t| t.id != id);
    };
    
    rsx! {
        div { class: "{css}_todo-container",
            div { class: "{css}_todo-input-section",
                input {
                    class: "{css}_todo-input",
                    r#type: "text",
                    value: "{input}",
                    placeholder: "Add a new todo...",
                    oninput: move |evt| input.set(evt.value()),
                    onkeypress: move |evt| {
                        if evt.key() == Key::Enter {
                            props.on_add.call(MouseEvent::default());
                        }
                    }
                }
                button {
                    class: "{css}_todo-add-btn",
                    onclick: move |evt| props.on_add.call(evt),
                    "Add"
                }
            }
            
            ul { class: "{css}_todo-list",
                for todo in todos.read().iter() {
                    TodoItem {
                        todo: todo.clone(),
                        on_toggle: move |_| toggle_todo(todo.id),
                        on_delete: move |_| delete_todo(todo.id)
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TodoItemProps {
    todo: Todo,
    on_toggle: EventHandler<MouseEvent>,
    on_delete: EventHandler<MouseEvent>,
}

#[with_css("todo.css")]
fn TodoItem(props: TodoItemProps) -> Element {
    let text_class = if props.todo.completed {
        format!("{css}_todo-text {css}_todo-text-completed")
    } else {
        format!("{css}_todo-text")
    };
    
    rsx! {
        li { class: "{css}_todo-item",
            input {
                class: "{css}_todo-checkbox",
                r#type: "checkbox",
                checked: props.todo.completed,
                onchange: move |evt| props.on_toggle.call(MouseEvent::default())
            }
            span { class: "{text_class}",
                "{props.todo.text}"
            }
            button {
                class: "{css}_todo-delete-btn",
                onclick: move |evt| props.on_delete.call(evt),
                "Delete"
            }
        }
    }
}
```

---

## Best Practices

### 1. Component Organization

```
src/
├── components/
│   ├── button/
│   │   ├── mod.rs
│   │   └── button.css
│   ├── card/
│   │   ├── mod.rs
│   │   └── card.css
│   └── layout/
│       ├── mod.rs
│       └── layout.css
├── app.rs
└── main.rs
```

### 2. Naming Conventions

```css
/* Use BEM-like naming for clarity */
.component-name { }
.component-name__element { }
.component-name--modifier { }

/* Example */
.card { }
.card__header { }
.card__body { }
.card--featured { }
```

### 3. Style Injection Pattern

```rust
// ✅ Good: Inject once at root
#[component]
fn App() -> Element {
    rsx! {
        style { dangerous_inner_html: "{inject_styles()}" }
        Router::<Route> {}
    }
}

// ❌ Avoid: Multiple injections
#[component]
fn Component() -> Element {
    rsx! {
        style { dangerous_inner_html: "{inject_styles()}" }  // Don't do this
        div { "content" }
    }
}
```

### 4. CSS File Organization

```css
/* Group related styles */

/* Base styles */
.card {
    background: white;
    padding: 20px;
}

/* Element styles */
.card-header { }
.card-body { }
.card-footer { }

/* State modifiers */
.card:hover { }
.card--active { }
.card--disabled { }

/* Responsive */
@media (max-width: 768px) {
    .card {
        padding: 10px;
    }
}
```

### 5. Performance Tips

```rust
// ✅ Good: Reuse style instances
lazy_static! {
    static ref BUTTON_STYLE: ScopedStyle = scoped_style!("button.css");
}

// ✅ Good: Use with_css for automatic management
#[with_css("button.css")]
fn Button() -> Element { }

// ❌ Avoid: Creating styles in render
fn Button() -> Element {
    let css = scoped_style!("button.css");  // Already cached via lazy_static
    // ...
}
```

---

## Running Examples

```bash
# Clone the repository
git clone https://github.com/jaiprakash274/dioxus_style
cd dioxus_style

# Run examples (if you add an examples/ directory)
cargo run --example todo
cargo run --example button
```

## More Resources

- [Main Documentation](README.md)
- [API Reference](https://docs.rs/dioxus_style)
- [Dioxus Documentation](https://dioxuslabs.com)

---

**Have more example ideas? Contribute them!** See [CONTRIBUTING.md](CONTRIBUTING.md)