// Tray icon support - simplified version
// Full tray support requires more complex event loop integration

pub struct TrayManager {
    #[allow(dead_code)]
    tooltip: String,
}

impl TrayManager {
    pub fn new() -> Self {
        TrayManager {
            tooltip: "SysMon - System Monitor".to_string(),
        }
    }

    pub fn show_notification(&self, message: &str) {
        let _ = notify_rust::Notification::new()
            .summary("SysMon")
            .body(message)
            .show();
    }
}
