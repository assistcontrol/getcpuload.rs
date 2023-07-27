use std::thread::sleep;
use std::time::Duration;
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

const REFRESH_TIME: Duration = Duration::from_secs(3);

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
        let mem = to_gb(sys.used_memory());

        println!("{cpu}% {mem}G");
        sleep(REFRESH_TIME);
    }
}

fn to_gb(bytes: u64) -> f64 {
    const B_TO_GB: f64 = 1024.0 * 1024.0 * 1024.0;

    (bytes as f64 / B_TO_GB).round()
}
