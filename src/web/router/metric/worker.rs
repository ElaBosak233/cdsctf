use std::time::Duration;

use sysinfo::System;
use tokio::time::interval;
use tracing::info;

use crate::metric::{CPU_USAGE, MEMORY_USAGE};

pub async fn init() {
    tokio::spawn(async {
        let mut sys = System::new_all();
        let interval = Duration::from_secs(1);

        loop {
            sys.refresh_all();

            let total_memory = sys.total_memory();
            let used_memory = sys.used_memory();
            let memory_usage = (used_memory as f64 / total_memory as f64 * 100.0) as i64;

            MEMORY_USAGE.set(memory_usage);

            let cpu_usage = sys.global_cpu_usage() as i64;
            CPU_USAGE.set(cpu_usage);

            tokio::time::sleep(interval).await;
        }
    });

    info!("metrics worker initialized successfully.");
}
