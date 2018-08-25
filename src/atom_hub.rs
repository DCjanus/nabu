use actix_web::App;
use source::{Source, SourceBuilder};
use std::collections::BTreeMap;

#[derive(Default)]
pub struct AtomHub {
    apps: BTreeMap<&'static str, Source>,
}

impl AtomHub {
    // Register your sources
    pub fn init() -> Self {
        AtomHub::default()
            .register(::routes::github::GitHubSource)
            .register(::routes::v2ex::V2exSource)
    }
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
