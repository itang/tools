///| White-box tests for the LRU cache and cache keys.

///|
test "normalize text collapses runs of whitespace" {
  debug_inspect(normalize_text("  a\n\t b  "), content="\"a b\"")
}

///|
test "normalize text collapses unicode whitespace like nbsp" {
  debug_inspect(normalize_text("a\u{00A0}\u{3000}b"), content="\"a b\"")
}

///|
test "cache key distinguishes target language and model" {
  assert_true(
    cache_key("zh-CN", "m1", "hello") != cache_key("zh-CN", "m2", "hello"),
  )
  assert_true(
    cache_key("zh-CN", "m1", "hello") != cache_key("en", "m1", "hello"),
  )
  assert_true(
    cache_key("zh-CN", "m1", "a  b") == cache_key("zh-CN", "m1", "a b"),
  )
}

///|
test "lru cache evicts the least recently used entry" {
  let cache = LruCache::new(2)
  cache.set("a", "1")
  cache.set("b", "2")
  debug_inspect(cache.get("a"), content="Some(\"1\")")
  cache.set("c", "3")
  assert_false(cache.has("b"))
  assert_true(cache.has("a"))
  assert_true(cache.has("c"))
  debug_inspect(cache.size(), content="2")
}
