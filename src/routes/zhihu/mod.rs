use routes::zhihu::column::ColumnGenerator;
use source::{Source, SourceBuilder};

pub mod column;

pub struct ZhihuSource;

impl SourceBuilder for ZhihuSource {
    fn build_source() -> Source {
        Source::new("zhihu").register(ColumnGenerator)
    }
}
