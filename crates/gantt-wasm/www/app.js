// GraphTool Gantt — browser glue.
// Loads gantt_wasm.wasm and re-renders the chart on every keystroke.

const encoder = new TextEncoder();
const decoder = new TextDecoder();

let wasm = null;

// Scratch allocator: grow the linear memory first, then hand out the freshly
// added pages. This keeps JS buffers strictly above the Rust heap/stack,
// which lives in the low pages.
const PAGE = 65536;

function wasmMalloc(len) {
  const aligned = (len + 15) & ~15;
  const oldPages = wasm.memory.buffer.byteLength / PAGE;
  wasm.memory.grow(Math.ceil(aligned / PAGE) + 2);
  return oldPages * PAGE;
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

function loadExamples() {
  const cap = wasm.examples_json_capacity();
  const outPtr = wasmMalloc(cap);
  const written = wasm.examples_json(outPtr, cap);
  if (written === -1 || written === 4294967295) {
    return [];
  }
  const bytes = new Uint8Array(wasm.memory.buffer, outPtr, written);
  return JSON.parse(decoder.decode(bytes));
}

function fillExampleMenu(examples) {
  const sel = document.getElementById("examples");
  const desc = document.getElementById("exdesc");
  for (const ex of examples) {
    const opt = document.createElement("option");
    opt.value = ex.name;
    opt.textContent = ex.name + " — " + ex.title;
    sel.appendChild(opt);
  }
  const apply = () => {
    const ex = examples.find((e) => e.name === sel.value);
    if (!ex) return;
    desc.textContent = ex.description;
    document.getElementById("editor").value = ex.dsl;
    renderNow();
  };
  sel.addEventListener("change", apply);
  apply();
}

async function init() {
  const { instance } = await WebAssembly.instantiateStreaming(
    fetch("./gantt_wasm.wasm"),
    {}
  );
  wasm = instance.exports;
  const editor = document.getElementById("editor");
  editor.addEventListener("input", renderNow);
  fillExampleMenu(loadExamples());
  renderNow();
}

init();
