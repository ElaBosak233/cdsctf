use once_cell::sync::OnceCell;
use opentelemetry::metrics::{Counter, UpDownCounter};

use crate::meter::METER;

pub static ACTIVE_REQUESTS: OnceCell<UpDownCounter<i64>> = OnceCell::new();

pub fn get_active_requests() -> &'static UpDownCounter<i64> {
    ACTIVE_REQUESTS.get_or_init(|| {
        METER
            .i64_up_down_counter("cdsctf.active_requests")
            .with_description("The number of active HTTP requests")
            .build()
    })
}

pub static REQUEST_BYTES: OnceCell<Counter<u64>> = OnceCell::new();

pub fn get_request_bytes() -> &'static Counter<u64> {
    REQUEST_BYTES.get_or_init(|| {
        METER
            .u64_counter("cdsctf.request_bytes")
            .with_description("The number of bytes received in HTTP requests")
            .build()
    })
}

pub static RESPONSE_BYTES: OnceCell<Counter<u64>> = OnceCell::new();

pub fn get_response_bytes() -> &'static Counter<u64> {
    RESPONSE_BYTES.get_or_init(|| {
        METER
            .u64_counter("cdsctf.response_bytes")
            .with_description("The number of bytes sent in HTTP responses")
            .build()
    })
}
