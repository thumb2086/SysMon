use eframe::egui;
use crate::monitor::SystemInfo;
use super::format_bytes;

pub fn render(
    ui: &mut egui::Ui,
    sys_info: &SystemInfo,
) {
    ui.heading("Processes");
    ui.separator();

    ui.horizontal(|ui| {
        ui.label("Sort by:");
        ui.selectable_value(&mut ui.data_mut(|d| d.get_persisted_mut_or("process_sort".to_string(), 0u8).clone()), 0, "CPU");
        ui.selectable_value(&mut ui.data_mut(|d| d.get_persisted_mut_or("process_sort".to_string(), 0u8).clone()), 1, "Memory");
    });

    ui.add_space(4.0);

    egui_extras::TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(egui_extras::Column::auto().at_least(60.0))
        .column(egui_extras::Column::remainder().at_least(100.0))
        .column(egui_extras::Column::auto().at_least(80.0))
        .column(egui_extras::Column::auto().at_least(80.0))
        .header(20.0, |mut header| {
            header.col(|ui| { ui.strong("PID"); });
            header.col(|ui| { ui.strong("Name"); });
            header.col(|ui| { ui.strong("CPU"); });
            header.col(|ui| { ui.strong("Memory"); });
        })
        .body(|mut body| {
            let mut processes: Vec<_> = sys_info.sys.processes().collect();
            
            let sort_by = ui.data_mut(|d| d.get_persisted_or("process_sort".to_string(), 0u8));
            match sort_by {
                0 => processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap()),
                _ => processes.sort_by(|a, b| b.1.memory().partial_cmp(&a.1.memory()).unwrap()),
            }
            
            for (pid, proc) in processes.iter().take(100) {
                body.row(18.0, |mut row| {
                    row.col(|ui| { ui.label(pid.to_string()); });
                    row.col(|ui| { ui.label(proc.name().to_string()); });
                    row.col(|ui| { 
                        let cpu = proc.cpu_usage();
                        let color = if cpu >= 50.0 {
                            egui::Color32::from_rgb(243, 139, 168)
                        } else if cpu >= 20.0 {
                            egui::Color32::from_rgb(249, 226, 175)
                        } else {
                            egui::Color32::from_rgb(166, 227, 161)
                        };
                        ui.colored_label(color, format!("{:.1}%", cpu)); 
                    });
                    row.col(|ui| { ui.label(format_bytes(proc.memory())); });
                });
            }
        });
}
