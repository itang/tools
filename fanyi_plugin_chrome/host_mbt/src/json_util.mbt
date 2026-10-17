///| Tolerant JSON parsing.
///
/// Reading and building JSON no longer needs wrapper helpers: the language has
/// JSON object patterns (`json is { "type": String(kind), .. }`) and JSON
/// literals (`{ "type": "translate", "seq": 3 }`) built on `ToJson`.

///|
/// Parse a JSON document, returning `None` on malformed input.
pub fn parse_json_option(text : String) -> Json? {
  Some(@json.parse(text)) catch {
    _ => None
  }
}
