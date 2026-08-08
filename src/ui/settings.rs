use eframe::egui;
use crate::config::Config;

pub fn render(
    ui: &mut egui::Ui,
    config: &mut Config,
) {
    ui.heading("Settings");
    ui.separator();

    // Network limits
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label("Traffic Limits");
        
        ui.horizontal(|ui| {
            ui.label("Daily limit (GB):");
            ui.add(egui::DragValue::new(&mut config.network.daily_limit_gb)
                .speed(0.1)
                .range(0.1..=1000.0)
                .suffix(" GB"));
        });
        
        ui.horizontal(|ui| {
            ui.label("Monthly limit (GB):");
            ui.add(egui::DragValue::new(&mut config.network.monthly_limit_gb)
                .speed(1.0)
                .range(1.0..=10000.0)
                .suffix(" GB"));
        });
        
        ui.horizontal(|ui| {
            ui.label("Warning threshold:");
            let mut warning_pct = config.network.warning_threshold * 100.0;
            ui.add(egui::DragValue::new(&mut warning_pct)
                .speed(1.0)
                .range(50.0..=99.0)
                .suffix("%"));
            config.network.warning_threshold = warning_pct / 100.0;
        });
    });

    ui.add_space(8.0);

    // Alerts
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label("Alerts");
        
        ui.checkbox(&mut config.alerts.enabled, "Enable alerts");
        ui.checkbox(&mut config.alerts.notification_sound, "Notification sound");
        ui.checkbox(&mut config.alerts.auto_disconnect_on_limit, "Auto-disconnect on limit");
    });

    ui.add_space(8.0);

    // General
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label("General");
        
        ui.checkbox(&mut config.general.start_with_windows, "Start with Windows");
        ui.checkbox(&mut config.general.minimize_to_tray, "Minimize to tray");
        ui.checkbox(&mut config.interface.show_gpu, "Show GPU info");
    });

    ui.add_space(8.0);

    // Interface
    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label("Interface");
        
        ui.horizontal(|ui| {
            ui.label("Theme:");
            ui.selectable_value(&mut config.interface.theme, "dark".to_string(), "Dark");
            ui.selectable_value(&mut config.interface.theme, "light".to_string(), "Light");
        });
    });

    ui.add_space(16.0);

    ui.horizontal(|ui| {
        if ui.button("Save").clicked() {
            config.save();
            set_autostart(config.general.start_with_windows);
        }
        
        if ui.button("Reset to defaults").clicked() {
            *config = Config::default();
            config.save();
        }
    });
}

#[cfg(target_os = "windows")]
fn set_autostart(enable: bool) {
    use winreg::enums::*;
    use winreg::RegKey;
    
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = r"Software\Microsoft\Windows\CurrentVersion\Run";
    
    if let Ok(key) = hkcu.open_subkey_with_flags(path, KEY_WRITE) {
        if enable {
            if let Ok(exe_path) = std::env::current_exe() {
                if let Some(path_str) = exe_path.to_str() {
                    key.set_value("SysMon", &path_str).ok();
                }
            }
        } else {
            key.delete_value("SysMon").ok();
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn set_autostart(_enable: bool) {
    // Auto-start not implemented for non-Windows platforms
}
