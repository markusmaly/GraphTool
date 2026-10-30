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

fn json_escape(s: &str, out: &mut String) {
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let mut buf = [0u16; 2];
                for u in c.encode_utf16(&mut buf) {
                    out.push_str(&format!("\\u{u:04x}"));
                }
            }
            c => out.push(c),
        }
    }
}

/// Writes a JSON array of all built-in examples
/// (`[{"name","title","description","dsl"}, ...]`) into `out`.
/// Returns bytes written (excluding NUL), or usize::MAX on error.
///
/// # Safety
/// `out` must be writable for `out_cap` bytes.
#[no_mangle]
pub unsafe extern "C" fn examples_json(out: *mut u8, out_cap: usize) -> usize {
    if out.is_null() || out_cap == 0 {
        return usize::MAX;
    }
    let mut s = String::from("[");
    for (i, ex) in ganttcore::examples::all().iter().enumerate() {
        if i > 0 {
            s.push(',');
        }
        s.push_str("{\"name\":\"");
        json_escape(ex.name, &mut s);
        s.push_str("\",\"title\":\"");
        json_escape(ex.title, &mut s);
        s.push_str("\",\"description\":\"");
        json_escape(ex.description, &mut s);
        s.push_str("\",\"dsl\":\"");
        json_escape(&ex.dsl, &mut s);
        s.push_str("\"}");
    }
    s.push(']');
    let bytes = s.as_bytes();
    let n = bytes.len().min(out_cap - 1);
    std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, n);
    *out.add(n) = 0;
    n
}

/// Upper bound for the `examples_json` output buffer. JSON escaping can at
/// most turn every byte into a 6-char `\uXXXX` sequence.
#[no_mangle]
pub extern "C" fn examples_json_capacity() -> usize {
    ganttcore::examples::all()
        .iter()
        .map(|e| (e.name.len() + e.title.len() + e.description.len() + e.dsl.len()) * 6 + 64)
        .sum::<usize>()
        .saturating_add(64)
}
