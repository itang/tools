///| Small helpers for reading and building JSON values.

///| Extract the object members of a JSON value, or `None` when it is not an object.
pub fn as_object(json : Json) -> Map[String, Json]? {
  match json {
    Object(members) => Some(members)
    _ => None
  }
}

///| Extract a JSON string, or `None`.
pub fn as_string(json : Json) -> String? {
  match json {
    String(s) => Some(s)
    _ => None
  }
}

///| Extract a JSON number as an integer, or `None`.
pub fn as_int(json : Json) -> Int? {
  match json {
    Number(n, ..) => Some(n.to_int())
    _ => None
  }
}

///| Extract a JSON array, or `None`.
pub fn as_array(json : Json) -> Array[Json]? {
  match json {
    Array(items) => Some(items)
    _ => None
  }
}

///| Read a member of a JSON object by key.
pub fn get_field(json : Json, key : String) -> Json? {
  match json {
    Object(members) => members.get(key)
    _ => None
  }
}

///| Read a string member of a JSON object.
pub fn get_str_field(json : Json, key : String) -> String? {
  match get_field(json, key) {
    Some(value) => as_string(value)
    None => None
  }
}

///| Read an integer member of a JSON object.
pub fn get_int_field(json : Json, key : String) -> Int? {
  match get_field(json, key) {
    Some(value) => as_int(value)
    None => None
  }
}

///| Wrap an integer as a JSON number.
pub fn json_int(value : Int) -> Json {
  Json::number(value.to_double())
}

///| Parse a JSON document, returning `None` on malformed input.
pub fn parse_json_option(text : String) -> Json? {
  try {
    Some(@json.parse(text))
  } catch {
    _ => None
  }
}
