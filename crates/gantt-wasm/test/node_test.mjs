// Node smoke test: loads the WASM module the same way the browser does
// and renders the sample chart.
import { readFileSync } from "node:fs";

const PAGE = 65536;
let top = 0;

const wasmBinary = readFileSync(new URL("../../../target/wasm32-unknown-unknown/release/gantt_wasm.wasm", import.meta.url));
const { instance } = await WebAssembly.instantiate(wasmBinary, {});
const wasm = instance.exports;

function wasmMalloc(len) {
  const aligned = (len + 15) & ~15;
  // Grow first, then hand out the freshly added pages. Growing before
  // computing the pointer keeps the Rust heap (which lives in the low
  // pages) strictly separated from JS scratch buffers.
  const oldPages = wasm.memory.buffer.byteLength / PAGE;
  const needPages = Math.ceil(aligned / PAGE) + 2;
  wasm.memory.grow(needPages);
  const ptr = oldPages * PAGE;
  top = ptr + aligned;
  return ptr;
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

// examples_json: list of built-in ggplot2-dataset examples must be valid JSON
// and every example DSL must render to SVG.
const jcap = wasm.examples_json_capacity();
const jsonPtr = wasmMalloc(jcap);
const jsonLen = wasm.examples_json(jsonPtr, jcap);
if (jsonLen === -1 || jsonLen === 4294967295) {
  console.error("FAIL: examples_json returned error");
  process.exit(1);
}
const examples = JSON.parse(Buffer.from(wasm.memory.buffer, jsonPtr, jsonLen).toString("utf8"));
if (!Array.isArray(examples) || examples.length < 8) {
  console.error("FAIL: expected >= 8 examples, got", examples.length);
  process.exit(1);
}
for (const name of ["mtcars", "diamonds", "mpg", "presidential"]) {
  if (!examples.some((e) => e.name.includes(name))) {
    console.error("FAIL: missing example containing", name);
    process.exit(1);
  }
}
for (const ex of examples) {
  const src = Buffer.from(ex.dsl, "utf8");
  const srcPtr = wasmMalloc(src.length);
  new Uint8Array(wasm.memory.buffer, srcPtr, src.length).set(src);
  const ocap = wasm.output_capacity_for(src.length);
  const op = wasmMalloc(ocap);
  const w = wasm.render(srcPtr, src.length, op, ocap);
  if (w === -1 || w === 4294967295) {
    console.error("FAIL: example", ex.name, "render error");
    process.exit(1);
  }
  const svg = Buffer.from(wasm.memory.buffer, op, w).toString("utf8");
  if (!svg.startsWith("<svg")) {
    console.error("FAIL: example", ex.name, "did not render SVG");
    process.exit(1);
  }
}
console.log("OK:", examples.length, "examples listed and rendered:", examples.map((e) => e.name).join(", "));
