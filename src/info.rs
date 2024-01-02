use sysinfo::{
    System, RefreshKind, CpuRefreshKind
};
use std::{collections::HashMap, env, io::{self, Write}};
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Print,
};

// use termion::cursor;

fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86_400;
    let hours = (seconds % 86_400) / 3_600;
    let mins = (seconds % 3_600) / 60;
    
    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

fn get_system_info() -> HashMap<String, String> {
    let mut info = HashMap::new();
    let cpu_info = System::new_with_specifics(
        RefreshKind::new().with_cpu(CpuRefreshKind::everything())
    );
    let mut sys = System::new_all();
    sys.refresh_all();
    info.insert("OS".to_string(), format!("{} {}",System::name().unwrap(), System::cpu_arch().unwrap()));
    info.insert("Memory".to_string(), format!("{:.2} GB/{:.2} GB", bytes_to_gb(sys.used_memory()), bytes_to_gb(sys.total_memory())));
    info.insert("Kernel".to_string(), System::kernel_version().unwrap());
    info.insert("CPU".to_string(), format!("{}({})", cpu_info.cpus()[0].brand(), cpu_info.cpus().len()));
    info.insert("Host".to_string(), System::host_name().unwrap());
    info.insert("Shell".to_string(), env::var("SHELL").unwrap().split('/').collect::<Vec<&str>>()[3].to_string());
    info.insert("Uptime".to_string(), format_uptime(System::uptime()));

    info
}

pub fn display_info() {
    let sys_info: HashMap<String, String> = get_system_info();
    let cur_row = 15;
    let left_pad = 45;
    execute!(
        std::io::stdout(),
        MoveTo(left_pad, cur_row),
        Print(format!("Host: {}", sys_info.get("Host").unwrap())),
        MoveTo(left_pad, cur_row + 1),
        Print(format!("OS: {}", sys_info.get("OS").unwrap())),
        MoveTo(left_pad, cur_row + 2),
        Print(format!("Kernel: {}", sys_info.get("Kernel").unwrap())),
        MoveTo(left_pad, cur_row + 3),
        Print(format!("CPU: {}", sys_info.get("CPU").unwrap())),
        MoveTo(left_pad, cur_row + 4),
        Print(format!("Memory: {}", sys_info.get("Memory").unwrap())),
        MoveTo(left_pad, cur_row + 5),
        Print(format!("Shell: {}", sys_info.get("Shell").unwrap())),
        MoveTo(left_pad, cur_row + 6),
        Print(format!("Uptime: {}", sys_info.get("Uptime").unwrap())),
    )
    .unwrap();
    for _ in 0..20 {
        execute!(io::stdout(), Print("\n")).unwrap();
    }
    io::stdout().flush().unwrap();
}
