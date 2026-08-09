use eframe::egui;
use crate::storage::Database;
use crate::config::Config;
use crate::ui::i18n::I18n;
use super::{progress_bar, format_bytes};
use chrono::Datelike;

pub fn render(
    ui: &mut egui::Ui,
    db: &Database,
    config: &Config,
    i18n: &I18n,
) {
    ui.heading(i18n.t("network"));
    ui.separator();

    ui.add_space(8.0);

    let today = chrono::Utc::now().date_naive();
    let daily_traffic = db.get_daily_traffic(today);
    let daily_limit = config.daily_limit_bytes();
    
    let now = chrono::Utc::now();
    let monthly_traffic = db.get_monthly_traffic(now.year(), now.month());
    let monthly_limit = config.monthly_limit_bytes();

    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.heading(format!("{} ({})", i18n.t("traffic_history"), i18n.t("today")));
        
        let daily_pct = if daily_limit > 0 {
            daily_traffic.total_bytes as f64 / daily_limit as f64
        } else {
            0.0
        };
        
        let color = if daily_pct >= 0.95 {
            egui::Color32::from_rgb(243, 139, 168)
        } else if daily_pct >= 0.8 {
            egui::Color32::from_rgb(249, 226, 175)
        } else {
            egui::Color32::from_rgb(166, 227, 161)
        };
        
        progress_bar(ui, daily_pct.min(1.0), color);
        
        ui.horizontal(|ui| {
            ui.label(format!("{}: {}", i18n.t("download"), format_bytes(daily_traffic.total_received)));
            ui.separator();
            ui.label(format!("{}: {}", i18n.t("upload"), format_bytes(daily_traffic.total_sent)));
        });
        
        ui.label(format!("{}: {} / {} ({:.1}%)", 
            i18n.t("total"),
            format_bytes(daily_traffic.total_bytes),
            format_bytes(daily_limit),
            daily_pct * 100.0
        ));
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.heading(format!("{} ({})", i18n.t("traffic_history"), i18n.t("total")));
        
        let monthly_pct = if monthly_limit > 0 {
            monthly_traffic.total_bytes as f64 / monthly_limit as f64
        } else {
            0.0
        };
        
        let color = if monthly_pct >= 0.95 {
            egui::Color32::from_rgb(243, 139, 168)
        } else if monthly_pct >= 0.8 {
            egui::Color32::from_rgb(249, 226, 175)
        } else {
            egui::Color32::from_rgb(166, 227, 161)
        };
        
        progress_bar(ui, monthly_pct.min(1.0), color);
        
        ui.horizontal(|ui| {
            ui.label(format!("{}: {}", i18n.t("download"), format_bytes(monthly_traffic.total_received)));
            ui.separator();
            ui.label(format!("{}: {}", i18n.t("upload"), format_bytes(monthly_traffic.total_sent)));
        });
        
        ui.label(format!("{}: {} / {} ({:.1}%)", 
            i18n.t("total"),
            format_bytes(monthly_traffic.total_bytes),
            format_bytes(monthly_limit),
            monthly_pct * 100.0
        ));
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.heading(format!("{} (7 {})", i18n.t("traffic_history"), i18n.t("total")));
        
        let history = db.get_traffic_history(7);
        
        if history.is_empty() || history.iter().all(|d| d.total_bytes == 0) {
            ui.label(i18n.t("no_data"));
        } else {
            let max_bytes = history.iter()
                .map(|d| d.total_bytes)
                .max()
                .unwrap_or(1) as f32;
            
            let available_width = ui.available_width();
            let bar_width = (available_width / 7.0).min(40.0);
            
            ui.horizontal(|ui| {
                for day in &history {
                    ui.vertical(|ui| {
                        let bar_height = if max_bytes > 0.0 {
                            (day.total_bytes as f32 / max_bytes) * 150.0
                        } else {
                            0.0
                        };
                        let (response, painter) = ui.allocate_painter(
                            egui::vec2(bar_width, 150.0),
                            egui::Sense::hover()
                        );
                        
                        let rect = response.rect;
                        let bar_rect = egui::Rect::from_min_size(
                            egui::pos2(rect.min.x, rect.max.y - bar_height),
                            egui::vec2(bar_width - 4.0, bar_height)
                        );
                        
                        painter.rect_filled(bar_rect, egui::Rounding::same(2.0), egui::Color32::from_rgb(137, 180, 250));
                        
                        ui.label(day.date.format("%m/%d").to_string());
                    });
                }
            });
        }
    });
}
