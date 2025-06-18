//this is my first rust project, expect many things to be far from perfect
use sysinfo::{
    Disks, System
};
const CLR: &str = "\x1b[0m"; //Clear color
const RED: &str = "\x1b[31m"; //RED
const BLUE: &str = "\x1b[36m"; //BLUE
const YELLOW: &str = "\x1b[33m"; //YELLOW
const GREEN: &str = "\x1b[32m"; //GREEN
//
fn main() {
    let logo1: String = String::from(r"-------------------");
    let logo2: String = String::from(r"------|\___/|------");
    let logo3: String = String::from(r"---n--/ O  O\------");
    let logo4: String = String::from(r"---\\ | =( w )=----");
    let logo5: String = String::from(r"----\/       \-----");
    let logo6: String = String::from(r"-----(|||) (|||)---");
    let logo7: String = String::from("-----\x1b[31mr \x1b[36mF e t c h\x1b[0m---");
    let mut sys = System::new_all();
 
    sys.refresh_all();

    println!("\n{RED}\t\t\tr{BLUE}Fetch{CLR} - created by vivi\n{BLUE}---------------------------------------------------------------{CLR}");

    println!("{logo1}{YELLOW}Welcome{RED}    {}@{}!{CLR}", whoami::username(), System::host_name().unwrap());
    println!("{logo1}{YELLOW}System OS:{CLR} {} {}", System::distribution_id(), System::cpu_arch());
    println!("{logo2}{YELLOW}Kernel:{CLR}    {:?}", System::kernel_version().unwrap());
    
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
    println!("{logo3}{YELLOW}Uptime:{CLR}    {time}");

    //Memory
    println!("{logo4}{BLUE}==> Memory <=={CLR}");
    let mem_used = sys.used_memory() / 1000000;
    let mem_total = sys.total_memory() / 1000000;
    println!("{logo5}{BLUE}Used:{CLR}      {}/{} MB", mem_used, mem_total);

    //CPU
    println!("{logo6}{YELLOW}==> CPU <=={CLR}");
    println!("{logo7}{YELLOW}CPU: {CLR}{}", sys.cpus()[0].brand());
    //println!("{TAB}{YELLOW}{}{CLR}", sys.global_cpu_usage());


    //Disks
    println!("{logo1}{GREEN}==> Disks <=={CLR}");
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
        println!("{logo1}{GREEN}{:?}:{CLR} {} / {}", disk.mount_point(), disk_used, disk_total);
    }

}
