use eframe::egui;
use crate::monitor::SystemInfo;
use crate::monitor::cpu::CpuMonitor;
use crate::storage::Database;
use crate::config::Config;
use super::{progress_bar, format_bytes, format_speed};
use std::sync::Arc;

pub fn render(
    ui: &mut egui::Ui,
    cpu_monitor: &Arc<CpuMonitor>,
    sys_info: &SystemInfo,
    db: &Database,
    config: &Config,
    network_sent_rate: u64,
    network_recv_rate: u64,
) {
    ui.heading("Dashboard");
    ui.separator();

    let cpu_usage = cpu_monitor.get_usage();
    let avg_cpu = if cpu_usage.is_empty() {
        0.0
    } else {
        cpu_usage.iter().sum::<f32>() / cpu_usage.len() as f32
    };

    ui.columns(2, |cols| {
        cols[0].group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.label("CPU");
            let cpu_color = if avg_cpu >= 80.0 {
                egui::Color32::from_rgb(243, 139, 168)
            } else if avg_cpu >= 50.0 {
                egui::Color32::from_rgb(249, 226, 175)
            } else {
                egui::Color32::from_rgb(166, 227, 161)
            };
            progress_bar(ui, avg_cpu as f64 / 100.0, cpu_color);
            ui.label(format!("{:.1}%", avg_cpu));
        });

        cols[1].group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.label("Memory");
            let mem_pct = if sys_info.memory_total > 0 {
                sys_info.memory_used as f64 / sys_info.memory_total as f64
            } else {
                0.0
            };
            let mem_color = if mem_pct >= 0.8 {
                egui::Color32::from_rgb(243, 139, 168)
            } else if mem_pct >= 0.5 {
                egui::Color32::from_rgb(249, 226, 175)
            } else {
                egui::Color32::from_rgb(166, 227, 161)
            };
            progress_bar(ui, mem_pct, mem_color);
            ui.label(format!("{} / {}", 
                format_bytes(sys_info.memory_used),
                format_bytes(sys_info.memory_total)
            ));
        });
    });

    ui.add_space(8.0);

    if config.interface.show_gpu {
        if let Some(gpu_usage) = sys_info.gpu_usage {
            ui.group(|ui| {
                ui.set_min_width(ui.available_width());
                ui.label("GPU");
                let gpu_color = if gpu_usage >= 80.0 {
                    egui::Color32::from_rgb(243, 139, 168)
                } else if gpu_usage >= 50.0 {
                    egui::Color32::from_rgb(249, 226, 175)
                } else {
                    egui::Color32::from_rgb(166, 227, 161)
                };
                progress_bar(ui, gpu_usage as f64 / 100.0, gpu_color);
                if let (Some(used), Some(total)) = (sys_info.gpu_memory_used, sys_info.gpu_memory_total) {
                    ui.label(format!("{} / {}", format_bytes(used), format_bytes(total)));
                } else {
                    ui.label(format!("{:.1}%", gpu_usage));
                }
            });
            ui.add_space(8.0);
        }
    }

    // Network Traffic Today
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.heading("Network Traffic (Today)");
        
        let today = chrono::Utc::now().date_naive();
        let daily_traffic = db.get_daily_traffic(today);
        let daily_limit = config.daily_limit_bytes();
        
        let daily_pct = if daily_limit > 0 {
            daily_traffic.total_bytes as f64 / daily_limit as f64
        } else {
            0.0
        };
        
        ui.horizontal(|ui| {
            ui.label("Download:");
            ui.colored_label(egui::Color32::from_rgb(166, 227, 161), format_speed(network_recv_rate));
        });
        
        ui.horizontal(|ui| {
            ui.label("Upload:");
            ui.colored_label(egui::Color32::from_rgb(249, 226, 175), format_speed(network_sent_rate));
        });
        
        ui.add_space(4.0);
        
        let color = if daily_pct >= 0.95 {
            egui::Color32::from_rgb(243, 139, 168)
        } else if daily_pct >= 0.8 {
            egui::Color32::from_rgb(249, 226, 175)
        } else {
            egui::Color32::from_rgb(166, 227, 161)
        };
        progress_bar(ui, daily_pct.min(1.0), color);
        
        ui.horizontal(|ui| {
            ui.label(format!("{} / {}", 
                format_bytes(daily_traffic.total_bytes),
                format_bytes(daily_limit)
            ));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(format!("{:.1}%", daily_pct * 100.0));
            });
        });
    });

    ui.add_space(8.0);

    // Traffic History (7 days)
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.heading("Traffic History (7 days)");
        
        let history = db.get_traffic_history(7);
        
        if history.is_empty() || history.iter().all(|d| d.total_bytes == 0) {
            ui.label("No traffic data recorded yet");
        } else {
            let max_bytes = history.iter()
                .map(|d| d.total_bytes)
                .max()
                .unwrap_or(1) as f32;
            
            let available_width = ui.available_width();
            let bar_width = (available_width / 7.0).min(50.0);
            
            ui.vertical(|ui| {
                for day in &history {
                    let pct = if max_bytes > 0.0 {
                        day.total_bytes as f32 / max_bytes
                    } else {
                        0.0
                    };
                    
                    let bar_height = pct * 30.0;
                    let color = if day.total_bytes as f64 > config.daily_limit_bytes() as f64 * 0.8 {
                        egui::Color32::from_rgb(243, 139, 168)  // Red if near limit
                    } else {
                        egui::Color32::from_rgb(137, 180, 250)  // Blue
                    };
                    
                    ui.horizontal(|ui| {
                        // Date label
                        ui.label(day.date.format("%m/%d").to_string());
                        
                        // Bar
                        let (response, painter) = ui.allocate_painter(
                            egui::vec2(100.0, 20.0),
                            egui::Sense::hover()
                        );
                        
                        let rect = response.rect;
                        let bg_rect = egui::Rect::from_min_size(
                            rect.min,
                            egui::vec2(100.0, 16.0)
                        );
                        painter.rect_filled(bg_rect, egui::Rounding::same(4.0), egui::Color32::from_gray(200));
                        
                        let bar_rect = egui::Rect::from_min_size(
                            rect.min,
                            egui::vec2(100.0 * pct.min(1.0), 16.0)
                        );
                        painter.rect_filled(bar_rect, egui::Rounding::same(4.0), color);
                        
                        // Amount
                        ui.label(format!("{}", format_bytes(day.total_bytes)));
                    });
                }
            });
        }
    });

    ui.add_space(8.0);

    // CPU cores
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label("CPU Cores");
        
        let num_cores = cpu_usage.len();
        let cols = (num_cores as f32).sqrt().ceil() as usize;
        let cols = cols.max(1);
        ui.columns(cols, |columns| {
            for (i, usage) in cpu_usage.iter().enumerate() {
                let col_idx = i % cols;
                columns[col_idx].horizontal(|ui| {
                    ui.label(format!("C{}", i));
                    let color = if *usage >= 80.0 {
                        egui::Color32::from_rgb(243, 139, 168)
                    } else if *usage >= 50.0 {
                        egui::Color32::from_rgb(249, 226, 175)
                    } else {
                        egui::Color32::from_rgb(166, 227, 161)
                    };
                    progress_bar(ui, *usage as f64 / 100.0, color);
                    ui.label(format!("{:.0}%", usage));
                });
            }
        });
    });
}
