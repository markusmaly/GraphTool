//! Native entry point for GraphTool GUI.

/// Application title with the version derived from Cargo.toml at compile time.
pub const APP_TITLE: &str = const {
    let v = env!("CARGO_PKG_VERSION");
    if v.is_empty() { "GraphTool" } else { concat!("GraphTool v", "") }
};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let title = format!("GraphTool v{}", env!("CARGO_PKG_VERSION"));
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title(title.clone()),
        ..Default::default()
    };
    eframe::run_native(
        title,
        options,
        Box::new(|_cc| Ok(Box::new(gt_app::GraphToolApp::new()))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    // On wasm the binary is not used; the browser loads the lib via wasm-bindgen.
}
