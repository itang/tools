/** Minimal MD5 (RFC 1321) over UTF-8 strings, hex output. No dependencies. */

const S = [
  7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
  5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20,
  4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
  6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K = Array.from(
  { length: 64 },
  (_, i) => Math.floor(Math.abs(Math.sin(i + 1)) * 0x100000000),
);

function rotl(x, n) {
  return (x << n) | (x >>> (32 - n));
}

/**
 * @param {string} message
 * @returns {32-char lowercase hex digest}
 */
export function md5(message) {
  const bytes = new TextEncoder().encode(String(message));
  const bitLen = bytes.length * 8;
  const padded = new Uint8Array((((bitLen + 64) >>> 9) + 1) * 64);
  padded.set(bytes);
  padded[bytes.length] = 0x80;
  new DataView(padded.buffer).setUint32(padded.length - 8, bitLen >>> 0, true);
  new DataView(padded.buffer).setUint32(padded.length - 4, Math.floor(bitLen / 0x100000000), true);

  let a0 = 0x67452301;
  let b0 = 0xefcdab89;
  let c0 = 0x98badcfe;
  let d0 = 0x10325476;
  const view = new DataView(padded.buffer);
  const m = new Int32Array(16);

  for (let offset = 0; offset < padded.length; offset += 64) {
    for (let i = 0; i < 16; i += 1) m[i] = view.getInt32(offset + i * 4, true);

    let a = a0, b = b0, c = c0, d = d0;
    for (let i = 0; i < 64; i += 1) {
      let f;
      let g;
      if (i < 16) {
        f = (b & c) | (~b & d);
        g = i;
      } else if (i < 32) {
        f = (d & b) | (~d & c);
        g = (5 * i + 1) & 15;
      } else if (i < 48) {
        f = b ^ c ^ d;
        g = (3 * i + 5) & 15;
      } else {
        f = c ^ (b | ~d);
        g = (7 * i) & 15;
      }
      const tmp = d;
      d = c;
      c = b;
      b = (b + rotl((a + f + K[i] + m[g]) | 0, S[i])) | 0;
      a = tmp;
    }

    a0 = (a0 + a) | 0;
    b0 = (b0 + b) | 0;
    c0 = (c0 + c) | 0;
    d0 = (d0 + d) | 0;
  }

  const out = new DataView(new ArrayBuffer(16));
  for (const [i, v] of [a0, b0, c0, d0].entries()) out.setUint32(i * 4, v >>> 0, true);
  return Array.from(new Uint8Array(out.buffer), (v) => v.toString(16).padStart(2, "0")).join("");
}
