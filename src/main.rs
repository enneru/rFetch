//this is my first rust project, expect many things to be far from perfect
use sysinfo::{
    Disks, System
};

const TAB: &str = "\t\t\t\t"; //tabs for temporary spacing
const CLR: &str = "\x1b[0m"; //Clear color
const RED: &str = "\x1b[31m"; //RED
const BLUE: &str = "\x1b[36m"; //BLUE
const YELLOW: &str = "\x1b[33m"; //YELLOW
const GREEN: &str = "\x1b[32m"; //GREEN
//
fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("\n{RED}{TAB}r{BLUE}Fetch{CLR} - created by vivi\n{BLUE}------------------------------------------------------------------------------{CLR}");

    println!("{TAB}{YELLOW}Welcome{RED}    {}!{CLR}", whoami::username());
    println!("{TAB}{YELLOW}System OS:{CLR}{:?}", System::long_os_version().unwrap());
    println!("{TAB}{YELLOW}Kernel:{CLR}   {:?}", System::kernel_version().unwrap());
    
    //uptime stuff- doing basic conversions from seconds to minuts/hours and modifying output text
    //accordingly
    let uptime = System::uptime();
    let time: String;
    if uptime <= 60 {
        time = format!("{uptime}s");
    } else if uptime > 60 && uptime < 3600 {
        let m = uptime / 60;
        let s = uptime % 60;
        time = format!("{m}m{s}s")
    } else if uptime > 3600 {
        let h = uptime / 3600;
        let m = (uptime / 60) % 60;
        time = format!("{h}h{m}m");
    } else {
        time = format!("{RED}err{CLR}")
    }
    println!("{TAB}{YELLOW}Uptime:{CLR}    {time}");

    //Memory
    println!("{TAB}{BLUE}==> Memory <=={CLR}");
    let mem_used = sys.used_memory() / 1000000;
    let mem_total = sys.total_memory() / 1000000;
    println!("{TAB}{BLUE}Used:{CLR}      {}/{} MB", mem_used, mem_total);

    //CPU
    println!("{TAB}{YELLOW}==> CPU <=={CLR}");
    println!("{TAB}{YELLOW}CPU: {CLR}{}", sys.cpus()[0].brand());
    //println!("{TAB}{YELLOW}{}{CLR}", sys.global_cpu_usage());


    //Disks
    println!("{TAB}{GREEN}==> Disks <=={CLR}");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let disk_used: String;
        let mut disk_a: f64 = ((disk.total_space() - disk.available_space()) / 1000000) as f64;
        if disk_a < 1000.0 {
            disk_used = format!("{disk_a} MB");
        } else {
            disk_a /= 1000.0;
            disk_used = format!("{disk_a} GB");
        }
        let disk_total: String;
        let mut disk_t: f64 = (disk.total_space() / 1000000) as f64;
        if disk_t < 1000.0 {
            disk_total = format!("{disk_t} MB");
        } else {
            disk_t /= 1000.0;
            disk_total = format!("{disk_t} GB");
        }
        println!("{TAB}{GREEN}{:?}:{CLR} {} / {}", disk.name(), disk_used, disk_total);
    }

}
