//! CSS parsing and scoping utilities.
//!
//! Transforms CSS selectors by adding a unique scope prefix to prevent conflicts.

use std::collections::HashSet;

/// Represents parsed CSS with scoping applied.
#[allow(dead_code)]
pub struct ScopedCss {
    pub scoped: String,
    pub class_names: Vec<String>,
}

/// Parses and scopes CSS content with a unique prefix.
pub fn parse_and_scope(css: &str, scope: &str, minify: bool) -> ScopedCss {
    let mut class_names = HashSet::with_capacity(16);

    // Pre-allocate with estimate
    let mut scoped_css = String::with_capacity(css.len() + scope.len() * 10);

    let rules = parse_css_rules(css);

    for rule in rules {
        if let Some(scoped_rule) = scope_rule(&rule, scope, &mut class_names) {
            scoped_css.push_str(&scoped_rule);
            if !minify {
                scoped_css.push('\n');
            }
        }
    }

    if minify {
        scoped_css = minify_css(&scoped_css);
    }

    ScopedCss {
        scoped: scoped_css,
        class_names: class_names.into_iter().collect(),
    }
}

/// Parses CSS into individual rules using a state machine approach.
#[inline]
fn parse_css_rules(css: &str) -> Vec<String> {
    let mut rules = Vec::with_capacity(16);
    let mut current_rule = String::with_capacity(128);
    let mut brace_count = 0;
    let mut chars = css.chars().peekable();

    while let Some(ch) = chars.next() {
        // Fast path: skip comments
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next(); // consume '*'
                          // Skip until end of comment
            while let Some(ch) = chars.next() {
                if ch == '*' && chars.peek() == Some(&'/') {
                    chars.next(); // consume '/'
                    break;
                }
            }
            continue;
        }

        current_rule.push(ch);

        match ch {
            '{' => brace_count += 1,
            '}' => {
                brace_count -= 1;

                if brace_count == 0 {
                    let trimmed = current_rule.trim();
                    if !trimmed.is_empty() {
                        rules.push(trimmed.to_string());
                    }
                    current_rule.clear();
                }
            }
            _ => {}
        }
    }

    rules
}

/// Scopes a single CSS rule by prefixing selectors.
/// Handles at-rules (@keyframes, @media, etc.) specially.
#[inline]
fn scope_rule(rule: &str, scope: &str, class_names: &mut HashSet<String>) -> Option<String> {
    let trimmed = rule.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Handle at-rules (they start with @)
    if trimmed.starts_with('@') {
        return handle_at_rule(trimmed, scope, class_names);
    }

    // Find the opening brace
    let brace_pos = trimmed.find('{')?;

    let selector = trimmed[..brace_pos].trim();
    let rest = &trimmed[brace_pos + 1..];

    // Remove closing brace
    let declarations = if let Some(pos) = rest.rfind('}') {
        rest[..pos].trim()
    } else {
        rest.trim()
    };

    let scoped_selector = scope_selector(selector, scope, class_names);

    Some(format!("{} {{ {} }}", scoped_selector, declarations))
}

/// Handles CSS at-rules like @keyframes, @media, @supports, @font-face.
#[inline]
fn handle_at_rule(rule: &str, scope: &str, class_names: &mut HashSet<String>) -> Option<String> {
    // Extract the at-rule name
    let at_end = rule[1..].find(|c: char| c.is_whitespace() || c == '{')
        .map(|i| i + 1)
        .unwrap_or(rule.len());
    let at_name = &rule[1..at_end].to_lowercase();
    
    match at_name.as_str() {
        // These at-rules should NOT be scoped - pass through unchanged
        "keyframes" | "-webkit-keyframes" | "-moz-keyframes" | "font-face" => {
            Some(rule.to_string())
        }
        
        // These at-rules contain nested rules that SHOULD be scoped
        "media" | "supports" | "layer" | "container" => {
            // Find the opening brace
            if let Some(brace_pos) = rule.find('{') {
                let condition = &rule[..brace_pos + 1]; // Keep "@media (...) {"
                let rest = &rule[brace_pos + 1..];
                
                // Find matching closing brace
                if let Some(close_pos) = rest.rfind('}') {
                    let inner_css = &rest[..close_pos];
                    
                    // Parse and scope the inner CSS rules
                    let inner_rules = parse_css_rules(inner_css);
                    let mut scoped_inner = String::with_capacity(inner_css.len());
                    
                    for inner_rule in inner_rules {
                        if let Some(scoped) = scope_rule(&inner_rule, scope, class_names) {
                            scoped_inner.push_str(&scoped);
                            scoped_inner.push(' ');
                        }
                    }
                    
                    return Some(format!("{} {} }}", condition, scoped_inner.trim()));
                }
            }
            Some(rule.to_string()) // Fallback: return unchanged
        }
        
        // Unknown at-rules: pass through unchanged
        _ => Some(rule.to_string())
    }
}

/// Scopes a CSS selector by adding the unique prefix.
#[inline]
fn scope_selector(selector: &str, scope: &str, class_names: &mut HashSet<String>) -> String {
    if !selector.contains(',') {
        // Fast path: single selector
        return scope_single_selector(selector, scope, class_names);
    }

    // Handle multiple selectors (comma-separated)
    selector
        .split(',')
        .map(|s| scope_single_selector(s.trim(), scope, class_names))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Scopes a single selector (no commas).
/// Scopes: classes (.class), IDs (#id), and elements (div, span, etc.)
/// Also handles functional pseudo-classes like :not(), :has(), :is(), :where()
#[inline]
fn scope_single_selector(selector: &str, scope: &str, class_names: &mut HashSet<String>) -> String {
    let mut result = String::with_capacity(selector.len() + scope.len() * 4);
    let mut chars = selector.chars().peekable();
    let mut at_start = true; // Track if we're at the start of a selector component

    while let Some(ch) = chars.next() {
        match ch {
            // Handle class selectors: .class → .scope_class
            '.' => {
                let mut class_name = String::with_capacity(16);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '-' || next_ch == '_' {
                        class_name.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if !class_name.is_empty() {
                    class_names.insert(class_name.clone());
                    result.push('.');
                    result.push_str(scope);
                    result.push('_');
                    result.push_str(&class_name);
                } else {
                    // Bug #1 fix: Preserve malformed empty class selector
                    result.push('.');
                }
                at_start = false;
            }

            // Handle ID selectors: #id → #scope_id
            '#' => {
                let mut id_name = String::with_capacity(16);
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '-' || next_ch == '_' {
                        id_name.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if !id_name.is_empty() {
                    result.push('#');
                    result.push_str(scope);
                    result.push('_');
                    result.push_str(&id_name);
                } else {
                    // Preserve malformed empty ID selector
                    result.push('#');
                }
                at_start = false;
            }

            // Handle combinators: reset the "at_start" flag
            ' ' | '>' | '+' | '~' => {
                result.push(ch);
                // Skip extra whitespace
                while let Some(&next_ch) = chars.peek() {
                    if next_ch == ' ' {
                        chars.next();
                    } else {
                        break;
                    }
                }
                at_start = true; // Next token is a new selector component
            }

            // Handle pseudo-classes and pseudo-elements
            ':' => {
                result.push(ch);
                
                // Collect pseudo-class/element name
                let mut pseudo_name = String::new();
                
                // Check for :: (pseudo-element)
                if chars.peek() == Some(&':') {
                    result.push(chars.next().unwrap());
                }
                
                // Collect the pseudo name
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '-' {
                        pseudo_name.push(next_ch);
                        result.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                
                // Bug #2 fix: Handle functional pseudo-classes that need scoping
                // :not(), :has(), :is(), :where() contain selectors that should be scoped
                // Optimization #2: Use eq_ignore_ascii_case to avoid allocation
                let is_functional = ["not", "has", "is", "where", "matches"]
                    .iter()
                    .any(|&p| pseudo_name.eq_ignore_ascii_case(p));
                if is_functional {
                    if chars.peek() == Some(&'(') {
                        result.push(chars.next().unwrap()); // Push '('
                        
                        // Collect content inside parentheses
                        let mut inner_content = String::new();
                        let mut paren_depth = 1;
                        
                        while let Some(c) = chars.next() {
                            match c {
                                '(' => {
                                    paren_depth += 1;
                                    inner_content.push(c);
                                }
                                ')' => {
                                    paren_depth -= 1;
                                    if paren_depth == 0 {
                                        break;
                                    }
                                    inner_content.push(c);
                                }
                                _ => inner_content.push(c),
                            }
                        }
                        
                        // Recursively scope the inner selector(s)
                        let scoped_inner = scope_selector(&inner_content, scope, class_names);
                        result.push_str(&scoped_inner);
                        result.push(')');
                    }
                }
                
                at_start = false;
            }

            // Handle attribute selectors with proper quote handling (Bug #3 fix)
            '[' => {
                result.push(ch);
                let mut in_quote: Option<char> = None;
                
                while let Some(next_ch) = chars.next() {
                    result.push(next_ch);
                    
                    match next_ch {
                        '"' | '\'' => {
                            if let Some(quote_char) = in_quote {
                                if quote_char == next_ch {
                                    in_quote = None; // Closing quote
                                }
                            } else {
                                in_quote = Some(next_ch); // Opening quote
                            }
                        }
                        '\\' => {
                            // Handle escape sequences inside quotes
                            if in_quote.is_some() {
                                if let Some(escaped) = chars.next() {
                                    result.push(escaped);
                                }
                            }
                        }
                        ']' => {
                            if in_quote.is_none() {
                                break; // Only break on ] if not inside quotes
                            }
                        }
                        _ => {}
                    }
                }
                at_start = false;
            }

            // Handle universal selector: * (do NOT scope - it's already global)
            '*' => {
                result.push(ch);
                at_start = false;
            }

            // Handle element selectors: div → div[data-scope="scope"]
            ch if ch.is_alphabetic() && at_start => {
                let mut element_name = String::from(ch);
                
                // Collect full element name
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '-' {
                        element_name.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Bug #4 fix: Don't scope :root or :host elements (they're global)
                // Note: These are pseudo-classes, not elements, but we handle edge cases
                if element_name == "root" || element_name == "host" {
                    result.push_str(&element_name);
                } else {
                    // Scope the element with data attribute
                    result.push_str(&element_name);
                    result.push_str("[data-scope=\"");
                    result.push_str(scope);
                    result.push_str("\"]");
                }
                
                at_start = false;
            }

            // Default: pass through
            _ => {
                result.push(ch);
                at_start = false;
            }
        }
    }

    result
}

/// Minifies CSS by removing whitespace and comments.
#[inline]
fn minify_css(css: &str) -> String {
    let mut result = String::with_capacity(css.len() / 2);
    let mut chars = css.chars().peekable();
    let mut last_was_space = false;

    while let Some(ch) = chars.next() {
        // Skip comments
        if ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            while let Some(ch) = chars.next() {
                if ch == '*' && chars.peek() == Some(&'/') {
                    chars.next();
                    break;
                }
            }
            continue;
        }

        if ch.is_whitespace() {
            if !last_was_space && !result.is_empty() {
                if let Some(last_ch) = result.chars().last() {
                    if !matches!(last_ch, '{' | '}' | ':' | ';' | ',') {
                        result.push(' ');
                        last_was_space = true;
                    }
                }
            }
        } else {
            result.push(ch);
            last_was_space = false;
        }
    }

    result.shrink_to_fit();
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_selector_scoping() {
        let css = ".button { color: red; }";
        let scoped = parse_and_scope(css, "sc_abc", false);
        // v0.6.0: Changed from .sc_abc.button to .sc_abc_button
        assert!(scoped.scoped.contains(".sc_abc_button"));
    }

    #[test]
    fn test_id_selector_scoping() {
        let css = "#header { color: blue; }";
        let scoped = parse_and_scope(css, "sc_abc", false);
        assert!(scoped.scoped.contains("#sc_abc_header"));
    }

    #[test]
    fn test_element_selector_scoping() {
        let css = "div { margin: 10px; }";
        let scoped = parse_and_scope(css, "sc_abc", false);
        assert!(scoped.scoped.contains("div[data-scope=\"sc_abc\"]"));
    }


    #[test]
    fn test_complex_selector_mixed() {
        let css = "div.container > .item + #special { color: green; }";
        let scoped = parse_and_scope(css, "sc_xyz", false);
        
        // v0.6.0: Updated format for all selector types
        assert!(scoped.scoped.contains("div[data-scope=\"sc_xyz\"].sc_xyz_container"));
        assert!(scoped.scoped.contains(".sc_xyz_item"));
        assert!(scoped.scoped.contains("#sc_xyz_special"));
    }


    #[test]
    fn test_pseudo_classes() {
        let css = ".button:hover { background: blue; }";
        let scoped = parse_and_scope(css, "sc_abc", false);
        // v0.6.0: Changed format
        assert!(scoped.scoped.contains(".sc_abc_button:hover"));
    }

    #[test]
    fn test_multiple_selectors() {
        let css = ".btn, .button, #submit { color: red; }";
        let scoped = parse_and_scope(css, "sc_xyz", false);
        
        // v0.6.0: Changed format
        assert!(scoped.scoped.contains(".sc_xyz_btn"));
        assert!(scoped.scoped.contains(".sc_xyz_button"));
        assert!(scoped.scoped.contains("#sc_xyz_submit"));
    }

    #[test]
    fn test_no_extra_braces() {
        let css = ".box { color: red; }";
        let scoped = parse_and_scope(css, "sc_test", false);

        let open_count = scoped.scoped.matches('{').count();
        let close_count = scoped.scoped.matches('}').count();

        assert_eq!(open_count, close_count);
    }

    #[test]
    fn test_minify() {
        let css = r#"
            .button {
                color: red;
            }
        "#;

        let scoped = parse_and_scope(css, "sc_test", true);
        assert!(scoped.scoped.len() < css.len());
    }

    #[test]
    fn test_comments_removed() {
        let css = r#"
            /* Comment */
            .button {
                color: red; /* inline */
            }
        "#;

        let scoped = parse_and_scope(css, "sc_test", true);

        assert!(!scoped.scoped.contains("/*"));
        assert!(!scoped.scoped.contains("*/"));
    }

    #[test]
    fn test_attribute_selectors() {
        let css = "input[type=\"text\"] { border: 1px solid; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert!(scoped.scoped.contains("input[data-scope=\"sc_test\"][type=\"text\"]"));
    }

    #[test]
    fn test_descendant_combinator() {
        let css = ".parent .child { color: blue; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        // v0.6.0: Changed format
        assert!(scoped.scoped.contains(".sc_test_parent .sc_test_child"));
    }

    // Additional tests for v0.2.0 features

    #[test]
    fn test_element_with_class() {
        let css = "div.container { padding: 20px; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert!(scoped.scoped.contains("div[data-scope=\"sc_test\"].sc_test_container"));
    }

    #[test]
    fn test_element_with_id() {
        let css = "section#main { margin: auto; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert!(scoped.scoped.contains("section[data-scope=\"sc_test\"]#sc_test_main"));
    }

    #[test]
    fn test_multiple_elements() {
        let css = "div, span, p { margin: 0; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert!(scoped.scoped.contains("div[data-scope=\"sc_test\"]"));
        assert!(scoped.scoped.contains("span[data-scope=\"sc_test\"]"));
        assert!(scoped.scoped.contains("p[data-scope=\"sc_test\"]"));
    }


    #[test]
    fn test_pseudo_element() {
        let css = ".button::before { content: '→'; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert!(scoped.scoped.contains(".sc_test_button::before"));
    }

    #[test]
    fn test_multiple_pseudo_classes() {
        let css = ".link:hover:focus { text-decoration: underline; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert!(scoped.scoped.contains(".sc_test_link:hover:focus"));
    }

    #[test]
    fn test_deeply_nested_selector() {
        let css = ".nav > ul > li > a.active { color: blue; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert!(scoped.scoped.contains(".sc_test_nav"));
        assert!(scoped.scoped.contains("ul[data-scope=\"sc_test\"]"));
        assert!(scoped.scoped.contains("li[data-scope=\"sc_test\"]"));
        assert!(scoped.scoped.contains("a[data-scope=\"sc_test\"].sc_test_active"));
    }

    #[test]
    fn test_empty_css() {
        let css = "";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert_eq!(scoped.scoped, "");
    }

    #[test]
    fn test_whitespace_only() {
        let css = "   \n\t   ";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert_eq!(scoped.scoped.trim(), "");
    }

    // Bug fix tests

    #[test]
    fn test_not_pseudo_class_scoping() {
        // Bug #2: :not() content should be scoped
        let css = ".button:not(.disabled) { color: red; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        // Both .button and .disabled should be scoped
        assert!(scoped.scoped.contains(".sc_test_button:not(.sc_test_disabled)"));
    }

    #[test]
    fn test_has_is_where_pseudo_class_scoping() {
        let css = ".container:has(.child) { display: flex; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        assert!(scoped.scoped.contains(".sc_test_container:has(.sc_test_child)"));
    }

    #[test]
    fn test_attribute_selector_with_quotes() {
        // Bug #3: Attribute selectors with ] inside quotes
        let css = r#"input[placeholder="Enter ]value"] { border: 1px solid; }"#;
        let scoped = parse_and_scope(css, "sc_test", false);
        // Should preserve the full attribute including the ] inside quotes
        assert!(scoped.scoped.contains(r#"[placeholder="Enter ]value"]"#));
    }

    #[test]
    fn test_empty_class_preserved() {
        // Bug #1: Empty class selector should be preserved
        let css = ". { color: red; }";
        let scoped = parse_and_scope(css, "sc_test", false);
        // Should contain the dot (malformed but preserved)
        assert!(scoped.scoped.contains("."));
    }
}