use sysinfo::Networks;

pub fn get_network_traffic(networks: &Networks) -> (u64, u64) {
    let mut sent = 0u64;
    let mut received = 0u64;
    
    for (_name, data) in networks.iter() {
        sent += data.total_transmitted();
        received += data.total_received();
    }
    
    (sent, received)
}

pub fn get_network_interfaces() -> Vec<String> {
    #[cfg(target_os = "windows")]
    {
        vec!["Ethernet".to_string(), "Wi-Fi".to_string()]
    }
    #[cfg(not(target_os = "windows"))]
    {
        vec!["eth0".to_string(), "wlan0".to_string()]
    }
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
