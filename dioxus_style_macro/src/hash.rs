//! Fast hashing utilities for generating unique class names.
//!
//! Uses xxHash (XXH3) for fast, collision-resistant hashing of CSS content.

use xxhash_rust::xxh3::xxh3_64;

/// Generates a unique, short hash for the given CSS content.
///
/// # Arguments
/// * `content` - The CSS content to hash
/// * `file_path` - Optional file path for additional uniqueness
///
/// # Returns
/// A short hash string like "sc_a1b2c3d"
#[inline]
pub fn generate_hash(content: &str, file_path: Option<&str>) -> String {
    // Pre-allocate capacity to avoid reallocations
    let capacity = file_path.map_or(content.len(), |p| p.len() + 2 + content.len());
    let mut hasher_input = String::with_capacity(capacity);

    if let Some(path) = file_path {
        hasher_input.push_str(path);
        hasher_input.push_str("::");
    }

    hasher_input.push_str(content);

    let hash = xxh3_64(hasher_input.as_bytes());
    format!("sc_{}", encode_base62(hash))
}

/// Encodes a u64 into a base62 string.
///
/// Base62 uses [0-9a-zA-Z] which are all valid CSS identifier characters.
#[inline]
fn encode_base62(mut num: u64) -> String {
    const BASE62_CHARS: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if num == 0 {
        return String::from("0");
    }

    // Pre-allocate: u64 max in base62 is ~11 chars
    let mut result = Vec::with_capacity(11);

    while num > 0 {
        result.push(BASE62_CHARS[(num % 62) as usize]);
        num /= 62;
    }

    result.reverse();
    // SAFETY: BASE62_CHARS is valid UTF-8
    unsafe { String::from_utf8_unchecked(result) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_hash() {
        let css1 = ".button { color: red; }";
        let css2 = ".button { color: blue; }";

        let hash1 = generate_hash(css1, None);
        let hash2 = generate_hash(css2, None);

        assert_ne!(hash1, hash2);

        let hash1_again = generate_hash(css1, None);
        assert_eq!(hash1, hash1_again);

        assert!(hash1.starts_with("sc_"));
    }

    #[test]
    fn test_hash_with_file_path() {
        let css = ".button { color: red; }";

        let hash1 = generate_hash(css, Some("components/button.rs"));
        let hash2 = generate_hash(css, Some("components/card.rs"));

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_encode_base62() {
        assert_eq!(encode_base62(0), "0");
        assert_eq!(encode_base62(61), "Z");
        assert_eq!(encode_base62(62), "10");

        let encoded = encode_base62(123456789);
        assert!(encoded.chars().all(|c| c.is_alphanumeric()));
    }
}
