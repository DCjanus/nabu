use crate::source::{Source, SourceBuilder};
use actix_web::App;
use log::error;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct AtomHub {
    apps: BTreeMap<&'static str, Source>,
}

impl AtomHub {
    pub fn register<T: SourceBuilder>(mut self, _: T) -> Self {
        let source = T::build_source();

        if self.apps.contains_key(source.prefix) {
            let error_message = format!("duplicate prefix: {prefix}", prefix = source.prefix);
            error!("{}", error_message);
            panic!(error_message);
        }

        self.apps.insert(source.prefix, source);
        self
    }

    pub fn into_apps(self) -> Vec<App> {
        self.apps.into_iter().map(|x| x.1.into_app()).collect()
    }
}
