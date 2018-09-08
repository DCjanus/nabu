use atom_syndication::{Feed, FixedDateTime};
use config::cache_duration;
use database::get_connection;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::to_value;
use std::hash::Hash;
use utils::{now, NabuResult};

pub trait FeedGenerator {
    type Info: DeserializeOwned + Serialize + Default + Hash;

    const PATH: &'static str;

    fn update(info: &Self::Info) -> NabuResult<Feed>;

    fn get_cache(path: &str, info: &Self::Info) -> NabuResult<Option<String>> {
        let query_result = get_connection()?
            .query(r"SELECT updated_time, content FROM fetch_cache WHERE path=$1 AND info@> $2 AND info<@ $2 limit 1", &[
                &path, &to_value(info)?
            ])?;
        if query_result.is_empty() {
            return Ok(None);
        }
        let row = query_result.get(0);

        let updated_time: FixedDateTime = row.get(0);
        let content: String = row.get(1);

        if now().signed_duration_since(updated_time).to_std()? > cache_duration() {
            Ok(None)
        } else {
            Ok(Some(content))
        }
    }

    fn set_cache(path: &str, info: &Self::Info, content: &str) -> NabuResult<()> {
        get_connection()?.execute(
            r#"INSERT INTO fetch_cache(path, info, content)
                            VALUES ($1, $2, $3)
                            ON CONFLICT ON CONSTRAINT logic_unique_key DO UPDATE
                                SET content=$3"#,
            &[&path, &to_value(info)?, &content],
        )?;
        Ok(())
    }
}
