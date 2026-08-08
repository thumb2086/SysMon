#[cfg(target_os = "windows")]
pub fn get_gpu_info() -> (Option<f32>, Option<u64>, Option<u64>) {
    use std::process::Command;
    
    let output = Command::new("wmic")
        .args(&["path", "win32_videocontroller", "get", "AdapterRAM,Utilization"])
        .output();
    
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let lines: Vec<&str> = lines.trim().lines().collect();
            
            if lines.len() > 1 {
                let parts: Vec<&str> = lines[1].split_whitespace().collect();
                if parts.len() >= 2 {
                    let mem = parts[0].parse::<u64>().ok().map(|v| v / 1024 / 1024);
                    let usage = parts[1].parse::<f32>().ok();
                    return (usage, mem, mem);
                }
            }
            (None, None, None)
        }
        Err(_) => (None, None, None),
    }
}

#[cfg(not(target_os = "windows"))]
pub fn get_gpu_info() -> (Option<f32>, Option<u64>, Option<u64>) {
    (None, None, None)
}
