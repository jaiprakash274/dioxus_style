//! Runtime style injection and registry.
//!
//! Manages the collection and injection of scoped styles into the DOM.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Global registry for all scoped styles.
/// Uses OnceLock (Rust 1.70+) instead of lazy_static for better performance.
/// RwLock allows concurrent reads which is the common case for inject_styles().
pub static STYLE_REGISTRY: OnceLock<RwLock<StyleRegistry>> = OnceLock::new();

/// Gets the style registry, initializing it if necessary.
#[inline]
fn get_registry() -> &'static RwLock<StyleRegistry> {
    STYLE_REGISTRY.get_or_init(|| RwLock::new(StyleRegistry::new()))
}

/// Registry that tracks all scoped styles in the application.
/// Uses &'static str to avoid runtime allocations - all CSS is embedded at compile time.
#[derive(Debug, Default)]
pub struct StyleRegistry {
    // HashMap for O(1) lookups and deduplication
    styles: HashMap<&'static str, &'static str>,
    // Maintain insertion order for consistent output
    order: Vec<&'static str>,
}

impl StyleRegistry {
    /// Creates a new empty style registry.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {
            styles: HashMap::with_capacity(32), // Pre-allocate for typical use
            order: Vec::with_capacity(32),
        }
    }

    /// Registers a scoped style with its hash.
    ///
    /// # Arguments
    /// * `hash` - The unique hash/scope for this style (compile-time static)
    /// * `css` - The scoped CSS content (compile-time static)
    #[inline]
    pub fn register(&mut self, hash: &'static str, css: &'static str) {
        use std::collections::hash_map::Entry;

        match self.styles.entry(hash) {
            Entry::Occupied(mut entry) => {
                // Update existing entry
                entry.insert(css);
            }
            Entry::Vacant(entry) => {
                // Insert new entry and track order
                entry.insert(css);
                self.order.push(hash);
            }
        }
    }

    /// Gets all registered styles as a single CSS string.
    #[inline]
    #[must_use]
    pub fn get_all_styles(&self) -> String {
        if self.order.is_empty() {
            return String::new();
        }

        // Pre-calculate total size to avoid reallocations
        let total_size: usize = self
            .styles
            .values()
            .map(|s| s.len() + 1) // +1 for newline
            .sum();

        let mut result = String::with_capacity(total_size);

        for hash in &self.order {
            if let Some(css) = self.styles.get(hash) {
                result.push_str(css);
                result.push('\n');
            }
        }

        result
    }

    /// Checks if a style hash is already registered.
    #[inline]
    #[must_use]
    pub fn contains(&self, hash: &str) -> bool {
        self.styles.contains_key(hash)
    }

    /// Clears all registered styles (useful for testing).
    #[inline]
    pub fn clear(&mut self) {
        self.styles.clear();
        self.order.clear();
    }

    /// Gets the number of registered styles.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.styles.len()
    }

    /// Checks if the registry is empty.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.styles.is_empty()
    }
}

/// Gets all styles from the global registry as a single CSS string for injection.
#[inline]
#[must_use]
pub fn inject_styles() -> String {
    get_registry()
        .read()
        .expect("StyleRegistry RwLock poisoned")
        .get_all_styles()
}

/// Helper struct for managing a single scoped style instance.
/// Uses &'static str for zero-allocation access to compile-time embedded strings.
#[derive(Debug, Clone, Copy)]
pub struct ScopedStyle {
    pub scope: &'static str,
}

impl ScopedStyle {
    /// Creates a new scoped style and registers it.
    /// Both scope and css are compile-time static strings.
    #[inline]
    pub fn new(scope: &'static str, css: &'static str) -> Self {
        get_registry()
            .write()
            .expect("StyleRegistry RwLock poisoned")
            .register(scope, css);

        Self { scope }
    }

    /// Returns the scope prefix for use in class names.
    #[inline]
    pub fn scope(&self) -> &'static str {
        self.scope
    }
}

impl std::fmt::Display for ScopedStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.scope)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_deduplication() {
        let mut registry = StyleRegistry::new();

        registry.register("hash1", "css1");
        registry.register("hash1", "css_updated");

        assert_eq!(registry.len(), 1);
        assert!(registry.get_all_styles().contains("css_updated"));
    }

    #[test]
    fn test_registry_order() {
        let mut registry = StyleRegistry::new();

        registry.register("a", "css_a");
        registry.register("b", "css_b");
        registry.register("c", "css_c");

        let styles = registry.get_all_styles();
        let a_pos = styles.find("css_a").unwrap();
        let b_pos = styles.find("css_b").unwrap();
        let c_pos = styles.find("css_c").unwrap();

        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);
    }
}
