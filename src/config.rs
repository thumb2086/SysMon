use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub monitoring: MonitoringConfig,
    pub network: NetworkConfig,
    pub alerts: AlertConfig,
    pub interface: InterfaceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub app_name: String,
    pub start_minimized: bool,
    pub start_with_windows: bool,
    pub minimize_to_tray: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub update_interval_ms: u64,
    pub record_interval_sec: u64,
    pub data_retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub daily_limit_gb: f64,
    pub monthly_limit_gb: f64,
    pub warning_threshold: f64,
    pub critical_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertConfig {
    pub enabled: bool,
    pub notification_sound: bool,
    pub auto_disconnect_on_limit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceConfig {
    pub theme: String,
    pub language: String,
    pub show_gpu: bool,
    pub default_tab: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: GeneralConfig {
                app_name: "SysMon".to_string(),
                start_minimized: false,
                start_with_windows: true,
                minimize_to_tray: true,
            },
            monitoring: MonitoringConfig {
                update_interval_ms: 1000,
                record_interval_sec: 60,
                data_retention_days: 90,
            },
            network: NetworkConfig {
                daily_limit_gb: 5.0,
                monthly_limit_gb: 100.0,
                warning_threshold: 0.8,
                critical_threshold: 0.95,
            },
            alerts: AlertConfig {
                enabled: true,
                notification_sound: true,
                auto_disconnect_on_limit: false,
            },
            interface: InterfaceConfig {
                theme: "dark".to_string(),
                language: "zh-TW".to_string(),
                show_gpu: true,
                default_tab: "dashboard".to_string(),
            },
        }
    }
}

impl Config {
    pub fn config_path() -> PathBuf {
        PathBuf::from("config.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            toml::from_str(&content).unwrap_or_default()
        } else {
            let config = Config::default();
            config.save();
            config
        }
    }

    pub fn save(&self) {
        let content = toml::to_string_pretty(self).unwrap();
        fs::write(Self::config_path(), content).ok();
    }

    pub fn daily_limit_bytes(&self) -> u64 {
        (self.network.daily_limit_gb * 1_073_741_824.0) as u64
    }

    pub fn monthly_limit_bytes(&self) -> u64 {
        (self.network.monthly_limit_gb * 1_073_741_824.0) as u64
    }
}
