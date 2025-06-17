use sysinfo::{
    Disks, System, Users
};

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("\n\x1b[31mr\x1b[36mFetch\x1b[0m - created by vivi\n\x1b[36m----------------------------------\x1b[0m");

    let users = Users::new_with_refreshed_list();
    for user in users.list() {
        println!("welcome, {}", user.name());
    }
    println!("\x1b[46mSystem OS:\x1b[0m {:?}", System::long_os_version().unwrap());
    println!("\x1b[46mKernel:\x1b[0m {:?}", System::kernel_version().unwrap());

    //Memory
    println!("\x1b[36m==> Memory: <==\x1b[0m");
    let mem_used = sys.used_memory() / 1000000;
    let mem_total = sys.total_memory() / 1000000;
    println!("{} / {} MB", mem_used, mem_total);

    //Disks
    println!("\x1b[32m==> Disks: <==\x1b[0m");
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
        println!("[{:?}]: {} / {}", disk.name(), disk_used, disk_total);
    }

}
