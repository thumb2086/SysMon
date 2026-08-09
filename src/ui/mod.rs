pub mod dashboard;
pub mod network;
pub mod processes;
pub mod settings;
pub mod i18n;

use eframe::egui;

pub fn progress_bar(ui: &mut egui::Ui, value: f64, color: egui::Color32) {
    let desired_size = egui::vec2(ui.available_width(), 20.0);
    let (response, painter) = ui.allocate_painter(desired_size, egui::Sense::hover());
    
    let rect = response.rect;
    let rounding = egui::Rounding::same(4.0);
    
    let bg_color = egui::Color32::from_rgb(220, 220, 220);
    painter.rect_filled(rect, rounding, bg_color);
    
    let progress_rect = egui::Rect::from_min_size(
        rect.min,
        egui::vec2(rect.width() * value.min(1.0) as f32, rect.height()),
    );
    painter.rect_filled(progress_rect, rounding, color);
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.2} GB", bytes as f64 / 1_000_000_000.0)
    } else if bytes >= 1_000_000 {
        format!("{:.2} MB", bytes as f64 / 1_000_000.0)
    } else if bytes >= 1_000 {
        format!("{:.2} KB", bytes as f64 / 1_000.0)
    } else {
        format!("{} B", bytes)
    }
}

pub fn format_speed(bytes_per_sec: u64) -> String {
    format!("{}/s", format_bytes(bytes_per_sec))
}

pub fn format_bytes_decimal(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.2} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.2} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.2} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}
