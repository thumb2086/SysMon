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

    pub fn t(&self, key: &str) -> String {
        match (&self.lang, key) {
            (Language::ZhTW, "dashboard") => "儀表板".to_string(),
            (Language::ZhTW, "network") => "網路流量".to_string(),
            (Language::ZhTW, "processes") => "進程".to_string(),
            (Language::ZhTW, "settings") => "設定".to_string(),
            (Language::ZhTW, "cpu") => "CPU".to_string(),
            (Language::ZhTW, "memory") => "記憶體".to_string(),
            (Language::ZhTW, "gpu") => "GPU".to_string(),
            (Language::ZhTW, "download") => "下載".to_string(),
            (Language::ZhTW, "upload") => "上傳".to_string(),
            (Language::ZhTW, "today") => "今日".to_string(),
            (Language::ZhTW, "daily_limit") => "每日上限".to_string(),
            (Language::ZhTW, "monthly_limit") => "每月上限".to_string(),
            (Language::ZhTW, "traffic_history") => "流量歷史".to_string(),
            (Language::ZhTW, "cpu_cores") => "CPU 核心".to_string(),
            (Language::ZhTW, "save") => "儲存".to_string(),
            (Language::ZhTW, "reset") => "還原預設".to_string(),
            (Language::ZhTW, "traffic_limits") => "流量限制".to_string(),
            (Language::ZhTW, "alerts") => "告警".to_string(),
            (Language::ZhTW, "enable_alerts") => "啟用告警".to_string(),
            (Language::ZhTW, "notification_sound") => "通知音效".to_string(),
            (Language::ZhTW, "auto_disconnect") => "達上限自動斷網".to_string(),
            (Language::ZhTW, "general") => "一般".to_string(),
            (Language::ZhTW, "start_with_windows") => "開機自動啟動".to_string(),
            (Language::ZhTW, "minimize_to_tray") => "最小化到系統匣".to_string(),
            (Language::ZhTW, "show_gpu") => "顯示 GPU".to_string(),
            (Language::ZhTW, "interface") => "介面".to_string(),
            (Language::ZhTW, "theme") => "主題".to_string(),
            (Language::ZhTW, "dark") => "深色".to_string(),
            (Language::ZhTW, "light") => "淺色".to_string(),
            (Language::ZhTW, "language") => "語言".to_string(),
            (Language::ZhTW, "no_data") => "尚無資料".to_string(),
            (Language::ZhTW, "speed") => "速度".to_string(),
            (Language::ZhTW, "total") => "總計".to_string(),
            
            (Language::En, "dashboard") => "Dashboard".to_string(),
            (Language::En, "network") => "Network".to_string(),
            (Language::En, "processes") => "Processes".to_string(),
            (Language::En, "settings") => "Settings".to_string(),
            (Language::En, "cpu") => "CPU".to_string(),
            (Language::En, "memory") => "Memory".to_string(),
            (Language::En, "gpu") => "GPU".to_string(),
            (Language::En, "download") => "Download".to_string(),
            (Language::En, "upload") => "Upload".to_string(),
            (Language::En, "today") => "Today".to_string(),
            (Language::En, "daily_limit") => "Daily Limit".to_string(),
            (Language::En, "monthly_limit") => "Monthly Limit".to_string(),
            (Language::En, "traffic_history") => "Traffic History".to_string(),
            (Language::En, "cpu_cores") => "CPU Cores".to_string(),
            (Language::En, "save") => "Save".to_string(),
            (Language::En, "reset") => "Reset".to_string(),
            (Language::En, "traffic_limits") => "Traffic Limits".to_string(),
            (Language::En, "alerts") => "Alerts".to_string(),
            (Language::En, "enable_alerts") => "Enable Alerts".to_string(),
            (Language::En, "notification_sound") => "Notification Sound".to_string(),
            (Language::En, "auto_disconnect") => "Auto-disconnect on Limit".to_string(),
            (Language::En, "general") => "General".to_string(),
            (Language::En, "start_with_windows") => "Start with Windows".to_string(),
            (Language::En, "minimize_to_tray") => "Minimize to Tray".to_string(),
            (Language::En, "show_gpu") => "Show GPU".to_string(),
            (Language::En, "interface") => "Interface".to_string(),
            (Language::En, "theme") => "Theme".to_string(),
            (Language::En, "dark") => "Dark".to_string(),
            (Language::En, "light") => "Light".to_string(),
            (Language::En, "language") => "Language".to_string(),
            (Language::En, "no_data") => "No data yet".to_string(),
            (Language::En, "speed") => "Speed".to_string(),
            (Language::En, "total") => "Total".to_string(),
            
            _ => key.to_string(),
        }
    }
}
