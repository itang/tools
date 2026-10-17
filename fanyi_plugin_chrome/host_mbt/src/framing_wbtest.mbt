///| White-box tests for length-prefixed JSON framing.

///|
/// Repeat `c` `n` times.
fn repeat_char(c : Char, n : Int32) -> String {
  let sb = StringBuilder()
  for i = 0; i < n; i = i + 1 {
    sb.write_char(c)
  }
  sb.to_string()
}

///|
test "framing round-trips a frame" {
  let decoder = FrameDecoder::new()
  let frame = encode_frame(({ "id": "a", "type": "health" } : Json))
  let messages = decoder.push(frame[:])
  debug_inspect(messages.length(), content="1")
  assert_true(messages[0] is Some({ "id": "a", "type": "health", .. }))
}

///|
test "framing reassembles a frame split across chunks" {
  let frame = encode_frame(({ "hello": "world", "n": 42 } : Json))
  let decoder = FrameDecoder::new()
  debug_inspect(decoder.push(frame[:2]).length(), content="0")
  let messages = decoder.push(frame[2:])
  debug_inspect(messages.length(), content="1")
  assert_true(
    messages[0] is Some({ "hello": "world", "n": Number(n, ..), .. }) &&
    n.to_int() == 42,
  )
}

///|
test "framing decodes several frames from one chunk" {
  let buffer = @buffer.Buffer::Buffer()
  buffer.write_bytes(encode_frame(({ "a": 1 } : Json)))
  buffer.write_bytes(encode_frame(({ "b": 2 } : Json)))
  let decoder = FrameDecoder::new()
  let messages = decoder.push(buffer.to_bytes()[:])
  debug_inspect(messages.length(), content="2")
  assert_true(
    messages[0] is Some({ "a": Number(a, ..), .. }) && a.to_int() == 1,
  )
  assert_true(
    messages[1] is Some({ "b": Number(b, ..), .. }) && b.to_int() == 2,
  )
}

///|
test "chunk_items splits oversized payloads and keeps items intact" {
  let items : Array[Json] = []
  for i = 0; i < 10; i = i + 1 {
    items.push({ "index": i, "text": repeat_char('x', 100) })
  }
  let render : (Array[Json]) -> Json = chunk => {
    "type": "translate",
    "translations": Json::array(chunk),
  }
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
  let items : Array[Json] = [{ "index": 0, "text": repeat_char('y', 5000) }]
  let render : (Array[Json]) -> Json = chunk => {
    "translations": Json::array(chunk),
  }
  let chunks = chunk_items(items, render, 100)
  debug_inspect(chunks.length(), content="1")
  debug_inspect(chunks[0].length(), content="1")
}
