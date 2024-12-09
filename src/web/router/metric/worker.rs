use std::time::Duration;

use sysinfo::{Pid, System};
use tracing::info;

use crate::metric::{CPU_USAGE, MEMORY_USAGE, MEMORY_USAGE_RATIO};

pub async fn init() {
    tokio::spawn(async {
        let mut sys = System::new_all();
        let interval = Duration::from_secs(1);

        loop {
            sys.refresh_all();

            let pid = std::process::id();
            if let Some(process) = sys.process(Pid::from_u32(pid)) {
                let used_memory = process.memory();
                MEMORY_USAGE.set(used_memory as i64);

                let total_memory = sys.total_memory();
                let memory_usage = used_memory as f64 / total_memory as f64 * 100.0;

                MEMORY_USAGE_RATIO.set(memory_usage);

                let cpu_usage = process.cpu_usage() as i64;
                CPU_USAGE.set(cpu_usage);
            }

            tokio::time::sleep(interval).await;
        }
    });

    info!("metrics worker initialized successfully.");
}
