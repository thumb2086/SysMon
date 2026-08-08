pub mod cpu;
pub mod memory;
pub mod gpu;
pub mod network;

use sysinfo::{System, Networks};

pub struct SystemInfo {
    pub sys: System,
    pub networks: Networks,
    pub cpu_usage: Vec<f32>,
    pub memory_used: u64,
    pub memory_total: u64,
    pub gpu_usage: Option<f32>,
    pub gpu_memory_used: Option<u64>,
    pub gpu_memory_total: Option<u64>,
    pub network_sent: u64,
    pub network_received: u64,
}

impl SystemInfo {
    pub fn new() -> Self {
        SystemInfo {
            sys: System::new_all(),
            networks: Networks::new_with_refreshed_list(),
            cpu_usage: Vec::new(),
            memory_used: 0,
            memory_total: 0,
            gpu_usage: None,
            gpu_memory_used: None,
            gpu_memory_total: None,
            network_sent: 0,
            network_received: 0,
        }
    }

    pub fn update(&mut self) {
        self.sys.refresh_all();
        self.networks.refresh();
        
        self.cpu_usage = cpu::get_cpu_usage(&mut self.sys);
        let mem = memory::get_memory_info(&self.sys);
        self.memory_used = mem.0;
        self.memory_total = mem.1;
        
        let gpu = gpu::get_gpu_info();
        self.gpu_usage = gpu.0;
        self.gpu_memory_used = gpu.1;
        self.gpu_memory_total = gpu.2;
        
        let net = network::get_network_traffic(&self.networks);
        self.network_sent = net.0;
        self.network_received = net.1;
    }
}
