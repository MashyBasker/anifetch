mod display;
mod info;

// use sysinfo::{System, RefreshKind, CpuRefreshKind};
// use sysinfo::{System, Users};

fn main() {
    // let sys = System::new_all();
    display::convert();
    info::display_info();    
}
// 