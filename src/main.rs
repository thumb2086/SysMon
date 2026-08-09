#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod config;
mod monitor;
mod storage;
mod alerts;
mod ui;

use single_instance::SingleInstance;
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG: &str = include_str!("config_default.toml");

fn main() {
    let instance = SingleInstance::new("SysMon_SingleInstance").unwrap();
    if !instance.is_single() {
        eprintln!("SysMon is already running!");
        return;
    }

    // Ensure data directory exists
    let data_dir = PathBuf::from("data");
    fs::create_dir_all(&data_dir).ok();

    // Ensure config exists
    let config_path = PathBuf::from("config.toml");
    if !config_path.exists() {
        fs::write(&config_path, DEFAULT_CONFIG).ok();
    }

    let config = config::Config::load();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 650.0])
            .with_min_inner_size([400.0, 500.0])
            .with_decorations(true)
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "SysMon",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_fonts(egui::FontDefinitions::default());
            Ok(Box::new(app::SysMonApp::new(config)))
        }),
    ).unwrap();
}
