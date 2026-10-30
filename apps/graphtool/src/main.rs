//! GraphTool: grammar-of-graphics data visualisation on decks.
//!
//! Phase 1: the menu structure as stub commands, now in a native egui
//! shell following the DeckCraft application conventions. `--console`
//! prints the menu tree to the terminal instead of opening a window
//! (useful on headless machines and in CI).

mod command;
mod document;
mod shell;

use shell::Shell;

fn main() -> eframe::Result {
    if std::env::args().any(|a| a == "--console") {
        print!("{}", command::menu_tree());
        let reg = command::registry();
        println!("{} commands registered across {} menus (all handlers: stubs)",
            reg.len(), command::MENUS.len());
        return Ok(());
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("GraphTool")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native("GraphTool", options, Box::new(|_cc| Ok(Box::new(Shell::new()))))
}
