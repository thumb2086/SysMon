use tray_icon::{TrayIconBuilder, TrayIconEvent, menu::{Menu, MenuEvent}};
use winit::event_loop::EventLoopProxy;

pub struct TrayManager {
    pub tray_icon: Option<tray_icon::TrayIcon>,
    pub menu_channel: MenuEvent,
}

impl TrayManager {
    pub fn new(event_loop_proxy: EventLoopProxy<UserEvent>) -> Self {
        let menu = Menu::new();
        let menu_channel = MenuEvent::receiver().clone();
        
        let tray_icon = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("SysMon - System Monitor")
            .build()
            .ok();
        
        TrayManager {
            tray_icon,
            menu_channel,
        }
    }

    pub fn show_notification(&self, message: &str) {
        if let Some(tray) = &self.tray_icon {
            // tray_icon doesn't have direct notification support
            // Use notify-rust crate instead
            let _ = notify_rust::Notification::new()
                .summary("SysMon")
                .body(message)
                .show();
        }
    }
}

#[derive(Debug, Clone)]
pub enum UserEvent {
    ShowWindow,
    HideWindow,
    Quit,
}
