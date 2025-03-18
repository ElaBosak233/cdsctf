use crate::meter::METER;
use once_cell::sync::OnceCell;
use opentelemetry::metrics::{Counter, UpDownCounter};

pub static ACTIVE_REQUESTS: OnceCell<UpDownCounter<i64>> = OnceCell::new();

pub fn init_active_requests() {
    ACTIVE_REQUESTS.set(
        METER.
            i64_up_down_counter("cdsctf.active_requests")
            .with_description("The number of active HTTP requests")
            .build(),
    ).ok();
}

pub fn get_active_requests() -> &'static UpDownCounter<i64> {
    ACTIVE_REQUESTS.get().unwrap()
}

pub static REQUEST_BYTES: OnceCell<Counter<u64>> = OnceCell::new();

pub fn init_request_bytes() {
    REQUEST_BYTES.set(
        METER.u64_counter("cdsctf.request_bytes")
            .with_description("The number of bytes received in HTTP requests")
            .build()
    ).ok();
}

pub fn get_request_bytes() -> &'static Counter<u64> {
    REQUEST_BYTES.get().unwrap()
}

pub static RESPONSE_BYTES: OnceCell<Counter<u64>> = OnceCell::new();

pub fn init_response_bytes() {
    RESPONSE_BYTES.set(
        METER
            .u64_counter("cdsctf.response_bytes")
            .with_description("The number of bytes sent in HTTP responses")
            .build()
    ).ok();
}

pub fn get_response_bytes() -> &'static Counter<u64> {
    RESPONSE_BYTES.get().unwrap()
}