use std::thread::sleep;
use std::time::Duration;
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

const REFRESH_TIME: Duration = Duration::from_secs(3);
const B_TO_GB: u64 = 1024 * 1024 * 1024;

fn main() {
    let refresh = RefreshKind::new()
        .with_cpu(CpuRefreshKind::new().with_cpu_usage())
        .with_memory();
    let mut sys = System::new_with_specifics(refresh);

    // Sleep for 1 second to get an initial reading
    sleep(Duration::from_secs(1));

    loop {
        sys.refresh_specifics(refresh);

        let cpu = sys.global_cpu_info().cpu_usage().round();
        let mem = sys.used_memory() / B_TO_GB;

        println!("{cpu}% {mem}G");
        sleep(REFRESH_TIME);
    }
}
