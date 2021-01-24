use crate::util::AnyResult;
use tracing_subscriber::prelude::*;

pub fn init() -> AnyResult {
    let level: String = crate::conf::default_get("log.level", "info")?;

    let filter = tracing_subscriber::EnvFilter::new(level);
    let format = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(filter)
        .with(format)
        .init();

    Ok(())
}
