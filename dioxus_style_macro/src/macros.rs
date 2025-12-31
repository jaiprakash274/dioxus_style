//! Procedural macro implementations for scoped styling with SCSS support.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use crate::hash::generate_hash;
use crate::style_parser::parse_and_scope;
use crate::scss_compiler::{compile_scss_to_css, is_scss_file};

/// Implementation of the `scoped_style!` macro with SCSS support.
pub fn scoped_style_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let css_content = input_str.value();

    // Determine if this is a file path or inline CSS
    if is_likely_file_path(&css_content) {
        let file_path = css_content.clone();

        // Try to find the CSS/SCSS file in common locations
        let possible_paths = [
            file_path.clone(),              // As specified
            format!("../{}", file_path),    // Parent directory
            format!("../../{}", file_path), // Two levels up
            format!("src/{}", file_path),   // In src
        ];

        let (_actual_path, raw_file_content) = match possible_paths.iter().find_map(|path| {
            std::fs::read_to_string(path)
                .ok()
                .map(|content| (path.clone(), content))
        }) {
            Some(result) => result,
            None => {
                let error = format!(
                    "Failed to find CSS/SCSS file '{}'. Tried paths:\n{}",
                    file_path,
                    possible_paths
                        .iter()
                        .map(|p| format!("  - {}", p))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
                return syn::Error::new(input_str.span(), error)
                    .to_compile_error()
                    .into();
            }
        };

        let css_content = if is_scss_file(&file_path) {
            let minify = cfg!(not(debug_assertions));
            match compile_scss_to_css(&raw_file_content, Some(&file_path), minify) {
                Ok(css) => css,
                Err(e) => {
                    return syn::Error::new(input_str.span(), e)
                        .to_compile_error()
                        .into();
                }
            }
        } else {
            raw_file_content
        };

        // Generate hash with both file path and content
        let scope_lit = generate_hash(&css_content, Some(&file_path));
        
        // Scope the CSS (now already compiled from SCSS if needed)
        let minify = cfg!(not(debug_assertions));
        let scoped = parse_and_scope(&css_content, &scope_lit, minify);
        let scoped_css = scoped.scoped;

        // Use the original file_path for include_str! (not actual_path)
        let include_path = &file_path;

        let expanded = quote! {
            {
                use ::std::sync::OnceLock;
                static STYLE_INSTANCE: OnceLock<::dioxus_style::ScopedStyle> = OnceLock::new();
                
                *STYLE_INSTANCE.get_or_init(|| {
                    // include_str! tracks the file for hot-reload rebuilds
                    let _css_tracker = include_str!(#include_path);
                    ::dioxus_style::ScopedStyle::new(#scope_lit, #scoped_css)
                })
            }
        };

        TokenStream::from(expanded)
    } else {
        // Inline CSS/SCSS - process at compile time
        let css_content = if looks_like_scss(&css_content) {
            let minify = cfg!(not(debug_assertions));
            match compile_scss_to_css(&css_content, None, minify) {
                Ok(css) => css,
                Err(e) => {
                    return syn::Error::new(input_str.span(), e)
                        .to_compile_error()
                        .into();
                }
            }
        } else {
            css_content
        };

        let scope = generate_hash(&css_content, None);
        let minify = cfg!(not(debug_assertions));
        let scoped = parse_and_scope(&css_content, &scope, minify);
        let scoped_css = scoped.scoped;

        let expanded = quote! {
            {
                use ::std::sync::OnceLock;
                static STYLE_INSTANCE: OnceLock<::dioxus_style::ScopedStyle> = OnceLock::new();
                
                // Pass static string literals directly - no allocation!
                *STYLE_INSTANCE.get_or_init(|| {
                    ::dioxus_style::ScopedStyle::new(#scope, #scoped_css)
                })
            }
        };

        TokenStream::from(expanded)
    }
}

/// Implementation of the `css!` macro for inline styles.
pub fn css_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let css_content = input_str.value();

    let scope = generate_hash(&css_content, None);
    let wrapped_css = format!(".{} {{ {} }}", scope, css_content);

    let minify = cfg!(not(debug_assertions));
    let final_css = if minify {
        crate::style_parser::parse_and_scope(&wrapped_css, &scope, true).scoped
    } else {
        wrapped_css
    };

    let expanded = quote! {
        {
            use ::std::sync::OnceLock;
            static STYLE_INSTANCE: OnceLock<::dioxus_style::ScopedStyle> = OnceLock::new();
            
            // Pass static string literals directly - no allocation!
            *STYLE_INSTANCE.get_or_init(|| {
                ::dioxus_style::ScopedStyle::new(#scope, #final_css)
            })
        }
    };

    TokenStream::from(expanded)
}

/// Checks if a string looks like a file path.
fn is_likely_file_path(s: &str) -> bool {
    s.ends_with(".css") || s.ends_with(".scss") || s.ends_with(".sass") 
        || s.contains('/') || s.contains('\\')
}

/// Checks if inline content looks like SCSS (has SCSS-specific syntax).
fn looks_like_scss(content: &str) -> bool {
    // Check for common SCSS features
    content.contains('$')           // Variables
        || content.contains("@mixin")   // Mixins
        || content.contains("@include") // Includes
        || content.contains("@import")  // Imports
        || content.contains("@use")     // Use statements
        || content.contains("@extend")  // Extends
        || content.contains('&')        // Parent selector (SCSS-only)
        || has_nesting(content)         // Nesting
}

/// Detects nested selectors in SCSS content.
/// Returns true if there's a selector with `{` inside another selector block.
fn has_nesting(content: &str) -> bool {
    let mut brace_depth: u32 = 0;
    let mut chars = content.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '{' => {
                brace_depth += 1;
                // If we're already inside a block and see another {, that's nesting
                if brace_depth > 1 {
                    return true;
                }
            }
            '}' => {
                brace_depth = brace_depth.saturating_sub(1);
            }
            // Skip string literals
            '"' | '\'' => {
                let quote = ch;
                while let Some(c) = chars.next() {
                    if c == quote {
                        break;
                    }
                    if c == '\\' {
                        chars.next(); // Skip escaped character
                    }
                }
            }
            // Skip comments
            '/' => {
                if chars.peek() == Some(&'*') {
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            break;
                        }
                    }
                } else if chars.peek() == Some(&'/') {
                    // Line comment
                    while let Some(c) = chars.next() {
                        if c == '\n' {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_likely_file_path() {
        assert!(is_likely_file_path("button.css"));
        assert!(is_likely_file_path("button.scss"));
        assert!(is_likely_file_path("button.sass"));
        assert!(is_likely_file_path("styles/button.css"));
        assert!(is_likely_file_path("./button.scss"));

        assert!(!is_likely_file_path(".button { color: red; }"));
        assert!(!is_likely_file_path("color: red; font-size: 16px;"));
    }

    #[test]
    fn test_looks_like_scss() {
        // Should detect SCSS
        assert!(looks_like_scss("$primary: blue;"));
        assert!(looks_like_scss("@mixin flex { display: flex; }"));
        assert!(looks_like_scss("@include flex;"));
        assert!(looks_like_scss(".parent { .child { } }"));
        assert!(looks_like_scss("&:hover { }"));

        // Should NOT detect SCSS
        assert!(!looks_like_scss(".button { color: red; }"));
        assert!(!looks_like_scss("div { margin: 0; }"));
    }

    #[test]
    fn test_has_nesting() {
        let nested = r#"
            .parent {
                color: red;
                .child {
                    color: blue;
                }
            }
        "#;
        assert!(has_nesting(nested));

        let not_nested = r#"
            .button {
                color: red;
                padding: 10px;
            }
        "#;
        assert!(!has_nesting(not_nested));
    }
}