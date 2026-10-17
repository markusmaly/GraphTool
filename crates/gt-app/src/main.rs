//! Native entry point for GraphTool GUI.

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("GraphTool"),
        ..Default::default()
    };
    eframe::run_native(
        "GraphTool",
        options,
        Box::new(|_cc| Ok(Box::new(gt_app::GraphToolApp::new()))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // On wasm the binary is not used; the browser loads the lib via wasm-bindgen.
}
