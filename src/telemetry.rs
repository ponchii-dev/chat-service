// /src/telemetry.rs

use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
