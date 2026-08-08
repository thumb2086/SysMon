use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub fn get_cpu_usage(sys: &mut sysinfo::System) -> Vec<f32> {
    sys.refresh_cpu_all();
    sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
}

pub fn get_cpu_info(sys: &sysinfo::System) -> (String, u32) {
    let brand = sys.cpus().first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let cores = sys.cpus().len() as u32;
    (brand, cores)
}

/// Background CPU monitor that updates without blocking UI
pub struct CpuMonitor {
    usage: Arc<Mutex<Vec<f32>>>,
    last_update: Arc<Mutex<Instant>>,
}

impl CpuMonitor {
    pub fn new() -> Self {
        let usage = Arc::new(Mutex::new(Vec::new()));
        let last_update = Arc::new(Mutex::new(Instant::now()));
        
        let usage_clone = usage.clone();
        let last_update_clone = last_update.clone();
        
        thread::spawn(move || {
            let mut sys = sysinfo::System::new();
            loop {
                sys.refresh_cpu_all();
                thread::sleep(Duration::from_millis(100));
                sys.refresh_cpu_all();
                
                let cpu_usage: Vec<f32> = sys.cpus().iter().map(|c| c.cpu_usage()).collect();
                
                if let Ok(mut u) = usage_clone.lock() {
                    *u = cpu_usage;
                }
                if let Ok(mut t) = last_update_clone.lock() {
                    *t = Instant::now();
                }
            }
        });
        
        CpuMonitor { usage, last_update }
    }
    
    pub fn get_usage(&self) -> Vec<f32> {
        self.usage.lock().unwrap().clone()
    }
    
    pub fn last_update(&self) -> Instant {
        *self.last_update.lock().unwrap()
    }
}
