use atom_syndication::{Entry, Feed, Link, Person};
use feed_generator::FeedGenerator;
use routes::github::GITHUB_API_PREFIX;
use utils::{now, NabuResult};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct UserRepoInfo {
    username: String,
    #[serde(rename = "type")]
    ty: String,
    sort: String,
    direction: String,
}

impl Default for UserRepoInfo {
    fn default() -> Self {
        UserRepoInfo {
            username: "DCjanus".to_string(),
            ty: "owner".to_string(),
            sort: "full_name".to_string(),
            direction: "desc".to_string(),
        }
    }
}

pub struct UserRepoGenerator;

#[derive(Debug, Deserialize)]
pub struct Repo {
    id: u128,
    name: String,
    html_url: String,
    description: Option<String>,
    owner: RepoOwner,
}

#[derive(Debug, Deserialize)]
pub struct RepoOwner {
    html_url: String,
    login: String,
}

impl FeedGenerator for UserRepoGenerator {
    type Info = UserRepoInfo;

    const PATH: &'static str = "/user/repos";

    fn update(info: &Self::Info) -> NabuResult<Feed> {
        let url = format!(
            "{prefix}/users/{username}/repos",
            prefix = GITHUB_API_PREFIX,
            username = info.username,
        );
        let mut response = ::reqwest::get(&url)?;
        let text = response.text()?;
        let repos = ::serde_json::from_str::<Vec<Repo>>(&text)?;

        let result = Feed {
            id: format!(
                "https://github.com/{username}?tab=repositories",
                username = info.username
            ),
            title: format!(
                "GitHub Public Repositories for {username}",
                username = info.username
            ),
            updated: now(), // TODO 应该仅在数据更新时更新这个字段

            authors: vec![Person {
                name: info.username.clone(),
                email: None,
                uri: Some(format!(
                    "https://github.com/{username}",
                    username = info.username
                )),
            }],

            entries: Self::build_entries(&repos),
            ..Default::default()
        };

        Ok(result)
    }
}

impl UserRepoGenerator {
    fn build_entries(repos: &[Repo]) -> Vec<Entry> {
        repos
            .iter()
            .map(|x| Entry {
                id: x.html_url.clone(),
                title: x.name.clone(),
                updated: now(), // TODO 应该仅在数据更新时更新这个字段
                links: vec![Link {
                    href: x.html_url.clone(),
                    ..Default::default()
                }],
                summary: x.description.clone(),
                ..Default::default()
            }).collect()
    }
}
