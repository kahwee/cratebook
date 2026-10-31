#!/usr/bin/env node
import { createServer } from 'node:http';
import { createReadStream } from 'node:fs';
import { stat } from 'node:fs/promises';

const root = new URL('../web/', import.meta.url);
const files = new Map([
  ['/', ['index.html', 'text/html; charset=utf-8']],
  ['/index.html', ['index.html', 'text/html; charset=utf-8']],
  ['/pkg/cratebook.js', ['pkg/cratebook.js', 'text/javascript']],
  ['/pkg/cratebook_bg.wasm', ['pkg/cratebook_bg.wasm', 'application/wasm']],
]);
const args = process.argv.slice(2);
const check = args.length === 1 && args[0] === '--check';
const port = Number(args[1] ?? 4173);
if (!check && args.length && (args.length !== 2 || args[0] !== '--port' || !Number.isInteger(port) || port < 1 || port > 65535)) {
  console.error('Usage: cratebook [--port 4173]');
  process.exit(2);
}
for (const [path] of files.values()) {
  if (!(await stat(new URL(path, root)).catch(() => null))?.size) {
    console.error('Missing gallery asset: ' + path + '. Run pnpm build before serving or packing.');
    process.exit(1);
  }
}
if (!check) {
  const server = createServer((req, res) => {
    const entry = files.get(req.url.split('?')[0]);
    if (!entry || !['GET','HEAD'].includes(req.method)) {
      res.writeHead(entry ? 405 : 404).end();
      return;
    }
    res.writeHead(200, { 'Content-Type': entry[1] });
    if (req.method === 'HEAD') return res.end();
    createReadStream(new URL(entry[0], root)).on('error', () => res.destroy()).pipe(res);
  });
  server.on('error', error => { console.error(error.message); process.exitCode = 1; });
  server.listen(port, '127.0.0.1', () => console.log('Cratebook: http://127.0.0.1:' + port));
}
