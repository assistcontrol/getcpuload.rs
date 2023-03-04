use std::thread::sleep;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};

const REFRESHTIME: Duration = Duration::from_secs(3);

fn main() {
    let mut sys = System::new();

    sys.refresh_cpu();
    let ncpu = sys.cpus().len() as f32;

    // Sleep for 1 second to get an initial reading
    sleep(Duration::from_secs(1));

    loop {
        sys.refresh_cpu();

        let scaled = cpu_usage(sys.cpus(), ncpu);
        println!("{}%", scaled);

        sleep(REFRESHTIME);
    }
}

// cpu_usage calculates the average cpu usage across all cores
// and returns a rounded percentage
//
// @param cpu: a slice of sysinfo::Cpu
// @param ncpu: the number of cores
// @return: the average cpu usage across all cores
fn cpu_usage(cpus: &[sysinfo::Cpu], ncpu: f32) -> f32 {
    let mut total = 0.0;
    for cpu in cpus {
        total += cpu.cpu_usage();
    }

    (total / ncpu).round()
}
