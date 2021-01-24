use crate::util::AnyResult;
use clap::Clap;
use config::Config;
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
    sync::Mutex,
};

#[derive(Debug, Clap)]
pub struct Cmd {
    /// configuration file path
    #[clap(long, env = "NABU_CONFIG")]
    config: Option<PathBuf>,
}

pub static CONF: OnceCell<Mutex<Config>> = OnceCell::new();

pub fn init() -> AnyResult {
    let cmd: Cmd = Clap::parse();

    let mut result = Config::new();
    result.merge(config::Environment::with_prefix("NABU"))?;

    if let Some(path) = cmd.config {
        result.merge(config::File::from(path))?;
    }

    let _ = CONF.set(Mutex::new(result));

    Ok(())
}

fn conf() -> impl Deref<Target = Config> + DerefMut<Target = Config> + 'static {
    CONF.get().unwrap().lock().unwrap()
}

pub fn default_get<'de, T: Deserialize<'de>>(
    key: &str,
    default: impl Into<config::Value>,
) -> AnyResult<T> {
    conf().set_default(key, default)?;
    Ok(conf().get(key)?)
}
