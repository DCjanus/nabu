use routes::github::user_repos::UserRepoGenerator;
use source::{IntoSource, Source};

pub mod user_repos;

pub const GITHUB_API_PREFIX: &str = "https://api.github.com";

pub struct GitHubSource;

impl IntoSource for GitHubSource {
    fn into_source(self) -> Source {
        Source::new("github").register::<UserRepoGenerator>()
    }
}
