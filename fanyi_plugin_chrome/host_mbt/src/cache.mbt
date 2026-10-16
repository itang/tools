///| Translation result cache with LRU eviction.

///| Collapse whitespace so cosmetic differences share a cache entry.
pub fn normalize_text(text : String) -> String {
  let sb = StringBuilder()
  let mut started = false
  let mut pending = false
  for c in text {
    if c == ' ' || c == '\t' || c == '\n' || c == '\r' {
      if started {
        pending = true
      }
    } else {
      if pending {
        sb.write_char(' ')
        pending = false
      }
      sb.write_char(c)
      started = true
    }
  }
  sb.to_string()
}

///| Stable content hash (FNV-1a, 64-bit, rendered as hexadecimal) for a
/// normalized string.
pub fn hash_text(text : String) -> String {
  let mut hash : UInt64 = 0xcbf29ce484222325
  for unit in normalize_text(text).code_units() {
    hash = (hash ^ unit.to_uint64()) * 0x100000001b3
  }
  hash.to_string(radix=16)
}

///| Cache key for one (language, model, text) triple.
pub fn cache_key(target_lang : String, model : String, text : String) -> String {
  target_lang + "\u{0000}" + model + "\u{0000}" + hash_text(text)
}

///| A least-recently-used cache mapping cache keys to translations.
pub struct LruCache {
  max : Int
  map : Map[String, String]
}

///| Create an LRU cache holding at most `max` entries.
pub fn LruCache::new(max : Int) -> LruCache {
  { max: if max < 1 { 1 } else { max }, map: Map([]) }
}

///| Number of cached entries.
pub fn LruCache::size(self : LruCache) -> Int {
  self.map.length()
}

///| Whether a key is present (without refreshing its recency).
pub fn LruCache::has(self : LruCache, key : String) -> Bool {
  self.map.contains(key)
}

///| Look up a key, refreshing its recency on a hit.
pub fn LruCache::get(self : LruCache, key : String) -> String? {
  match self.map.get(key) {
    Some(value) => {
      self.map.remove(key)
      self.map.set(key, value)
      Some(value)
    }
    None => None
  }
}

///| Insert a key/value pair, evicting the least recently used entries.
pub fn LruCache::set(
  self : LruCache,
  key : String,
  value : String,
) -> Unit {
  self.map.remove(key)
  self.map.set(key, value)
  while self.map.length() > self.max {
    match self.map.keys().next() {
      Some(oldest) => self.map.remove(oldest)
      None => break
    }
  }
}
