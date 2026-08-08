use sysinfo::System;

pub fn get_cpu_usage(sys: &mut System) -> Vec<f32> {
    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_cpu_all();
    
    sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect()
}

pub fn get_cpu_info(sys: &System) -> (String, u32) {
    let brand = sys.cpus().first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());
    let cores = sys.cpus().len() as u32;
    (brand, cores)
}
