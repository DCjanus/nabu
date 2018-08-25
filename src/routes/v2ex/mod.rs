use self::hot_topics::HotTopicsGenerator;
use source::{Source, SourceBuilder};

pub mod hot_topics;

pub struct V2exSource;

impl SourceBuilder for V2exSource {
    fn build_source() -> Source {
        Source::new("v2ex").register(HotTopicsGenerator)
    }
}
