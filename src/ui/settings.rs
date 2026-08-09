use eframe::egui;
use crate::config::Config;
use crate::ui::i18n::I18n;

pub fn render(
    ui: &mut egui::Ui,
    config: &mut Config,
    i18n: &I18n,
) {
    ui.heading(i18n.t("settings"));
    ui.separator();

    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label(i18n.t("traffic_limits"));
        
        ui.horizontal(|ui| {
            ui.label(format!("{} (GB):", i18n.t("daily_limit")));
            ui.add(egui::DragValue::new(&mut config.network.daily_limit_gb)
                .speed(0.1)
                .range(0.1..=1000.0)
                .suffix(" GB"));
        });
        
        ui.horizontal(|ui| {
            ui.label(format!("{} (GB):", i18n.t("monthly_limit")));
            ui.add(egui::DragValue::new(&mut config.network.monthly_limit_gb)
                .speed(1.0)
                .range(1.0..=10000.0)
                .suffix(" GB"));
        });
        
        ui.horizontal(|ui| {
            ui.label(format!("{}:", i18n.t("alerts")));
            let mut warning_pct = config.network.warning_threshold * 100.0;
            ui.add(egui::DragValue::new(&mut warning_pct)
                .speed(1.0)
                .range(50.0..=99.0)
                .suffix("%"));
            config.network.warning_threshold = warning_pct / 100.0;
        });
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label(i18n.t("alerts"));
        
        ui.checkbox(&mut config.alerts.enabled, i18n.t("enable_alerts"));
        ui.checkbox(&mut config.alerts.notification_sound, i18n.t("notification_sound"));
        ui.checkbox(&mut config.alerts.auto_disconnect_on_limit, i18n.t("auto_disconnect"));
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label(i18n.t("general"));
        
        ui.checkbox(&mut config.general.start_with_windows, i18n.t("start_with_windows"));
        ui.checkbox(&mut config.general.minimize_to_tray, i18n.t("minimize_to_tray"));
        ui.checkbox(&mut config.interface.show_gpu, i18n.t("show_gpu"));
    });

    ui.add_space(8.0);

    ui.group(|ui| {
        ui.set_min_width(ui.available_width());
        ui.label(i18n.t("interface"));
        
        ui.horizontal(|ui| {
            ui.label(format!("{}:", i18n.t("theme")));
            ui.selectable_value(&mut config.interface.theme, "dark".to_string(), i18n.t("dark"));
            ui.selectable_value(&mut config.interface.theme, "light".to_string(), i18n.t("light"));
        });
        
        ui.horizontal(|ui| {
            ui.label(format!("{}:", i18n.t("language")));
            ui.selectable_value(&mut config.interface.language, "zh-TW".to_string(), "中文");
            ui.selectable_value(&mut config.interface.language, "en".to_string(), "English");
        });
    });

    ui.add_space(16.0);

    ui.horizontal(|ui| {
        if ui.button(i18n.t("save")).clicked() {
            config.save();
            set_autostart(config.general.start_with_windows);
        }
        
        if ui.button(i18n.t("reset")).clicked() {
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
}
