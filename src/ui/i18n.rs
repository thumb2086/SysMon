#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    ZhTW,
    En,
}

pub struct I18n {
    pub lang: Language,
}

impl I18n {
    pub fn new(lang: &str) -> Self {
        let lang = match lang {
            "en" => Language::En,
            _ => Language::ZhTW,
        };
        I18n { lang }
    }

    pub fn t(&self, key: &str) -> &str {
        match (&self.lang, key) {
            (Language::ZhTW, "dashboard") => "儀表板",
            (Language::ZhTW, "network") => "網路流量",
            (Language::ZhTW, "processes") => "進程",
            (Language::ZhTW, "settings") => "設定",
            (Language::ZhTW, "cpu") => "CPU",
            (Language::ZhTW, "memory") => "記憶體",
            (Language::ZhTW, "gpu") => "GPU",
            (Language::ZhTW, "download") => "下載",
            (Language::ZhTW, "upload") => "上傳",
            (Language::ZhTW, "today") => "今日",
            (Language::ZhTW, "daily_limit") => "每日上限",
            (Language::ZhTW, "monthly_limit") => "每月上限",
            (Language::ZhTW, "traffic_history") => "流量歷史",
            (Language::ZhTW, "cpu_cores") => "CPU 核心",
            (Language::ZhTW, "save") => "儲存",
            (Language::ZhTW, "reset") => "還原預設",
            (Language::ZhTW, "traffic_limits") => "流量限制",
            (Language::ZhTW, "alerts") => "告警",
            (Language::ZhTW, "enable_alerts") => "啟用告警",
            (Language::ZhTW, "notification_sound") => "通知音效",
            (Language::ZhTW, "auto_disconnect") => "達上限自動斷網",
            (Language::ZhTW, "general") => "一般",
            (Language::ZhTW, "start_with_windows") => "開機自動啟動",
            (Language::ZhTW, "minimize_to_tray") => "最小化到系統匣",
            (Language::ZhTW, "show_gpu") => "顯示 GPU",
            (Language::ZhTW, "interface") => "介面",
            (Language::ZhTW, "theme") => "主題",
            (Language::ZhTW, "dark") => "深色",
            (Language::ZhTW, "light") => "淺色",
            (Language::ZhTW, "language") => "語言",
            (Language::ZhTW, "no_data") => "尚無資料",
            (Language::ZhTW, "speed") => "速度",
            (Language::ZhTW, "total") => "總計",
            
            (Language::En, "dashboard") => "Dashboard",
            (Language::En, "network") => "Network",
            (Language::En, "processes") => "Processes",
            (Language::En, "settings") => "Settings",
            (Language::En, "cpu") => "CPU",
            (Language::En, "memory") => "Memory",
            (Language::En, "gpu") => "GPU",
            (Language::En, "download") => "Download",
            (Language::En, "upload") => "Upload",
            (Language::En, "today") => "Today",
            (Language::En, "daily_limit") => "Daily Limit",
            (Language::En, "monthly_limit") => "Monthly Limit",
            (Language::En, "traffic_history") => "Traffic History",
            (Language::En, "cpu_cores") => "CPU Cores",
            (Language::En, "save") => "Save",
            (Language::En, "reset") => "Reset",
            (Language::En, "traffic_limits") => "Traffic Limits",
            (Language::En, "alerts") => "Alerts",
            (Language::En, "enable_alerts") => "Enable Alerts",
            (Language::En, "notification_sound") => "Notification Sound",
            (Language::En, "auto_disconnect") => "Auto-disconnect on Limit",
            (Language::En, "general") => "General",
            (Language::En, "start_with_windows") => "Start with Windows",
            (Language::En, "minimize_to_tray") => "Minimize to Tray",
            (Language::En, "show_gpu") => "Show GPU",
            (Language::En, "interface") => "Interface",
            (Language::En, "theme") => "Theme",
            (Language::En, "dark") => "Dark",
            (Language::En, "light") => "Light",
            (Language::En, "language") => "Language",
            (Language::En, "no_data") => "No data yet",
            (Language::En, "speed") => "Speed",
            (Language::En, "total") => "Total",
            
            _ => key,
        }
    }
}
