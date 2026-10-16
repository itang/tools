/** Length-prefixed JSON framing for Chrome native messaging. */

import { MAX_FRAME_BYTES } from "./protocol.js";

/** Serialized byte size of a message when framed (JSON only, no prefix). */
export function messageBytes(message) {
  return Buffer.byteLength(JSON.stringify(message), "utf8");
}

/**
 * Encode a message as a Chrome native messaging frame:
 * 4-byte little-endian length prefix followed by UTF-8 JSON.
 * @param {unknown} message
 * @returns {Buffer}
 */
export function encodeFrame(message) {
  const json = Buffer.from(JSON.stringify(message), "utf8");
  const frame = Buffer.allocUnsafe(4 + json.length);
  frame.writeUInt32LE(json.length, 0);
  json.copy(frame, 4);
  return frame;
}

/** Incrementally decode a stream of length-prefixed frames. */
export class FrameDecoder {
  constructor() {
    /** @type {Buffer} */
    this.buffer = Buffer.alloc(0);
  }

  /**
   * Feed raw bytes and return every complete message decoded so far.
   * @param {Uint8Array} chunk
   * @returns {unknown[]}
   */
  push(chunk) {
    this.buffer = Buffer.concat([this.buffer, Buffer.from(chunk)]);
    const messages = [];
    while (this.buffer.length >= 4) {
      const length = this.buffer.readUInt32LE(0);
      if (this.buffer.length < 4 + length) break;
      const json = this.buffer.subarray(4, 4 + length).toString("utf8");
      this.buffer = this.buffer.subarray(4 + length);
      try {
        messages.push(JSON.parse(json));
      } catch {
        // Skip malformed frames rather than tearing down the whole channel.
        messages.push(null);
      }
    }
    return messages;
  }
}

/**
 * Split an array of items into consecutive chunks such that each rendered
 * message stays within `maxBytes`. A single oversized item is kept alone
 * (it cannot be split further).
 *
 * @template T
 * @param {T[]} items
 * @param {(chunk: T[]) => unknown} render
 * @param {number} [maxBytes]
 * @returns {T[][]}
 */
export function chunkItems(items, render, maxBytes = MAX_FRAME_BYTES) {
  const chunks = [];
  let current = [];
  for (const item of items) {
    if (current.length > 0 && messageBytes(render([...current, item])) > maxBytes) {
      chunks.push(current);
      current = [item];
    } else {
      current.push(item);
    }
  }
  chunks.push(current);
  return chunks;
}
