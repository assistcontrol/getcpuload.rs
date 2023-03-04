use std::thread::sleep;
use std::time::Duration;
use sysinfo::{CpuExt, System, SystemExt};

const REFRESH_TIME: Duration = Duration::from_secs(3);

fn main() {
    let mut sys = System::new();

    sys.refresh_cpu();
    let ncpu = sys.cpus().len() as f32;

    // Sleep for 1 second to get an initial reading
    sleep(Duration::from_secs(1));

    loop {
        sys.refresh_cpu();

        // Get the average CPU usage across all cores
        let total = sys
            .cpus()
            .iter()
            .fold(0.0, |sum, cpu| sum + cpu.cpu_usage());
        let total = (total / ncpu).round();

        println!("{total}%");
        sleep(REFRESH_TIME);
    }
}
