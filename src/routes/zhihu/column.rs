use atom_syndication::{Category, Content, Entry, Feed, FixedDateTime, Link, Person};
use crate::{errors::WrongResponse, feed_generator::FeedGenerator, utils::NabuResult};
use serde::{Deserialize, Serialize};

/// 知乎专栏
pub struct ColumnGenerator;

impl FeedGenerator for ColumnGenerator {
    type Info = ZhihuColumnInfo;

    const PATH: &'static str = "column";

    fn update(info: &Self::Info) -> NabuResult<Feed> {
        let articles_url = format!(
            "https://www.zhihu.com/api/v4/columns/{name}/articles?include=data[*].admin_closed_comment,comment_count,suggest_edit,is_title_image_full_screen,can_comment,upvoted_followees,can_open_tipjar,can_tip,voteup_count,voting,topics,review_info,author.is_following",
            name = info.name
        );
        let mut response = ::reqwest::get(&articles_url)?;
        let text = response.text()?;
        let data = ::serde_json::from_str::<::serde_json::Value>(&text)?
            .get("data")
            .ok_or_else(|| WrongResponse { text })?
            .to_owned();
        let articles = ::serde_json::from_value::<Vec<Article>>(data)?;

        let authors_url = format!(
            "https://zhuanlan.zhihu.com/api/columns/{name}/authors?filter=all",
            name = info.name
        );
        let mut response = ::reqwest::get(&authors_url)?;
        let text = response.text()?;
        let authors = ::serde_json::from_str::<Vec<User>>(&text)?;

        let column_url = format!(
            "https://www.zhihu.com/api/v4/columns/{name}",
            name = info.name
        );
        let mut response = ::reqwest::get(&column_url)?;
        let text = response.text()?;
        let column_info = ::serde_json::from_str::<ZhihuColumn>(&text)?;

        let result = Feed {
            id: format!("https://zhuanlan.zhihu.com/{name}", name = info.name),
            title: format!("Articles in Zhihu Column {name}", name = info.name),
            updated: column_info.updated,

            authors: authors.into_iter().map(|x| x.into_feed_person()).collect(),
            entries: Self::build_entries(&articles),

            ..Default::default()
        };

        Ok(result)
    }
}

impl ColumnGenerator {
    pub fn build_entries(articles: &[Article]) -> Vec<Entry> {
        articles
            .iter()
            .cloned()
            .map(|x| Entry {
                title: x.title.clone(),
                id: x.url.clone(),
                updated: x.updated,
                authors: vec![x.author.into_feed_person()],
                categories: x
                    .topics
                    .into_iter()
                    .map(|x| x.into_feed_category())
                    .collect(),
                links: vec![Link {
                    href: x.url.clone(),
                    title: Some(x.title.clone()),
                    ..Default::default()
                }],
                published: Some(x.created),
                content: Some(Content {
                    value: Some(x.excerpt),
                    src: Some(x.url.clone()),
                    content_type: Some("html".to_string()),
                }),
                ..Default::default()
            })
            .collect()
    }
}

#[derive(Debug, Deserialize, Serialize, Hash)]
#[serde(default)]
pub struct ZhihuColumnInfo {
    pub name: String,
}

impl Default for ZhihuColumnInfo {
    fn default() -> Self {
        Self {
            name: "taowen".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub struct Article {
    pub author: User,
    pub id: u64,
    #[serde(deserialize_with = "crate::utils::secord_to_datetime")]
    pub created: FixedDateTime,
    #[serde(deserialize_with = "crate::utils::secord_to_datetime")]
    pub updated: FixedDateTime,
    pub excerpt: String,
    pub title: String,
    pub url: String,
    pub topics: Vec<ZhihuArticleTopic>,
}

#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub struct User {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub struct ZhihuColumn {
    pub title: String,
    pub url: String,
    #[serde(deserialize_with = "crate::utils::secord_to_datetime")]
    pub updated: FixedDateTime,
}

#[derive(Debug, Deserialize, Serialize, Hash, Clone)]
pub struct ZhihuArticleTopic {
    pub url: String,
    pub name: String,
    pub id: String,
}

impl ZhihuArticleTopic {
    pub fn into_feed_category(self) -> Category {
        Category {
            term: self.id,
            scheme: Some(self.url),
            label: Some(self.name),
        }
    }
}

impl User {
    pub fn into_feed_person(self) -> Person {
        Person {
            uri: Some(self.url()),
            name: self.name,
            email: None,
        }
    }

    pub fn url(&self) -> String {
        format!("https://www.zhihu.com/people/{name}", name = self.name)
    }
}
