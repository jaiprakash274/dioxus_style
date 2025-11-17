//! dioxus_style_macro/src/lib.rs
//! Removed quick_component macro

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

mod hash;
mod macros;
mod style_parser;

// ============================================
// CORE MACROS
// ============================================

#[proc_macro]
pub fn scoped_style(input: TokenStream) -> TokenStream {
    macros::scoped_style_impl(input)
}

#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    macros::css_impl(input)
}

// ============================================
// COMPONENT MACROS
// ============================================

/// Attribute macro with AUTOMATIC style injection!
///
/// # Usage
/// ```rust
/// #[with_css("card.css")]
/// fn Card() -> Element {
///     rsx! {
///         // ✅ No manual inject_styles() needed!
///         div { class: "{css}_card", "Hello" }
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn with_css(attr: TokenStream, item: TokenStream) -> TokenStream {
    let css_file = parse_macro_input!(attr as LitStr);
    let func = parse_macro_input!(item as ItemFn);

    let fn_name = &func.sig.ident;
    let fn_inputs = &func.sig.inputs;
    let fn_output = &func.sig.output;
    let fn_vis = &func.vis;
    let fn_body = &func.block;

    // Validate return type
    let has_element_return = match fn_output {
        syn::ReturnType::Type(_, ty) => {
            if let syn::Type::Path(type_path) = ty.as_ref() {
                type_path
                    .path
                    .segments
                    .last()
                    .map(|seg| seg.ident == "Element")
                    .unwrap_or(false)
            } else {
                false
            }
        }
        _ => false,
    };

    if !has_element_return {
        return syn::Error::new_spanned(
            fn_output,
            "#[with_css] can only be used on functions that return Element",
        )
        .to_compile_error()
        .into();
    }

    let expanded = quote! {
        #[::dioxus::prelude::component]
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            use ::dioxus::prelude::*;

            // Create scoped CSS variable
            let css = ::dioxus_style::scoped_style!(#css_file);

            // Auto-inject: Wrap user's rsx! to prepend <style> tag
            let user_element = { #fn_body };

            // Inject styles BEFORE rendering user content
            rsx! {
                style { dangerous_inner_html: "{::dioxus_style::inject_styles()}" }
                {user_element}
            }
        }
    };

    TokenStream::from(expanded)
}

/// Function-like macro version of with_css
///
/// # Usage
/// ```rust
/// component_with_css! {
///     css: "button.css",
///     fn Button() -> Element {
///         rsx! {
///             button { class: "{css}_btn", "Click" }
///         }
///     }
/// }
/// ```
#[proc_macro]
pub fn component_with_css(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let css_start = match input_str.find("css:") {
        Some(pos) => pos + 4,
        None => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected format: component_with_css! { css: \"file.css\", fn Component() -> Element { ... } }"
            )
            .to_compile_error()
            .into();
        }
    };

    let after_css = input_str[css_start..].trim();
    let quote_start = match after_css.find('"') {
        Some(pos) => pos,
        None => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "Expected CSS file path in quotes",
            )
            .to_compile_error()
            .into();
        }
    };

    let after_first_quote = &after_css[quote_start + 1..];
    let quote_end = match after_first_quote.find('"') {
        Some(pos) => pos,
        None => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "Unclosed CSS file path quote",
            )
            .to_compile_error()
            .into();
        }
    };

    let css_file = &after_first_quote[..quote_end];

    let fn_start = match input_str.find("fn ") {
        Some(pos) => pos,
        None => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                "No function definition found",
            )
            .to_compile_error()
            .into();
        }
    };

    let fn_code = &input_str[fn_start..];
    let fn_tokens: TokenStream = match fn_code.parse() {
        Ok(tokens) => tokens,
        Err(e) => {
            return syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("Failed to parse function: {}", e),
            )
            .to_compile_error()
            .into();
        }
    };

    let func = parse_macro_input!(fn_tokens as ItemFn);

    let fn_name = &func.sig.ident;
    let fn_inputs = &func.sig.inputs;
    let fn_output = &func.sig.output;
    let fn_vis = &func.vis;
    let fn_body = &func.block;

    let css_file_lit = LitStr::new(css_file, proc_macro2::Span::call_site());

    let expanded = quote! {
        #[::dioxus::prelude::component]
        #fn_vis fn #fn_name(#fn_inputs) #fn_output {
            use ::dioxus::prelude::*;
            let css = ::dioxus_style::scoped_style!(#css_file_lit);

            let user_element = { #fn_body };

            rsx! {
                style { dangerous_inner_html: "{::dioxus_style::inject_styles()}" }
                {user_element}
            }
        }
    };

    TokenStream::from(expanded)
}
