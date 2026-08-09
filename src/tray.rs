use std::sync::{Arc, Mutex};

pub enum UserEvent {
    Show,
    Hide,
    Quit,
}

pub struct TrayManager {
    pub show_flag: Arc<Mutex<bool>>,
}

impl TrayManager {
    pub fn new() -> Self {
        TrayManager {
            show_flag: Arc::new(Mutex::new(true)),
        }
    }

    pub fn show_notification(&self, title: &str, _message: &str) {
        #[cfg(target_os = "windows")]
        {
            let _ = notify_rust::Notification::new()
                .summary(title)
                .body(_message)
                .appname("SysMon")
                .show();
        }
    }
}
