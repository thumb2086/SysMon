use sysinfo::System;

pub fn get_memory_info(sys: &System) -> (u64, u64) {
    (sys.used_memory(), sys.total_memory())
}

pub fn get_swap_info(sys: &System) -> (u64, u64) {
    (sys.used_swap(), sys.total_swap())
}
