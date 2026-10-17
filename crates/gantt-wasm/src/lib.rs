//! WASM binding: flat byte-buffer ABI, no JS imports required.
//!
//! JS usage:
//!   const svgLen = wasm.render(srcPtr, srcLen, outPtr, outCap);
//! Strings are UTF-8; `render` returns the number of bytes written,
//! or -1 (usize::MAX) on error.

/// Renders UTF-8 DSL input into the provided output buffer.
/// Returns bytes written (excluding NUL), or usize::MAX on invalid input.
///
/// # Safety
/// `src` must be readable for `src_len` bytes; `out` writable for `out_cap`.
#[no_mangle]
pub unsafe extern "C" fn render(src: *const u8, src_len: usize, out: *mut u8, out_cap: usize) -> usize {
    if src.is_null() || out.is_null() || out_cap == 0 {
        return usize::MAX;
    }
    let input = match std::str::from_utf8(std::slice::from_raw_parts(src, src_len)) {
        Ok(s) => s,
        Err(_) => return usize::MAX,
    };
    let result = ganttcore::render_source_to_svg(input);
    let svg = match result {
        Ok(s) => s,
        Err(e) => format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"420\" height=\"60\"><text x=\"10\" y=\"30\" fill=\"red\">error: {}</text></svg>",
            e.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
        ),
    };
    let bytes = svg.as_bytes();
    let n = bytes.len().min(out_cap - 1);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, n);
    *out.add(n) = 0;
    n
}

/// Returns a conservative upper bound for the output buffer size
/// for a given input length (heuristic: 16x input + 4 KiB).
#[no_mangle]
pub extern "C" fn output_capacity_for(input_len: usize) -> usize {
    input_len.saturating_mul(16).saturating_add(4096)
}
