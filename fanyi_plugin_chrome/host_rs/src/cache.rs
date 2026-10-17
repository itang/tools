//! Translation result cache with LRU eviction.

use std::collections::{HashMap, VecDeque};

/// Collapse whitespace so cosmetic differences share a cache entry.
pub fn normalize_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_space = false;
    for ch in text.chars() {
        if ch.is_whitespace() {
            if !out.is_empty() {
                pending_space = true;
            }
        } else {
            if pending_space {
                out.push(' ');
                pending_space = false;
            }
            out.push(ch);
        }
    }
    out
}

/// Stable content hash for a normalized string.
///
/// Determinism and distinctness are all the in-memory cache requires, so a
/// FNV-1a 64-bit hash rendered as hexadecimal is sufficient (no SHA-256
/// dependency needed).
pub fn hash_text(text: &str) -> String {
    let normalized = normalize_text(text);
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in normalized.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// Cache key for one (language, model, text) triple.
pub fn cache_key(target_lang: &str, model: &str, text: &str) -> String {
    format!("{target_lang}\u{0}{model}\u{0}{}", hash_text(text))
}

/// A bounded map that evicts the least recently used entry.
pub struct LruCache {
    max: usize,
    map: HashMap<String, String>,
    order: VecDeque<String>,
}

impl LruCache {
    pub fn new(max: usize) -> Self {
        Self {
            max: max.max(1),
            map: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[allow(dead_code)]
    pub fn has(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn get(&mut self, key: &str) -> Option<String> {
        let value = self.map.get(key).cloned()?;
        self.touch(key);
        Some(value)
    }

    pub fn set(&mut self, key: &str, value: String) {
        if self.map.contains_key(key) {
            self.order.retain(|existing| existing != key);
        }
        self.map.insert(key.to_string(), value);
        self.order.push_back(key.to_string());
        while self.map.len() > self.max {
            match self.order.pop_front() {
                Some(oldest) => {
                    self.map.remove(&oldest);
                }
                None => break,
            }
        }
    }

    fn touch(&mut self, key: &str) {
        self.order.retain(|existing| existing != key);
        self.order.push_back(key.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_whitespace() {
        assert_eq!(normalize_text("  a\n\t b  "), "a b");
    }

    #[test]
    fn cache_key_distinguishes_target_language_and_model() {
        assert_ne!(
            cache_key("zh-CN", "m1", "hello"),
            cache_key("zh-CN", "m2", "hello")
        );
        assert_ne!(
            cache_key("zh-CN", "m1", "hello"),
            cache_key("en", "m1", "hello")
        );
        assert_eq!(
            cache_key("zh-CN", "m1", "a  b"),
            cache_key("zh-CN", "m1", "a b")
        );
    }

    #[test]
    fn evicts_the_least_recently_used_entry() {
        let mut cache = LruCache::new(2);
        cache.set("a", "1".to_string());
        cache.set("b", "2".to_string());
        assert_eq!(cache.get("a"), Some("1".to_string())); // "a" becomes newest
        cache.set("c", "3".to_string()); // evicts "b"
        assert!(!cache.has("b"));
        assert!(cache.has("a"));
        assert!(cache.has("c"));
        assert_eq!(cache.len(), 2);
    }
}
