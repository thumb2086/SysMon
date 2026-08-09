use eframe::egui;
use crate::config::Config;
use crate::monitor::SystemInfo;
use crate::monitor::cpu::CpuMonitor;
use crate::storage::Database;
use crate::alerts::{AlertManager, AlertAction};
use crate::tray::TrayManager;
use crate::ui::i18n::I18n;
use chrono::Datelike;
use std::sync::Arc;
use std::collections::VecDeque;

pub struct SysMonApp {
    config: Config,
    sys_info: SystemInfo,
    cpu_monitor: Arc<CpuMonitor>,
    db: Database,
    alert_manager: AlertManager,
    tray: TrayManager,
    i18n: I18n,
    current_tab: Tab,
    network_sent: u64,
    network_recv: u64,
    last_network_sent: u64,
    last_network_recv: u64,
    network_sent_rate: u64,
    network_recv_rate: u64,
    download_history: VecDeque<u64>,
    upload_history: VecDeque<u64>,
    last_record_time: std::time::Instant,
    last_day: chrono::NaiveDate,
    last_threshold_check: std::time::Instant,
    cached_monthly_traffic: Option<(i32, u32, u64)>,
}

#[derive(Debug, Clone, PartialEq)]
enum Tab {
    Dashboard,
    Network,
    Processes,
    Settings,
}

impl SysMonApp {
    pub fn new(config: Config) -> Self {
        let db = Database::new();
        let mut sys_info = SystemInfo::new();
        sys_info.update();
        
        let now = chrono::Utc::now();
        let daily_traffic = db.get_daily_traffic(now.date_naive());
        
        let last_network_sent = sys_info.network_sent;
        let last_network_recv = sys_info.network_received;
        
        let mut download_history = VecDeque::new();
        let mut upload_history = VecDeque::new();
        for _ in 0..60 {
            download_history.push_back(0);
            upload_history.push_back(0);
        }
        
        SysMonApp {
            tray: TrayManager::new(),
            i18n: I18n::new(&config.interface.language),
            alert_manager: AlertManager::new(config.alerts.clone()),
            config,
            cpu_monitor: Arc::new(CpuMonitor::new()),
            sys_info,
            db,
            current_tab: Tab::Dashboard,
            network_sent: daily_traffic.total_sent,
            network_recv: daily_traffic.total_received,
            last_network_sent,
            last_network_recv,
            network_sent_rate: 0,
            network_recv_rate: 0,
            download_history,
            upload_history,
            last_record_time: std::time::Instant::now(),
            last_day: now.date_naive(),
            last_threshold_check: std::time::Instant::now(),
            cached_monthly_traffic: None,
        }
    }

    fn update_network_rates(&mut self) {
        self.sys_info.update_network();
        
        if self.sys_info.network_sent >= self.last_network_sent {
            self.network_sent_rate = self.sys_info.network_sent - self.last_network_sent;
        }
        if self.sys_info.network_received >= self.last_network_recv {
            self.network_recv_rate = self.sys_info.network_received - self.last_network_recv;
        }
        
        self.last_network_sent = self.sys_info.network_sent;
        self.last_network_recv = self.sys_info.network_received;
        
        self.network_sent += self.network_sent_rate;
        self.network_recv += self.network_recv_rate;
        
        self.download_history.push_back(self.network_recv_rate);
        self.upload_history.push_back(self.network_sent_rate);
        if self.download_history.len() > 60 {
            self.download_history.pop_front();
        }
        if self.upload_history.len() > 60 {
            self.upload_history.pop_front();
        }
    }

    fn check_alerts(&mut self) {
        if self.last_threshold_check.elapsed() < std::time::Duration::from_secs(5) {
            return;
        }
        self.last_threshold_check = std::time::Instant::now();
        
        let today = chrono::Utc::now().date_naive();
        if today != self.last_day {
            self.alert_manager.reset_daily_alerts();
            self.last_day = today;
        }
        
        let daily_limit = self.config.daily_limit_bytes();
        let now = chrono::Utc::now();
        let current_year = now.year();
        let current_month = now.month();
        
        let monthly_bytes = if let Some((y, m, bytes)) = &self.cached_monthly_traffic {
            if *y == current_year && *m == current_month {
                *bytes
            } else {
                let traffic = self.db.get_monthly_traffic(current_year, current_month);
                self.cached_monthly_traffic = Some((current_year, current_month, traffic.total_bytes));
                traffic.total_bytes
            }
        } else {
            let traffic = self.db.get_monthly_traffic(current_year, current_month);
            self.cached_monthly_traffic = Some((current_year, current_month, traffic.total_bytes));
            traffic.total_bytes
        };
        
        let monthly_limit = self.config.monthly_limit_bytes();
        
        let action = self.alert_manager.check_thresholds(
            self.network_sent + self.network_recv,
            daily_limit,
            monthly_bytes,
            monthly_limit,
        );
        
        if action == AlertAction::Disconnect {
            self.alert_manager.disconnect_network();
        }
    }

    fn record_traffic(&mut self) {
        if self.last_record_time.elapsed() >= std::time::Duration::from_secs(60) {
            let record = crate::storage::TrafficRecord {
                id: None,
                timestamp: chrono::Utc::now().naive_utc(),
                interface_name: "primary".to_string(),
                bytes_sent: self.network_sent_rate,
                bytes_received: self.network_recv_rate,
                total_bytes: self.network_sent_rate + self.network_recv_rate,
            };
            self.db.insert_traffic(&record);
            self.last_record_time = std::time::Instant::now();
        }
    }
}

impl eframe::App for SysMonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(500));
        
        self.update_network_rates();
        self.check_alerts();
        self.record_traffic();

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("SysMon");
                ui.separator();
                
                if ui.selectable_label(self.current_tab == Tab::Dashboard, self.i18n.t("dashboard")).clicked() {
                    self.current_tab = Tab::Dashboard;
                }
                if ui.selectable_label(self.current_tab == Tab::Network, self.i18n.t("network")).clicked() {
                    self.current_tab = Tab::Network;
                }
                if ui.selectable_label(self.current_tab == Tab::Processes, self.i18n.t("processes")).clicked() {
                    self.current_tab = Tab::Processes;
                }
                if ui.selectable_label(self.current_tab == Tab::Settings, self.i18n.t("settings")).clicked() {
                    self.current_tab = Tab::Settings;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Dashboard => crate::ui::dashboard::render(
                    ui,
                    &self.cpu_monitor,
                    &self.sys_info,
                    &self.db,
                    &self.config,
                    &self.i18n,
                    self.network_sent_rate,
                    self.network_recv_rate,
                    &self.download_history,
                    &self.upload_history,
                ),
                Tab::Network => crate::ui::network::render(
                    ui,
                    &self.db,
                    &self.config,
                    &self.i18n,
                ),
                Tab::Processes => crate::ui::processes::render(
                    ui,
                    &self.cpu_monitor,
                    &self.sys_info,
                ),
                Tab::Settings => crate::ui::settings::render(
                    ui,
                    &mut self.config,
                    &self.i18n,
                ),
            }
        });
    }
}
