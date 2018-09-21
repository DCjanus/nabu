use atom_syndication::{Entry, Feed, FixedDateTime, Link, Person};
use feed_generator::FeedGenerator;
use utils::{now, NabuResult};

pub struct CrateVersionsGenerator;

#[derive(Debug, Deserialize, Serialize, Hash)]
#[serde(default)]
pub struct CrateVersionInfo {
    name: String,
}

impl Default for CrateVersionInfo {
    fn default() -> Self {
        Self {
            name: "lone-ranger".to_string(),
        }
    }
}

impl FeedGenerator for CrateVersionsGenerator {
    type Info = CrateVersionInfo;

    const PATH: &'static str = "versions";

    fn update(info: &Self::Info) -> NabuResult<Feed> {
        let versions_url = format!("https://crates.io/api/v1/crates/{name}", name = info.name);
        let versions = ::reqwest::get(&versions_url)?
            .json::<CrateInfoResponse>()?
            .versions;
        let result = Feed {
            id: format!("https://crates.io/crates/{name}", name = info.name),
            title: format!("Latest 10 Versions of Crate '{name}'", name = info.name),
            updated: now(), // TODO 应该仅在数据更新时更新这个字段
            entries: Self::build_entries(&versions[..10])?,
            ..Default::default()
        };
        Ok(result)
    }
}

impl CrateVersionsGenerator {
    fn build_entries(versions: &[VersionInfo]) -> NabuResult<Vec<Entry>> {
        let mut result = Vec::with_capacity(versions.len());
        for i in versions {
            result.push(i.to_entry()?);
        }
        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
struct CrateInfoResponse {
    versions: Vec<VersionInfo>,
}

#[derive(Debug, Deserialize)]
struct VersionInfo {
    #[serde(rename = "crate")]
    name: String,
    num: String,
    updated_at: FixedDateTime,
    created_at: FixedDateTime,
    license: String,
}

impl VersionInfo {
    pub fn authors(&self) -> NabuResult<Vec<Person>> {
        let author_url = format!(
            "https://crates.io/api/v1/crates/{name}/{version}/authors",
            name = self.name,
            version = self.num
        );
        let result = ::reqwest::get(&author_url)?
            .json::<AuthorsResponse>()?
            .meta
            .names
            .into_iter()
            .map(|x| Person {
                name: x,
                ..Default::default()
            }).collect();
        Ok(result)
    }

    pub fn url(&self) -> String {
        format!(
            "https://crates.io/crates/{name}/{version}",
            name = self.name,
            version = self.num
        )
    }

    pub fn to_entry(&self) -> NabuResult<Entry> {
        Ok(Entry {
            title: format!("{} {}", self.name, self.num),
            id: self.url(),
            updated: self.updated_at,
            authors: self.authors()?,
            links: vec![Link {
                href: self.url(),
                ..Default::default()
            }],
            published: Some(self.created_at),
            rights: Some(self.license.clone()),
            ..Default::default()
        })
    }
}

#[derive(Debug, Deserialize)]
struct AuthorsResponse {
    meta: AuthorsMeta,
}

#[derive(Debug, Deserialize)]
struct AuthorsMeta {
    names: Vec<String>,
}
