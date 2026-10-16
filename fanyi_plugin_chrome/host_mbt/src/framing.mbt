///| Length-prefixed JSON framing for Chrome native messaging.

///| Read a little-endian 32-bit unsigned integer from `bytes` at `offset`.
pub fn read_u32_le(bytes : Bytes, offset : Int) -> Int {
  bytes[offset].to_int() |
  (bytes[offset + 1].to_int() << 8) |
  (bytes[offset + 2].to_int() << 16) |
  (bytes[offset + 3].to_int() << 24)
}

///| Serialized UTF-8 byte size of a message (JSON only, no length prefix).
pub fn message_bytes(message : Json) -> Int {
  @utf8.encode(message.stringify()).length()
}

///| Encode a message as a Chrome native messaging frame:
/// 4-byte little-endian length prefix followed by UTF-8 JSON.
pub fn encode_frame(message : Json) -> Bytes {
  let payload = @utf8.encode(message.stringify())
  let size = payload.length()
  let out = @buffer.Buffer::Buffer()
  out.write_byte((size & 0xFF).to_byte())
  out.write_byte(((size >> 8) & 0xFF).to_byte())
  out.write_byte(((size >> 16) & 0xFF).to_byte())
  out.write_byte(((size >> 24) & 0xFF).to_byte())
  out.write_bytes(payload)
  out.to_bytes()
}

///| Decode one UTF-8 JSON payload, returning `None` for malformed input.
fn decode_payload(payload : BytesView) -> Json? {
  try {
    Some(@json.parse(@utf8.decode(payload)))
  } catch {
    _ => None
  }
}

///| Incrementally decode a stream of length-prefixed frames.
pub struct FrameDecoder {
  mut buffer : Bytes
}

///| Create an empty decoder.
pub fn FrameDecoder::new() -> FrameDecoder {
  { buffer: Bytes::new(0) }
}

///| Feed raw bytes and return every complete message decoded so far.
/// Malformed frames decode to `None` rather than tearing down the channel.
pub fn FrameDecoder::push(
  self : FrameDecoder,
  chunk : BytesView,
) -> Array[Json?] {
  let acc = @buffer.from_bytes(self.buffer[:])
  acc.write_bytes(chunk)
  let data = acc.to_bytes()
  let messages : Array[Json?] = []
  let total = data.length()
  let mut offset = 0
  while offset + 4 <= total {
    let size = read_u32_le(data, offset)
    if offset + 4 + size > total {
      break
    }
    messages.push(decode_payload(data[offset + 4 : offset + 4 + size]))
    offset = offset + 4 + size
  }
  self.buffer = data[offset : total].to_owned()
  messages
}

///| Split `items` into consecutive chunks such that each rendered message
/// stays within `max_bytes`. A single oversized item is kept alone (it cannot
/// be split further).
pub fn[T] chunk_items(
  items : Array[T],
  render : (Array[T]) -> Json,
  max_bytes : Int,
) -> Array[Array[T]] {
  let chunks : Array[Array[T]] = []
  let mut current : Array[T] = []
  for item in items {
    if current.is_empty() {
      current.push(item)
    } else {
      let probe : Array[T] = []
      probe.append(current[:])
      probe.push(item)
      if message_bytes(render(probe)) > max_bytes {
        chunks.push(current)
        current = [item]
      } else {
        current.push(item)
      }
    }
  }
  chunks.push(current)
  chunks
}
