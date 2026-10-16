///| White-box tests for length-prefixed JSON framing.

///| Repeat `c` `n` times.
fn repeat_char(c : Char, n : Int) -> String {
  let sb = StringBuilder()
  for i = 0; i < n; i = i + 1 {
    sb.write_char(c)
  }
  sb.to_string()
}

///|
test "framing round-trips a frame" {
  let decoder = FrameDecoder::new()
  let frame = encode_frame(
    Json::object({ "id": Json::string("a"), "type": Json::string("health") }),
  )
  let messages = decoder.push(frame[:])
  debug_inspect(messages.length(), content="1")
  let message = messages[0].unwrap()
  debug_inspect(get_str_field(message, "id"), content="Some(\"a\")")
  debug_inspect(get_str_field(message, "type"), content="Some(\"health\")")
}

///|
test "framing reassembles a frame split across chunks" {
  let frame = encode_frame(
    Json::object({ "hello": Json::string("world"), "n": json_int(42) }),
  )
  let decoder = FrameDecoder::new()
  debug_inspect(decoder.push(frame[:2]).length(), content="0")
  let messages = decoder.push(frame[2:])
  debug_inspect(messages.length(), content="1")
  let message = messages[0].unwrap()
  debug_inspect(get_str_field(message, "hello"), content="Some(\"world\")")
  debug_inspect(get_int_field(message, "n"), content="Some(42)")
}

///|
test "framing decodes several frames from one chunk" {
  let buffer = @buffer.Buffer::Buffer()
  buffer.write_bytes(encode_frame(Json::object({ "a": json_int(1) })))
  buffer.write_bytes(encode_frame(Json::object({ "b": json_int(2) })))
  let decoder = FrameDecoder::new()
  let messages = decoder.push(buffer.to_bytes()[:])
  debug_inspect(messages.length(), content="2")
  debug_inspect(get_int_field(messages[0].unwrap(), "a"), content="Some(1)")
  debug_inspect(get_int_field(messages[1].unwrap(), "b"), content="Some(2)")
}

///|
test "chunk_items splits oversized payloads and keeps items intact" {
  let items : Array[Json] = []
  for i = 0; i < 10; i = i + 1 {
    items.push(
      Json::object({
        "index": json_int(i),
        "text": Json::string(repeat_char('x', 100)),
      }),
    )
  }
  let render : (Array[Json]) -> Json = chunk => Json::object({
    "type": Json::string("translate"),
    "translations": Json::array(chunk),
  })
  let chunks = chunk_items(items, render, 300)
  assert_true(chunks.length() > 1)
  let flat : Array[Json] = []
  for chunk in chunks {
    flat.append(chunk[:])
  }
  debug_inspect(flat.length(), content="10")
  for chunk in chunks {
    assert_true(message_bytes(render(chunk)) <= 300)
  }
}

///|
test "chunk_items keeps a single item that cannot be split further" {
  let items : Array[Json] = [
    Json::object({
      "index": json_int(0),
      "text": Json::string(repeat_char('y', 5000)),
    }),
  ]
  let render : (Array[Json]) -> Json = chunk => Json::object({
    "translations": Json::array(chunk),
  })
  let chunks = chunk_items(items, render, 100)
  debug_inspect(chunks.length(), content="1")
  debug_inspect(chunks[0].length(), content="1")
}
