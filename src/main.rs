#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod monitor;
mod storage;
mod alerts;
mod tray;
mod ui;

use single_instance::SingleInstance;

fn main() {
    let instance = SingleInstance::new("SysMon_SingleInstance").unwrap();
    if !instance.is_single() {
        eprintln!("SysMon is already running!");
        return;
    }

    let config = config::Config::load();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 600.0])
            .with_min_inner_size([400.0, 500.0])
            .with_title("SysMon"),
        ..Default::default()
    };

    eframe::run_native(
        "SysMon",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(app::SysMonApp::new(config)))
        }),
    ).unwrap();
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let _fonts = egui::FontDefinitions::default();
    ctx.set_fonts(egui::FontDefinitions::default());
}
