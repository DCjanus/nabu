use routes::crates_io::crate_versions::CrateVersionsGenerator;
use source::{Source, SourceBuilder};

pub mod crate_versions;

pub struct CratesIoSource;

impl SourceBuilder for CratesIoSource {
    fn build_source() -> Source {
        Source::new("crates-io").register(CrateVersionsGenerator)
    }
}
