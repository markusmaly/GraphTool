// Node smoke test: loads the WASM module the same way the browser does
// and renders the sample chart.
import { readFileSync } from "node:fs";

const PAGE = 65536;
let top = 0;
const FLOOR_PAGES = 16;

const wasmBinary = readFileSync(new URL("../../../target/wasm32-unknown-unknown/release/gantt_wasm.wasm", import.meta.url));
const { instance } = await WebAssembly.instantiate(wasmBinary, {});
const wasm = instance.exports;

function wasmMalloc(len) {
  const aligned = (len + 15) & ~15;
  if (top === 0) top = wasm.memory.buffer.byteLength;
  const floor = FLOOR_PAGES * PAGE;
  if (top - aligned < floor) {
    const needPages = Math.ceil((floor + aligned - top) / PAGE);
    wasm.memory.grow(needPages);
  }
  top -= aligned;
  return top;
}

const src = Buffer.from(readFileSync(new URL("../../../examples/simple.gantt", import.meta.url), "utf8"), "utf8");
const srcPtr = wasmMalloc(src.length);
new Uint8Array(wasm.memory.buffer, srcPtr, src.length).set(src);

const cap = wasm.output_capacity_for(src.length);
const outPtr = wasmMalloc(cap);
const written = wasm.render(srcPtr, src.length, outPtr, cap);
if (written === -1 || written === 4294967295) {
  console.error("FAIL: render returned error");
  process.exit(1);
}
const out = Buffer.from(wasm.memory.buffer, outPtr, written).toString("utf8");
if (!out.startsWith("<svg") || !out.includes("Prototype design")) {
  console.error("FAIL: unexpected SVG output:\n" + out.slice(0, 200));
  process.exit(1);
}
console.log("OK: rendered", written, "bytes of SVG");
