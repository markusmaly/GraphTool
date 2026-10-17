//! WebAssembly entry point (wasm-bindgen based, mirrors eframe's documented
//! web usage).

use wasm_bindgen::prelude::*;

/// Handle to the web app from JavaScript.
#[derive(Clone)]
#[wasm_bindgen]
pub struct WebHandle {
    runner: eframe::WebRunner,
}

#[wasm_bindgen]
impl WebHandle {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        eframe::WebLogger::init(log::LevelFilter::Debug).ok();
        Self { runner: eframe::WebRunner::new() }
    }

    /// Call this once from JavaScript to start the app.
    pub async fn start(&self, canvas: web_sys::HtmlCanvasElement) -> Result<(), JsValue> {
        self.runner
            .start(
                canvas,
                eframe::WebOptions::default(),
                Box::new(|_cc| Ok(Box::new(crate::app::GraphToolApp::new()))),
            )
            .await
    }

    pub fn destroy(&self) {
        self.runner.destroy();
    }
}
