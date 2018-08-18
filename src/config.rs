// TODO should be configurable
use chrono::FixedOffset;
use std::{env, time::Duration};

#[derive(Eq, PartialEq, Debug)]
pub enum ServeMode {
    Prod,
    Dev,
    Test,
}

pub fn pg_url() -> impl AsRef<str> {
    let default_db_url = "postgresql://nabu@localhost:5432/nabu".to_string();
    env::var("PG_URL").unwrap_or(default_db_url)
}

pub fn local_address() -> impl AsRef<str> {
    let default_address = "0.0.0.0:80".to_string();
    env::var("LOCAL_ADDR").unwrap_or(default_address)
}

pub fn cache_duration() -> Duration {
    Duration::from_secs(30 * 60)
}

pub fn offset() -> FixedOffset {
    FixedOffset::east(8 * 3600)
}

pub fn serve_mode() -> ServeMode {
    let server_env = env::var("SERVER_ENV")
        .unwrap_or_else(|_| "PROD".to_string())
        .to_uppercase();

    if server_env == "TEST" {
        ServeMode::Test
    } else if server_env == "DEV" {
        ServeMode::Dev
    } else {
        ServeMode::Prod
    }
}
