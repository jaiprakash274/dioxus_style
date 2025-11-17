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
#[inline]
fn scope_rule(rule: &str, scope: &str, class_names: &mut HashSet<String>) -> Option<String> {
    let trimmed = rule.trim();
    if trimmed.is_empty() {
        return None;
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

/// Scopes a CSS selector by adding the unique prefix.
#[inline]
fn scope_selector(selector: &str, scope: &str, class_names: &mut HashSet<String>) -> String {
    if !selector.contains(',') {
        // Fast path: single selector
        return scope_single_selector(selector, scope, class_names);
    }

    // Handle multiple selectors
    selector
        .split(',')
        .map(|s| scope_single_selector(s.trim(), scope, class_names))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Scopes a single selector (no commas).
#[inline]
fn scope_single_selector(selector: &str, scope: &str, class_names: &mut HashSet<String>) -> String {
    let mut result = String::with_capacity(selector.len() + scope.len() * 2);
    let mut current_token = String::with_capacity(32);
    let mut chars = selector.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '.' => {
                result.push_str(&current_token);
                current_token.clear();

                // Collect class name
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
                }
            }
            ' ' | '>' | '+' | '~' => {
                result.push_str(&current_token);
                result.push(ch);
                current_token.clear();
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    result.push_str(&current_token);
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
    fn test_parse_and_scope() {
        let css = r#"
            .button {
                color: red;
            }
            .button:hover {
                color: blue;
            }
        "#;

        let scoped = parse_and_scope(css, "sc_abc123", false);

        assert!(scoped.scoped.contains(".sc_abc123_button"));
        assert!(scoped.class_names.contains(&"button".to_string()));
        assert!(!scoped.scoped.contains("} }"));
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
    fn test_multiple_selectors() {
        let css = ".btn, .button { color: red; }";
        let scoped = parse_and_scope(css, "sc_xyz", false);

        assert!(scoped.scoped.contains(".sc_xyz_btn"));
        assert!(scoped.scoped.contains(".sc_xyz_button"));
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
}
