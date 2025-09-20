use once_cell::sync::OnceCell;
use opentelemetry::metrics::ObservableGauge;
use sysinfo::{ProcessesToUpdate, System};

use crate::meter::METER;

static CPU_USAGE_OBSERVABLE_GAUGE: OnceCell<ObservableGauge<f64>> = OnceCell::new();

pub fn init_cpu_usage_observable_gauge() {
    CPU_USAGE_OBSERVABLE_GAUGE
        .set(
            METER
                .f64_observable_gauge("cdsctf.cpu_usage")
                .with_description("CPU usage")
                .with_callback(|observer| {
                    let mut system = System::new();
                    system.refresh_processes(ProcessesToUpdate::All, true);
                    let pid =
                        sysinfo::get_current_pid().expect("Failed to get current process's PID");
                    let measurement = if let Some(process) = system.process(pid) {
                        process.cpu_usage() as f64
                    } else {
                        0.0
                    };
                    observer.observe(measurement, &[])
                })
                .build(),
        )
        .ok();
}

static RAM_USAGE_OBSERVABLE_GAUGE: OnceCell<ObservableGauge<u64>> = OnceCell::new();

pub fn init_ram_usage_observable_gauge() {
    RAM_USAGE_OBSERVABLE_GAUGE
        .set(
            METER
                .u64_observable_gauge("cdsctf.ram_usage")
                .with_description("RAM usage")
                .with_callback(|observer| {
                    let mut system = System::new();
                    system.refresh_processes(ProcessesToUpdate::All, true);
                    let pid =
                        sysinfo::get_current_pid().expect("Failed to get current process's PID");
                    let measurement = if let Some(process) = system.process(pid) {
                        process.memory()
                    } else {
                        0
                    };
                    observer.observe(measurement, &[])
                })
                .build(),
        )
        .ok();
}
