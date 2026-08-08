use eframe::egui;
use crate::config::Config;
use crate::monitor::SystemInfo;
use crate::storage::Database;
use crate::alerts::{AlertManager, AlertAction};

pub struct SysMonApp {
    config: Config,
    sys_info: SystemInfo,
    db: Database,
    alert_manager: AlertManager,
    current_tab: Tab,
    network_sent: u64,
    network_recv: u64,
    last_network_sent: u64,
    last_network_recv: u64,
    network_sent_rate: u64,
    network_recv_rate: u64,
    last_record_time: std::time::Instant,
    last_day: chrono::NaiveDate,
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
        let monthly_traffic = db.get_monthly_traffic(now.year(), now.month());
        
        SysMonApp {
            alert_manager: AlertManager::new(config.alerts.clone()),
            config,
            sys_info,
            db,
            current_tab: Tab::Dashboard,
            network_sent: daily_traffic.total_sent,
            network_recv: daily_traffic.total_received,
            last_network_sent: sys_info.network_sent,
            last_network_recv: sys_info.network_received,
            network_sent_rate: 0,
            network_recv_rate: 0,
            last_record_time: std::time::Instant::now(),
            last_day: now.date_naive(),
        }
    }

    fn update_system_info(&mut self) {
        self.sys_info.update();
        
        // Calculate network rates
        if self.sys_info.network_sent >= self.last_network_sent {
            self.network_sent_rate = self.sys_info.network_sent - self.last_network_sent;
        }
        if self.sys_info.network_received >= self.last_network_recv {
            self.network_recv_rate = self.sys_info.network_received - self.last_network_recv;
        }
        
        self.last_network_sent = self.sys_info.network_sent;
        self.last_network_recv = self.sys_info.network_received;
        
        // Update daily totals
        self.network_sent += self.network_sent_rate;
        self.network_recv += self.network_recv_rate;
        
        // Check for day change
        let today = chrono::Utc::now().date_naive();
        if today != self.last_day {
            self.alert_manager.reset_daily_alerts();
            self.last_day = today;
        }
        
        // Check thresholds
        let daily_limit = self.config.daily_limit_bytes();
        let monthly_traffic = self.db.get_monthly_traffic(
            chrono::Utc::now().year(),
            chrono::Utc::now().month()
        );
        let monthly_limit = self.config.monthly_limit_bytes();
        
        let action = self.alert_manager.check_thresholds(
            self.network_sent + self.network_recv,
            daily_limit,
            monthly_traffic.total_bytes,
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
        // Update system info periodically
        ctx.request_repaint_after(std::time::Duration::from_millis(
            self.config.monitoring.update_interval_ms
        ));
        
        self.update_system_info();
        self.record_traffic();

        // Top panel with tabs
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("SysMon");
                ui.separator();
                
                if ui.selectable_label(self.current_tab == Tab::Dashboard, "Dashboard").clicked() {
                    self.current_tab = Tab::Dashboard;
                }
                if ui.selectable_label(self.current_tab == Tab::Network, "Network").clicked() {
                    self.current_tab = Tab::Network;
                }
                if ui.selectable_label(self.current_tab == Tab::Processes, "Processes").clicked() {
                    self.current_tab = Tab::Processes;
                }
                if ui.selectable_label(self.current_tab == Tab::Settings, "Settings").clicked() {
                    self.current_tab = Tab::Settings;
                }
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                Tab::Dashboard => crate::ui::dashboard::render(
                    ui,
                    &self.sys_info,
                    &self.db,
                    &self.config,
                    self.network_sent_rate,
                    self.network_recv_rate,
                ),
                Tab::Network => crate::ui::network::render(
                    ui,
                    &self.db,
                    &self.config,
                ),
                Tab::Processes => crate::ui::processes::render(
                    ui,
                    &self.sys_info,
                ),
                Tab::Settings => crate::ui::settings::render(
                    ui,
                    &mut self.config,
                ),
            }
        });
    }
}
