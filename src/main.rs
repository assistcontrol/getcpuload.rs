use std::thread::sleep;
use std::time::Duration;
use sysinfo::{CpuExt, CpuRefreshKind, RefreshKind, System, SystemExt};

const REFRESH_TIME: Duration = Duration::from_secs(3);

fn main() {
    let refresh = RefreshKind::new()
        .with_cpu(CpuRefreshKind::new().with_cpu_usage())
        .with_memory();
    let mut sys = System::new_with_specifics(refresh);

    let mem_total = sys.total_memory();

    // Sleep for 1 second to get an initial reading
    sleep(Duration::from_secs(1));

    loop {
        sys.refresh_specifics(refresh);

        let cpu = sys.global_cpu_info().cpu_usage();
        let cpu_percent = fmt_cpu_percent(cpu);

        let mem_used = sys.used_memory();
        let mem_gb = fmt_memory_gb(mem_used);
        let mem_percent = fmt_memory_percent(mem_used, mem_total);

        println!("{cpu_percent} {mem_percent}({mem_gb})");
        sleep(REFRESH_TIME);
    }
}

fn fmt_cpu_percent(cpu: f32) -> String {
    let percent = cpu.round();

    format!("{percent}%")
}

fn fmt_memory_gb(mem_used: u64) -> String {
    let mem_bytes = to_gb(mem_used);
    format!("{mem_bytes}G")
}

fn fmt_memory_percent(mem_used: u64, mem_total: u64) -> String {
    let mem_percent = ((mem_used as f64 / mem_total as f64) * 100.0).round();
    format!("{mem_percent}%")
}

fn to_gb(bytes: u64) -> f64 {
    const B_TO_GB: f64 = 1024.0 * 1024.0 * 1024.0;

    (bytes as f64 / B_TO_GB).round()
}
