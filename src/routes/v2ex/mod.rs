use self::hot_topics::HotTopicsGenerator;
use source::{IntoSource, Source};

pub mod hot_topics;

pub struct V2exSource;

impl IntoSource for V2exSource {
    fn into_source(self) -> Source {
        Source::new("v2ex").register::<HotTopicsGenerator>()
    }
}
