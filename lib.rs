//! dioxus_style/src/lib.rs
//! Scoped CSS styling for Dioxus

mod runtime_injector;

// Re-export core macros
pub use dioxus_style_macro::{
    component_with_css, // Function-like macro for components
    css,                // Utility-style inline CSS
    scoped_style,       // Main scoped CSS macro (file or inline)
    with_css,           // Attribute macro for components
};

// Export runtime components
pub use runtime_injector::{inject_styles, ScopedStyle, StyleRegistry, STYLE_REGISTRY};

// Re-export lazy_static for internal use
pub use lazy_static::lazy_static;
