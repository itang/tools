//! Length-prefixed JSON framing for Chrome native messaging.

use serde_json::Value;

/// Serialized byte size of a message when framed (JSON only, no prefix).
pub fn message_bytes(message: &Value) -> usize {
    serde_json::to_vec(message)
        .map(|bytes| bytes.len())
        .unwrap_or(0)
}

/// Encode a message as a Chrome native messaging frame:
/// 4-byte little-endian length prefix followed by UTF-8 JSON.
pub fn encode_frame(message: &Value) -> Vec<u8> {
    let json = serde_json::to_vec(message).unwrap_or_default();
    let mut frame = Vec::with_capacity(4 + json.len());
    frame.extend_from_slice(&(json.len() as u32).to_le_bytes());
    frame.extend_from_slice(&json);
    frame
}

/// Incrementally decode a stream of length-prefixed frames.
#[derive(Default)]
pub struct FrameDecoder {
    buffer: Vec<u8>,
}

impl FrameDecoder {
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Feed raw bytes and return every complete message decoded so far.
    ///
    /// Malformed frames decode to `Value::Null` so the caller can skip them
    /// rather than tear down the whole channel.
    pub fn push(&mut self, chunk: &[u8]) -> Vec<Value> {
        self.buffer.extend_from_slice(chunk);
        let mut messages = Vec::new();
        loop {
            if self.buffer.len() < 4 {
                break;
            }
            let length = u32::from_le_bytes([
                self.buffer[0],
                self.buffer[1],
                self.buffer[2],
                self.buffer[3],
            ]) as usize;
            if self.buffer.len() < 4 + length {
                break;
            }
            let parsed =
                serde_json::from_slice::<Value>(&self.buffer[4..4 + length]).unwrap_or(Value::Null);
            messages.push(parsed);
            self.buffer.drain(..4 + length);
        }
        messages
    }
}

/// Split an array of items into consecutive chunks such that each rendered
/// message stays within `max_bytes`. A single oversized item is kept alone
/// (it cannot be split further).
pub fn chunk_items<T, R>(items: &[T], render: R, max_bytes: usize) -> Vec<Vec<T>>
where
    T: Clone,
    R: Fn(&[T]) -> Value,
{
    let mut chunks: Vec<Vec<T>> = Vec::new();
    let mut current: Vec<T> = Vec::new();
    for item in items {
        if !current.is_empty() {
            let mut probe = current.clone();
            probe.push(item.clone());
            if message_bytes(&render(&probe)) > max_bytes {
                chunks.push(std::mem::take(&mut current));
            }
        }
        current.push(item.clone());
    }
    chunks.push(current);
    chunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn round_trips_a_message() {
        let mut decoder = FrameDecoder::new();
        let frame = encode_frame(&json!({ "id": "a", "type": "health" }));
        assert_eq!(
            decoder.push(&frame),
            vec![json!({ "id": "a", "type": "health" })]
        );
    }

    #[test]
    fn reassembles_a_frame_split_across_chunks() {
        let frame = encode_frame(&json!({ "hello": "world", "n": 42 }));
        let mut decoder = FrameDecoder::new();
        assert!(decoder.push(&frame[..2]).is_empty());
        assert_eq!(
            decoder.push(&frame[2..]),
            vec![json!({ "hello": "world", "n": 42 })]
        );
    }

    #[test]
    fn decodes_several_frames_from_one_chunk() {
        let mut decoder = FrameDecoder::new();
        let mut joined = encode_frame(&json!({ "a": 1 }));
        joined.extend(encode_frame(&json!({ "b": 2 })));
        assert_eq!(
            decoder.push(&joined),
            vec![json!({ "a": 1 }), json!({ "b": 2 })]
        );
    }

    #[test]
    fn malformed_frame_decodes_to_null() {
        let mut decoder = FrameDecoder::new();
        let mut frame = Vec::new();
        frame.extend_from_slice(&3u32.to_le_bytes());
        frame.extend_from_slice(b"n/a");
        assert_eq!(decoder.push(&frame), vec![Value::Null]);
    }

    #[test]
    fn chunk_items_splits_oversized_payloads_and_keeps_items_intact() {
        let items: Vec<Value> = (0..10)
            .map(|index| json!({ "index": index, "text": "x".repeat(100) }))
            .collect();
        let chunks = chunk_items(
            &items,
            |chunk| json!({ "type": "translate", "translations": chunk }),
            300,
        );
        assert!(chunks.len() > 1);
        assert_eq!(chunks.iter().flatten().cloned().collect::<Vec<_>>(), items);
        for chunk in &chunks {
            assert!(message_bytes(&json!({ "type": "translate", "translations": chunk })) <= 300);
        }
    }

    #[test]
    fn chunk_items_keeps_a_single_item_that_cannot_be_split_further() {
        let items = vec![json!({ "index": 0, "text": "y".repeat(5000) })];
        let chunks = chunk_items(&items, |c| json!({ "translations": c }), 100);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].len(), 1);
    }
}
