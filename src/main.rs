use sysinfo::{System, Users};
use rascii_art::{
    render_to,
    RenderOptions,
};

fn get_current_user_name(system: &System) -> String {
    match system.process(sysinfo::get_current_pid().unwrap()) {
        Some(process) => {
            let users = Users::new_with_refreshed_list();
            match process.user_id() {
                Some(user_id) => match users.get_user_by_id(user_id) {
                    Some(curr_user) => curr_user.name().to_string(),
                    None => "Unknown".to_string(), // User ID found, but no user object
                },
                None => "Unknown".to_string(), // No user ID for the process
            }
        }
        None => "Unknown".to_string(), // Process not found
    }
}

fn format_info_name(info_name: &str) -> String {
    format!("\x1b[38;2;255;254;187m{}\x1b[0m", info_name)
}

fn get_headline(system: &System) -> String {
    let host_name = System::host_name().expect("Error obtaining system host name");
    let user_name = get_current_user_name(&system);
    format!("\x1b[38;2;166;252;104m{}\x1b[0m@\x1b[38;2;166;252;104m{}\x1b[0m", user_name, host_name)
}

fn main() {
    let mut buffer = String::new();
                                                            
    render_to(
        r"/Users/kllarena/documents/github/myfetch/mental_illness.png",
        &mut buffer,
        &RenderOptions::new()
            .width(80)
            .height(30)
            .colored(true)
            .charset(rascii_art::charsets::BLOCK)
    )
    .unwrap();

    let parts = buffer.split('\n');

    let mut system = System::new_all();
    system.refresh_all();

    for (index, part) in parts.enumerate() {
        match index {
            0 => {
                println!("{}   {}", part, get_headline(&system));
            },
            1 => {
                println!("{}   --------------------------------", part);
            },
            2 => {
                let long_os_version = System::long_os_version().unwrap();
                let cpu_arch = System::cpu_arch().unwrap();
                println!("{}   {}: {}{}", part, format_info_name("OS"), long_os_version, cpu_arch);
            },
            3 => {
                println!("{}   {}:", part, format_info_name("Host"));
            }
            4 => {
                let kernel_version = System::kernel_version().unwrap();
                println!("{}   {}: {}", part, format_info_name("Kernel"), kernel_version);
            },
            5 => {
                println!("{}   {}:", part, format_info_name("Uptime"));
            },
            6 => {
                println!("{}   {}:", part, format_info_name("Packages"));
            },
            7 => {
                println!("{}   {}:", part, format_info_name("Shell"));
            },
            8 => {
                println!("{}   {}:", part, format_info_name("Resolution"));
            },
            9 => {
                println!("{}   {}:", part, format_info_name("DE"));
            },
            10 => {
                println!("{}   {}:", part, format_info_name("WM"));
            },
            11 => {
                println!("{}   {}:", part, format_info_name("WM Theme"));
            },
            12 => {
                println!("{}   {}:", part, format_info_name("Terminal"));
            },
            13 => {
                println!("{}   {}:", part, format_info_name("CPU"));
            },
            14 => {
                println!("{}   {}:", part, format_info_name("GPU"));
            },
            15 => {
                let used_memory = system.used_memory() / 1048576;
                let total_memory = system.total_memory() / 1048576;
                println!("{}   {}: {} MiB / {} MiB", part, format_info_name("Memory"), used_memory, total_memory);
            },
            _ => {
                println!("{}", part);
            }
        }
    }
}
