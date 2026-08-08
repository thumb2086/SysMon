#[cfg(target_os = "windows")]
pub fn get_gpu_info() -> (Option<f32>, Option<u64>, Option<u64>) {
    use std::process::Command;
    
    // Try nvidia-smi first (more reliable)
    let output = Command::new("nvidia-smi")
        .args(&["--query-gpu=utilization.gpu,memory.used,memory.total", "--format=csv,noheader,nounits"])
        .output();
    
    if let Ok(out) = output {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let parts: Vec<&str> = stdout.trim().split(", ").collect();
            if parts.len() >= 3 {
                let usage = parts[0].parse::<f32>().ok();
                let mem_used = parts[1].parse::<u64>().ok().map(|v| v * 1024 * 1024); // MB to bytes
                let mem_total = parts[2].parse::<u64>().ok().map(|v| v * 1024 * 1024);
                return (usage, mem_used, mem_total);
            }
        }
    }
    
    // Fallback to wmic
    let output = Command::new("wmic")
        .args(&["path", "win32_videocontroller", "get", "AdapterRAM,Utilization"])
        .output();
    
    match output {
        Ok(out) => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let lines: Vec<&str> = stdout.trim().lines().collect();
            
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

/// Get per-process GPU usage using nvidia-smi
pub fn get_process_gpu_usage() -> Vec<(u32, String, f32)> {
    use std::process::Command;
    
    let mut result = Vec::new();
    
    let output = Command::new("nvidia-smi")
        .args(&["--query-compute-apps=pid,used_memory,name", "--format=csv,noheader,nounits"])
        .output();
    
    if let Ok(out) = output {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split(", ").collect();
                if parts.len() >= 3 {
                    if let Ok(pid) = parts[0].parse::<u32>() {
                        let name = parts[1].to_string();
                        // Note: used_memory is in MiB, we'll use it as approximate GPU usage indicator
                        let mem = parts[1].parse::<f32>().unwrap_or(0.0);
                        result.push((pid, name, mem));
                    }
                }
            }
        }
    }
    
    result
}

#[cfg(not(target_os = "windows"))]
pub fn get_gpu_info() -> (Option<f32>, Option<u64>, Option<u64>) {
    (None, None, None)
}

#[cfg(not(target_os = "windows"))]
pub fn get_process_gpu_usage() -> Vec<(u32, String, f32)> {
    Vec::new()
}
