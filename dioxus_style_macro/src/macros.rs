//! Procedural macro implementations for scoped styling.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use crate::hash::generate_hash;
use crate::style_parser::parse_and_scope;

/// Implementation of the `scoped_style!` macro.
pub fn scoped_style_impl(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let css_content = input_str.value();

    // Determine if this is a file path or inline CSS
    if is_likely_file_path(&css_content) {
        let file_path = css_content.clone();

        // Try to find the CSS file in common locations
        let possible_paths = [
            file_path.clone(),              // As specified
            format!("../{}", file_path),    // Parent directory
            format!("../../{}", file_path), // Two levels up
            format!("src/{}", file_path),   // In src
        ];

        let (actual_path, css_file_content) = match possible_paths.iter().find_map(|path| {
            std::fs::read_to_string(path)
                .ok()
                .map(|content| (path.clone(), content))
        }) {
            Some(result) => result,
            None => {
                let error = format!(
                    "Failed to find CSS file '{}'. Tried paths:\n{}",
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

        eprintln!("✅ Found CSS file at: {}", actual_path);

        // Determine if we should minify
        let minify = cfg!(not(debug_assertions));

        // We need to read the file at compile time to generate proper hash
        // Generate hash with both file path and content
        let scope_lit = generate_hash(&css_file_content, Some(&file_path));
        let scoped = parse_and_scope(&css_file_content, &scope_lit, minify);
        let scoped_css = scoped.scoped;

        // Use the original file_path for include_str! (not actual_path)
        // because include_str! is relative to the caller's file location
        let include_path = &file_path;

        // Generate code that uses include_str! at compile time
        let expanded = quote! {
            {
                ::dioxus_style::lazy_static! {
                    static ref STYLE_INSTANCE: ::dioxus_style::ScopedStyle = {
                        // include_str! runs at compile time and tracks the file for rebuilds
                        // Use the original path as specified by user
                        let _css_tracker = include_str!(#include_path);

                        // Use pre-processed CSS
                        let scope = #scope_lit.to_string();
                        let css = #scoped_css.to_string();

                        eprintln!("🚀 [STATIC INIT] Loaded CSS from file: {}", #file_path);
                        eprintln!("🎯 [STATIC INIT] Scope: {}, CSS length: {}", scope, css.len());

                        ::dioxus_style::ScopedStyle::new(scope, css)
                    };
                }

                STYLE_INSTANCE.clone()
            }
        };

        TokenStream::from(expanded)
    } else {
        // Inline CSS - process at compile time as before
        eprintln!("📝 Using inline CSS (not a file path)");

        let scope = generate_hash(&css_content, None);
        let minify = cfg!(not(debug_assertions));
        let scoped = parse_and_scope(&css_content, &scope, minify);
        let scoped_css = scoped.scoped;

        let expanded = quote! {
            {
                ::dioxus_style::lazy_static! {
                    static ref STYLE_INSTANCE: ::dioxus_style::ScopedStyle = {
                        let scope = #scope.to_string();
                        let css = #scoped_css.to_string();
                        ::dioxus_style::ScopedStyle::new(scope, css)
                    };
                }

                STYLE_INSTANCE.clone()
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
            ::dioxus_style::lazy_static! {
                static ref STYLE_INSTANCE: ::dioxus_style::ScopedStyle = {
                    let scope = #scope.to_string();
                    let css = #final_css.to_string();
                    ::dioxus_style::ScopedStyle::new(scope, css)
                };
            }

            STYLE_INSTANCE.clone()
        }
    };

    TokenStream::from(expanded)
}

/// Checks if a string looks like a file path.
fn is_likely_file_path(s: &str) -> bool {
    s.ends_with(".css") || s.contains('/') || s.contains('\\')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_likely_file_path() {
        assert!(is_likely_file_path("button.css"));
        assert!(is_likely_file_path("styles/button.css"));
        assert!(is_likely_file_path("./button.css"));

        assert!(!is_likely_file_path(".button { color: red; }"));
        assert!(!is_likely_file_path("color: red; font-size: 16px;"));
    }
}
