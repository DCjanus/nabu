use crate::atom_hub::AtomHub;

pub mod crates_io;
pub mod github;
pub mod v2ex;
pub mod zhihu;

pub fn atom_hub() -> AtomHub {
    AtomHub::default()
        .register(crate::routes::github::GitHubSource)
        .register(crate::routes::v2ex::V2exSource)
        .register(crate::routes::crates_io::CratesIoSource)
        .register(crate::routes::zhihu::ZhihuSource)
}
