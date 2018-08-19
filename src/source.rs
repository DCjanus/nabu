use actix_web::{middleware::Logger, App, HttpRequest, HttpResponse};
use feed_generator::FeedGenerator;
use std::collections::BTreeMap;

pub trait IntoSource {
    fn into_source(self) -> Source;
}

pub struct Source {
    pub prefix: &'static str,
    pub entries: BTreeMap<&'static str, fn(&HttpRequest) -> HttpResponse>,
}

impl Source {
    pub fn new(prefix: &'static str) -> Self {
        Source {
            prefix,
            entries: BTreeMap::new(),
        }
    }

    pub fn register<T: FeedGenerator>(mut self) -> Self {
        let path = T::PATH;
        let handler = T::actix_web_handler;
        self.entries.insert(path, handler);
        self
    }

    pub fn into_app(self) -> App {
        let mut result = App::new().prefix(self.prefix).middleware(Logger::default());
        for (path, handler) in self.entries {
            result = result.resource(path, move |resource| resource.f(handler));
        }
        result
    }
}

impl IntoSource for Source {
    fn into_source(self) -> Source {
        self
    }
}
