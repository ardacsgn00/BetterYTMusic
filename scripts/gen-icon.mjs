// Generates app-icon.png (1024x1024): solid red rounded square + white play glyph.
// Usage: node scripts/gen-icon.mjs && npx tauri icon app-icon.png
import { deflateSync } from 'node:zlib';
import { writeFileSync } from 'node:fs';

const S = 1024;
const px = Buffer.alloc(S * S * 4);
const bg = [255, 78, 69], shade = [196, 44, 38], fg = [255, 255, 255];
const r = 220, m = 64;

const inRounded = (x, y, x0, y0, x1, y1, rad) => {
  if (x < x0 || x > x1 || y < y0 || y > y1) return false;
  const cx = Math.min(Math.max(x, x0 + rad), x1 - rad);
  const cy = Math.min(Math.max(y, y0 + rad), y1 - rad);
  return (x - cx) ** 2 + (y - cy) ** 2 <= rad * rad;
};
// Play triangle
const ax = 400, ay = 300, bx = 400, by = 700, cx = 740, cy = 500;
const sign = (px1, py1, px2, py2, px3, py3) => (px1 - px3) * (py2 - py3) - (px2 - px3) * (py1 - py3);
const inTri = (x, y) => {
  const d1 = sign(x, y, ax, ay, bx, by), d2 = sign(x, y, bx, by, cx, cy), d3 = sign(x, y, cx, cy, ax, ay);
  return !((d1 < 0 || d2 < 0 || d3 < 0) && (d1 > 0 || d2 > 0 || d3 > 0));
};

for (let y = 0; y < S; y++) for (let x = 0; x < S; x++) {
  let c = null;
  if (inRounded(x, y, m, m + 40, S - m, S - m, r)) c = shade; // tactile bottom edge
  if (inRounded(x, y, m, m, S - m, S - m - 40, r)) c = bg;
  if (c === bg && inTri(x, y)) c = fg;
  if (c) { const i = (y * S + x) * 4; px[i] = c[0]; px[i + 1] = c[1]; px[i + 2] = c[2]; px[i + 3] = 255; }
}

const raw = Buffer.alloc((S * 4 + 1) * S);
for (let y = 0; y < S; y++) { raw[y * (S * 4 + 1)] = 0; px.copy(raw, y * (S * 4 + 1) + 1, y * S * 4, (y + 1) * S * 4); }

const crcTable = Array.from({ length: 256 }, (_, n) => { let c = n; for (let k = 0; k < 8; k++) c = c & 1 ? 0xedb88320 ^ (c >>> 1) : c >>> 1; return c >>> 0; });
const crc = (b) => { let c = 0xffffffff; for (const v of b) c = crcTable[(c ^ v) & 0xff] ^ (c >>> 8); return (c ^ 0xffffffff) >>> 0; };
const chunk = (type, data) => {
  const len = Buffer.alloc(4); len.writeUInt32BE(data.length);
  const td = Buffer.concat([Buffer.from(type), data]);
  const c = Buffer.alloc(4); c.writeUInt32BE(crc(td));
  return Buffer.concat([len, td, c]);
};
const ihdr = Buffer.alloc(13);
ihdr.writeUInt32BE(S, 0); ihdr.writeUInt32BE(S, 4); ihdr[8] = 8; ihdr[9] = 6;

writeFileSync('app-icon.png', Buffer.concat([
  Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
  chunk('IHDR', ihdr), chunk('IDAT', deflateSync(raw)), chunk('IEND', Buffer.alloc(0))
]));
console.log('app-icon.png written');

