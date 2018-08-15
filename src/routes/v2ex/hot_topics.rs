use serde_json;

use atom_syndication::{Category, Entry, Feed, FixedDateTime, Link, Person};
use feed_generator::FeedGenerator;
use reqwest;
use utils::{now, NabuResult};

pub struct HotTopicsGenerator;

#[derive(Debug, Deserialize)]
struct Topic {
    node: Node,
    member: Member,
    #[serde(deserialize_with = "::utils::secord_to_datetime")]
    last_modified: FixedDateTime,
    id: i128,
    content: String,
    title: String,
    #[serde(deserialize_with = "::utils::secord_to_datetime")]
    created: FixedDateTime,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Node {
    name: String,
    title: String,
    url: String,
    id: i128,
}

#[derive(Debug, Deserialize)]
struct Member {
    username: String,
    url: String,
    id: i128,
}

impl FeedGenerator for HotTopicsGenerator {
    type Info = ();

    fn path() -> &'static str {
        "topics/hot"
    }

    fn update(_: &Self::Info) -> NabuResult<Feed> {
        let url = "https://www.v2ex.com/api/topics/hot.json";
        let text = reqwest::get(url)?.text()?;
        let topics: Vec<Topic> = serde_json::from_str(&text)?;

        let result = Feed {
            title: "V2ex每日热点".to_string(),
            id: "V2ex Hot Topics".to_string(),
            updated: now(),
            links: vec![Link {
                href: "https://www.v2ex.com/".to_string(),
                ..Default::default()
            }],
            entries: Self::build_entries(&topics),
            ..Default::default()
        };

        Ok(result)
    }
}

impl HotTopicsGenerator {
    fn build_entries(topics: &[Topic]) -> Vec<Entry> {
        topics
            .iter()
            .map(|x| Entry {
                title: x.title.clone(),
                id: format!("{}", x.id),
                updated: x.last_modified,
                authors: vec![Person {
                    name: x.member.username.clone(),
                    email: None,
                    uri: Some(x.member.url.clone()),
                }],
                categories: vec![Category {
                    term: x.node.name.clone(),
                    scheme: None,
                    label: Some(x.node.title.clone()),
                }],
                links: vec![Link {
                    href: x.url.clone(),
                    ..Default::default()
                }],
                published: Some(x.created),
                summary: Some(x.content.clone()),
                ..Default::default()
            }).collect()
    }
}
