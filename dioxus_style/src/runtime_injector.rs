//! Runtime style injection and registry.
//!
//! Manages the collection and injection of scoped styles into the DOM.

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    /// Global registry for all scoped styles.
    pub static ref STYLE_REGISTRY: Arc<Mutex<StyleRegistry>> = Arc::new(Mutex::new(StyleRegistry::new()));
}

/// Registry that tracks all scoped styles in the application.
#[derive(Debug, Default)]
pub struct StyleRegistry {
    // HashMap for O(1) lookups and deduplication
    styles: HashMap<String, String>,
    // Maintain insertion order for consistent output
    order: Vec<String>,
}

impl StyleRegistry {
    /// Creates a new empty style registry.
    #[inline]
    pub fn new() -> Self {
        Self {
            styles: HashMap::with_capacity(32), // Pre-allocate for typical use
            order: Vec::with_capacity(32),
        }
    }

    /// Registers a scoped style with its hash.
    ///
    /// # Arguments
    /// * `hash` - The unique hash/scope for this style
    /// * `css` - The scoped CSS content
    #[inline]
    pub fn register(&mut self, hash: String, css: String) {
        use std::collections::hash_map::Entry;

        match self.styles.entry(hash.clone()) {
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
    pub fn len(&self) -> usize {
        self.styles.len()
    }

    /// Checks if the registry is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.styles.is_empty()
    }
}

#[inline]
pub fn inject_styles() -> String {
    STYLE_REGISTRY
        .lock()
        .expect("StyleRegistry lock poisoned")
        .get_all_styles()
}

/// Helper struct for managing a single scoped style instance.
#[derive(Debug, Clone)]
pub struct ScopedStyle {
    pub scope: String,
}

impl ScopedStyle {
    /// Creates a new scoped style and registers it.
    #[inline]
    pub fn new(scope: String, css: String) -> Self {
        STYLE_REGISTRY
            .lock()
            .expect("StyleRegistry lock poisoned")
            .register(scope.clone(), css);

        Self { scope }
    }

    /// Returns the scope prefix for use in class names.
    #[inline]
    pub fn scope(&self) -> &str {
        &self.scope
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

        registry.register("hash1".to_string(), "css1".to_string());
        registry.register("hash1".to_string(), "css_updated".to_string());

        assert_eq!(registry.len(), 1);
        assert!(registry.get_all_styles().contains("css_updated"));
    }

    #[test]
    fn test_registry_order() {
        let mut registry = StyleRegistry::new();

        registry.register("a".to_string(), "css_a".to_string());
        registry.register("b".to_string(), "css_b".to_string());
        registry.register("c".to_string(), "css_c".to_string());

        let styles = registry.get_all_styles();
        let a_pos = styles.find("css_a").unwrap();
        let b_pos = styles.find("css_b").unwrap();
        let c_pos = styles.find("css_c").unwrap();

        assert!(a_pos < b_pos);
        assert!(b_pos < c_pos);
    }
}
