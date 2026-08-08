use sysinfo::System;

pub fn get_network_traffic(sys: &System) -> (u64, u64) {
    let mut sent = 0u64;
    let mut received = 0u64;
    
    for (_name, data) in sys.networks() {
        sent += data.total_transmitted();
        received += data.total_received();
    }
    
    (sent, received)
}

pub fn get_network_interfaces() -> Vec<String> {
    sysinfo::Networks::new_with_refreshed_list()
        .iter()
        .map(|(name, _)| name.clone())
        .collect()
}

pub fn disconnect_network(interface: &str) -> bool {
    use std::process::Command;
    
    #[cfg(target_os = "windows")]
    {
        Command::new("netsh")
            .args(&["interface", "set", "interface", interface, "disabled"])
            .output()
            .is_ok()
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Command::new("ifconfig")
            .args(&[interface, "down"])
            .output()
            .is_ok()
    }
}

pub fn reconnect_network(interface: &str) -> bool {
    use std::process::Command;
    
    #[cfg(target_os = "windows")]
    {
        Command::new("netsh")
            .args(&["interface", "set", "interface", interface, "enabled"])
            .output()
            .is_ok()
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Command::new("ifconfig")
            .args(&[interface, "up"])
            .output()
            .is_ok()
    }
}
