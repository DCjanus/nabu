use actix_web::{middleware::Logger, App};
use feed_generator::FeedGenerator;
use feed_worker::FeedWorker;
use std::collections::BTreeMap;

pub trait SourceBuilder {
    fn build_source() -> Source;
}

pub struct Source {
    pub prefix: &'static str,
    pub entries: BTreeMap<&'static str, FeedWorker>,
}

impl Source {
    pub fn new(prefix: &'static str) -> Self {
        Source {
            prefix,
            entries: BTreeMap::new(),
        }
    }

    pub fn register<T: FeedGenerator>(mut self, _: T) -> Self {
        let path = T::PATH;

        if self.entries.contains_key(path) {
            let error_message = format!(
                "duplicate path for {prefix}: {path}",
                prefix = self.prefix,
                path = path
            );
            error!("{}", error_message);
            panic!(error_message);
        }

        let worker = FeedWorker {
            path,
            prefix: self.prefix,
            clean_query_string: T::clean_query_string,
            update_by_value: T::update_by_value,
        };

        self.entries.insert(path, worker);
        self
    }

    pub fn into_app(self) -> App {
        let mut result = App::new().prefix(self.prefix).middleware(Logger::default());
        for i in self.entries.values() {
            let path = i.path;
            let handler = i.clone().into_actix_web_handler();
            result = result.resource(path, move |resource| resource.f(handler));
        }
        result
    }
}
