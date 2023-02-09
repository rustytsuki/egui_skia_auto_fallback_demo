#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;

pub fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.maximized = true;
    eframe::run_native(
        "Egui Skia Auto Fallback Demo",
        native_options,
        Box::new(|cc| Box::new(app::SkiaApp::new(cc))),
    ).unwrap();
}
