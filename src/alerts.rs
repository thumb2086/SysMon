use crate::config::AlertConfig;
use crate::monitor::network;

#[derive(Debug, Clone, PartialEq)]
pub enum AlertAction {
    None,
    Warning,
    CriticalWarning,
    Disconnect,
}

pub struct AlertManager {
    config: AlertConfig,
    daily_warning_sent: bool,
    daily_critical_sent: bool,
    monthly_warning_sent: bool,
    monthly_critical_sent: bool,
}

impl AlertManager {
    pub fn new(config: AlertConfig) -> Self {
        AlertManager {
            config,
            daily_warning_sent: false,
            daily_critical_sent: false,
            monthly_warning_sent: false,
            monthly_critical_sent: false,
        }
    }

    pub fn check_thresholds(
        &mut self,
        daily_usage: u64,
        daily_limit: u64,
        monthly_usage: u64,
        monthly_limit: u64,
    ) -> AlertAction {
        if !self.config.enabled {
            return AlertAction::None;
        }

        let daily_pct = daily_usage as f64 / daily_limit as f64;
        let monthly_pct = monthly_usage as f64 / monthly_limit as f64;

        // Check daily limits
        if daily_pct >= 1.0 && self.config.auto_disconnect_on_limit {
            self.send_notification("Daily traffic limit reached. Disconnecting...");
            return AlertAction::Disconnect;
        }

        if daily_pct >= 0.95 && !self.daily_critical_sent {
            self.send_notification("Daily traffic at 95%!");
            self.daily_critical_sent = true;
            return AlertAction::CriticalWarning;
        }

        if daily_pct >= 0.8 && !self.daily_warning_sent {
            self.send_notification("Daily traffic at 80%");
            self.daily_warning_sent = true;
            return AlertAction::Warning;
        }

        // Check monthly limits
        if monthly_pct >= 1.0 && self.config.auto_disconnect_on_limit {
            self.send_notification("Monthly traffic limit reached. Disconnecting...");
            return AlertAction::Disconnect;
        }

        if monthly_pct >= 0.95 && !self.monthly_critical_sent {
            self.send_notification("Monthly traffic at 95%!");
            self.monthly_critical_sent = true;
            return AlertAction::CriticalWarning;
        }

        if monthly_pct >= 0.8 && !self.monthly_warning_sent {
            self.send_notification("Monthly traffic at 80%");
            self.monthly_warning_sent = true;
            return AlertAction::Warning;
        }

        AlertAction::None
    }

    pub fn reset_daily_alerts(&mut self) {
        self.daily_warning_sent = false;
        self.daily_critical_sent = false;
    }

    pub fn reset_monthly_alerts(&mut self) {
        self.monthly_warning_sent = false;
        self.monthly_critical_sent = false;
    }

    fn send_notification(&self, message: &str) {
        if let Err(e) = notify_rust::Notification::new()
            .summary("SysMon")
            .body(message)
            .appname("SysMon")
            .show() 
        {
            eprintln!("Failed to send notification: {}", e);
        }
    }

    pub fn disconnect_network(&self) {
        let interfaces = network::get_network_interfaces();
        for iface in interfaces {
            if iface != "lo" && iface != "Loopback" {
                network::disconnect_network(&iface);
                break;
            }
        }
    }
}
