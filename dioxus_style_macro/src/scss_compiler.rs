//! SCSS compilation utilities.
//!
//! Compiles SCSS/SASS to CSS at compile time using the grass crate.
//! Includes caching to avoid re-compiling the same SCSS content.

use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

/// Maximum number of cached SCSS compilations.
/// Prevents unbounded memory growth during long compilation sessions.
const MAX_CACHE_SIZE: usize = 128;

/// Cache for compiled SCSS → CSS to avoid redundant compilations.
/// Key is the SCSS content hash, value is the compiled CSS.
static SCSS_CACHE: OnceLock<RwLock<ScssCache>> = OnceLock::new();

/// SCSS cache with size tracking for bounded growth.
struct ScssCache {
    entries: HashMap<u64, String>,
    insertion_order: Vec<u64>, // For LRU-like eviction
}

impl ScssCache {
    fn new() -> Self {
        Self {
            entries: HashMap::with_capacity(32),
            insertion_order: Vec::with_capacity(32),
        }
    }

    fn get(&self, key: &u64) -> Option<&String> {
        self.entries.get(key)
    }

    fn insert(&mut self, key: u64, value: String) {
        // Evict oldest entries if cache is full
        while self.entries.len() >= MAX_CACHE_SIZE && !self.insertion_order.is_empty() {
            if let Some(oldest_key) = self.insertion_order.first().copied() {
                self.entries.remove(&oldest_key);
                self.insertion_order.remove(0);
            }
        }

        if self.entries.insert(key, value).is_none() {
            self.insertion_order.push(key);
        }
    }
}

fn get_cache() -> &'static RwLock<ScssCache> {
    SCSS_CACHE.get_or_init(|| RwLock::new(ScssCache::new()))
}

/// Simple hash function for cache key.
#[inline]
fn hash_content(content: &str, minify: bool) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content.hash(&mut hasher);
    minify.hash(&mut hasher);
    hasher.finish()
}

/// Compiles SCSS content to CSS with caching.
///
/// # Arguments
/// * `content` - The SCSS content to compile
/// * `file_path` - Optional file path for better error messages
/// * `minify` - Whether to minify the output
///
/// # Returns
/// Compiled CSS string or error message
#[cfg(feature = "scss")]
#[must_use = "compiled CSS should be used"]
pub fn compile_scss_to_css(
    content: &str,
    file_path: Option<&str>,
    minify: bool,
) -> Result<String, String> {
    use grass::{Options, OutputStyle};

    let cache_key = hash_content(content, minify);
    
    // Check cache first (graceful handling if lock poisoned)
    if let Ok(cache) = get_cache().read() {
        if let Some(cached) = cache.get(&cache_key) {
            return Ok(cached.clone());
        }
    }

    // Compile SCSS
    let options = Options::default().style(if minify {
        OutputStyle::Compressed
    } else {
        OutputStyle::Expanded
    });

    let result = grass::from_string(content.to_string(), &options).map_err(|e| {
        if let Some(path) = file_path {
            format!("SCSS compilation error in '{}': {}", path, e)
        } else {
            format!("SCSS compilation error: {}", e)
        }
    })?;

    // Store in cache (graceful handling if lock poisoned)
    if let Ok(mut cache) = get_cache().write() {
        cache.insert(cache_key, result.clone());
    }

    Ok(result)
}

/// Fallback when SCSS feature is disabled.
#[cfg(not(feature = "scss"))]
pub fn compile_scss_to_css(
    _content: &str,
    file_path: Option<&str>,
    _minify: bool,
) -> Result<String, String> {
    let file_info = file_path
        .map(|p| format!(" in '{}'", p))
        .unwrap_or_default();
    
    Err(format!(
        "SCSS support is not enabled{}. \
        Add 'scss' feature to dioxus_style dependency: \
        dioxus_style = {{ version = \"0.2\", features = [\"scss\"] }}",
        file_info
    ))
}

/// Checks if a file is SCSS/SASS based on extension.
#[inline]
pub fn is_scss_file(path: &str) -> bool {
    path.ends_with(".scss") || path.ends_with(".sass")
}

#[cfg(test)]
#[cfg(feature = "scss")]
mod tests {
    use super::*;

    #[test]
    fn test_basic_scss_compilation() {
        let scss = r#"
            $primary: #3498db;
            
            .button {
                color: $primary;
            }
        "#;

        let css = compile_scss_to_css(scss, None, false).unwrap();
        assert!(css.contains("#3498db"));
        assert!(css.contains(".button"));
    }

    #[test]
    fn test_scss_nesting() {
        let scss = r#"
            .parent {
                color: red;
                
                .child {
                    color: blue;
                }
            }
        "#;

        let css = compile_scss_to_css(scss, None, false).unwrap();
        assert!(css.contains(".parent"));
        assert!(css.contains(".parent .child"));
    }

    #[test]
    fn test_scss_parent_selector() {
        let scss = r#"
            .button {
                color: blue;
                
                &:hover {
                    color: red;
                }
            }
        "#;

        let css = compile_scss_to_css(scss, None, false).unwrap();
        assert!(css.contains(".button"));
        assert!(css.contains(".button:hover"));
    }

    #[test]
    fn test_scss_minification() {
        let scss = r#"
            .button {
                color: red;
                padding: 10px;
            }
        "#;

        let minified = compile_scss_to_css(scss, None, true).unwrap();
        let expanded = compile_scss_to_css(scss, None, false).unwrap();

        assert!(minified.len() < expanded.len());
        assert!(!minified.contains('\n'));
    }

    #[test]
    fn test_scss_error_handling() {
        let invalid_scss = r#"
            .button {
                color: $undefined-variable;
            }
        "#;

        let result = compile_scss_to_css(invalid_scss, Some("test.scss"), false);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("test.scss"));
    }

    #[test]
    fn test_is_scss_file() {
        assert!(is_scss_file("button.scss"));
        assert!(is_scss_file("styles/main.sass"));
        assert!(is_scss_file("path/to/style.scss"));
        
        assert!(!is_scss_file("button.css"));
        assert!(!is_scss_file("style.js"));
        assert!(!is_scss_file("component.rs"));
    }
}