use actix_web::App;
use source::IntoSource;

#[derive(Default)]
pub struct AtomHub {
    pub apps: Vec<App>,
}

impl AtomHub {
    pub fn new() -> Self {
        AtomHub::default()
    }

    pub fn register(mut self, source: impl IntoSource) -> Self {
        self.apps.push(source.into_source().into_app());
        self
    }
}
