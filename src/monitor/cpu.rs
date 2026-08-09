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
}

impl CpuMonitor {
    pub fn new() -> Self {
        let usage = Arc::new(Mutex::new(Vec::new()));
        let usage_clone = usage.clone();
        
        thread::spawn(move || {
            let mut sys = sysinfo::System::new_all();
            // Initial refresh to populate CPU data
            sys.refresh_cpu_all();
            thread::sleep(Duration::from_millis(500));
            
            loop {
                sys.refresh_cpu_all();
                
                let cpu_usage: Vec<f32> = sys.cpus().iter().map(|c| c.cpu_usage()).collect();
                
                if let Ok(mut u) = usage_clone.lock() {
                    *u = cpu_usage;
                }
                
                thread::sleep(Duration::from_millis(500));
            }
        });
        
        CpuMonitor { usage }
    }
    
    pub fn get_usage(&self) -> Vec<f32> {
        self.usage.lock().unwrap().clone()
    }
}
