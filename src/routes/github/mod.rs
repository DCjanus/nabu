use crate::{
    routes::github::user_repos::UserRepoGenerator,
    source::{Source, SourceBuilder},
};

pub mod user_repos;

pub const GITHUB_API_PREFIX: &str = "https://api.github.com";

pub struct GitHubSource;

impl SourceBuilder for GitHubSource {
    fn build_source() -> Source {
        Source::new("github").register(UserRepoGenerator)
    }
}
