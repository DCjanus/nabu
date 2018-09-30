use atom_hub::AtomHub;

pub mod crates_io;
pub mod github;
pub mod v2ex;
pub mod zhihu;

pub fn atom_hub() -> AtomHub {
    AtomHub::default()
        .register(::routes::github::GitHubSource)
        .register(::routes::v2ex::V2exSource)
        .register(::routes::crates_io::CratesIoSource)
        .register(::routes::zhihu::ZhihuSource)
}
