// GraphTool Gantt — browser glue.
// Loads gantt_wasm.wasm and re-renders the chart on every keystroke.

const encoder = new TextEncoder();
const decoder = new TextDecoder();

let wasm = null;

// Bump allocator allocating downward from the top of WASM linear memory.
// The Rust module uses only stack + statics (no allocator in the hot path),
// so anything beyond a safety floor is free for us.
const PAGE = 65536;
let top = 0;
const FLOOR_PAGES = 16; // leave the first 16 pages to the Rust module

function wasmMalloc(len) {
  const aligned = (len + 15) & ~15;
  if (top === 0) {
    top = wasm.memory.buffer.byteLength;
  }
  const floor = FLOOR_PAGES * PAGE;
  if (top - aligned < floor) {
    const needPages = Math.ceil((floor + aligned - top) / PAGE);
    wasm.memory.grow(needPages);
  }
  top -= aligned;
  return top;
}

function renderNow() {
  const editor = document.getElementById("editor");
  const preview = document.getElementById("preview");

  const src = encoder.encode(editor.value);
  const srcPtr = wasmMalloc(src.length);
  new Uint8Array(wasm.memory.buffer, srcPtr, src.length).set(src);

  const cap = wasm.output_capacity_for(src.length);
  const outPtr = wasmMalloc(cap);
  const written = wasm.render(srcPtr, src.length, outPtr, cap);

  if (written === -1 || written === 4294967295) {
    preview.textContent = "render error";
    return;
  }
  const bytes = new Uint8Array(wasm.memory.buffer, outPtr, written);
  preview.innerHTML = decoder.decode(bytes);
}

async function init() {
  const { instance } = await WebAssembly.instantiateStreaming(
    fetch("./gantt_wasm.wasm"),
    {}
  );
  wasm = instance.exports;
  const editor = document.getElementById("editor");
  editor.addEventListener("input", renderNow);
  renderNow();
}

init();
