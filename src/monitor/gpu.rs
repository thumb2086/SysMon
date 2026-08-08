use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

static mut GPU_CACHE: Option<(Instant, Option<f32>, Option<u64>, Option<u64>)> = None;
static mut GPU_PROCESS_CACHE: Option<(Instant, Vec<(u32, String, f32)>)> = None;

pub fn get_gpu_info() -> (Option<f32>, Option<u64>, Option<u64>) {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            if let Some((time, usage, used, total)) = &GPU_CACHE {
                if time.elapsed() < Duration::from_secs(2) {
                    return (*usage, *used, *total);
                }
            }
        }
        
        let result = get_gpu_info_inner();
        
        unsafe {
            GPU_CACHE = Some((Instant::now(), result.0, result.1, result.2));
        }
        
        result
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        (None, None, None)
    }
}

#[cfg(target_os = "windows")]
fn get_gpu_info_inner() -> (Option<f32>, Option<u64>, Option<u64>) {
    use std::process::Command;
    
    let output = Command::new("nvidia-smi")
        .args(&["--query-gpu=utilization.gpu,memory.used,memory.total", "--format=csv,noheader,nounits"])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output();
    
    if let Ok(out) = output {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let parts: Vec<&str> = stdout.trim().split(", ").collect();
            if parts.len() >= 3 {
                let usage = parts[0].parse::<f32>().ok();
                let mem_used = parts[1].parse::<u64>().ok().map(|v| v * 1024 * 1024);
                let mem_total = parts[2].parse::<u64>().ok().map(|v| v * 1024 * 1024);
                return (usage, mem_used, mem_total);
            }
        }
    }
    
    (None, None, None)
}

pub fn get_process_gpu_usage() -> Vec<(u32, String, f32)> {
    #[cfg(target_os = "windows")]
    {
        unsafe {
            if let Some((time, procs)) = &GPU_PROCESS_CACHE {
                if time.elapsed() < Duration::from_secs(3) {
                    return procs.clone();
                }
            }
        }
        
        let result = get_process_gpu_usage_inner();
        
        unsafe {
            GPU_PROCESS_CACHE = Some((Instant::now(), result.clone()));
        }
        
        result
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Vec::new()
    }
}

#[cfg(target_os = "windows")]
fn get_process_gpu_usage_inner() -> Vec<(u32, String, f32)> {
    use std::process::Command;
    
    let mut result = Vec::new();
    
    let output = Command::new("nvidia-smi")
        .args(&["--query-compute-apps=pid,used_memory,name", "--format=csv,noheader,nounits"])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output();
    
    if let Ok(out) = output {
        if out.status.success() {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split(", ").collect();
                if parts.len() >= 3 {
                    if let Ok(pid) = parts[0].parse::<u32>() {
                        let name = parts[2].to_string();
                        let mem = parts[1].parse::<f32>().unwrap_or(0.0);
                        result.push((pid, name, mem));
                    }
                }
            }
        }
    }
    
    result
}
