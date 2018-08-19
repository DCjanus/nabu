use actix_web::{http::ContentEncoding, HttpRequest, HttpResponse};
use atom_syndication::{Feed, FixedDateTime, Generator};
use config::{cache_duration, serve_mode, ServeMode};
use database::get_connection;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::to_value;
use utils::{now, NabuResult, ATOM_MIME, TEXT_PLAIN_UTF_8};

// Compression is no longer needed after using Caddy
const CONTENT_ENCODING: ContentEncoding = ContentEncoding::Identity;

pub trait FeedGenerator {
    type Info: DeserializeOwned + Serialize + Default;
    const PATH: &'static str;
    fn update(info: &Self::Info) -> NabuResult<Feed>;

    fn actix_web_handler(request: &HttpRequest) -> HttpResponse {
        let query_string = request.query_string();
        let info = if query_string.is_empty() {
            Self::Info::default()
        } else {
            let parse_result = ::serde_qs::from_str::<Self::Info>(query_string);
            if let Err(error) = parse_result {
                error!("{:?}", error);
                return HttpResponse::BadRequest()
                    .content_encoding(CONTENT_ENCODING)
                    .content_type(TEXT_PLAIN_UTF_8)
                    .body("Wrong parameters in query string");
            }
            parse_result.unwrap()
        };

        if serve_mode() != ServeMode::Dev {
            // Logic for cache
            let get_cache_result = Self::get_cache(request.path(), &info);
            if let Ok(cache_result) = get_cache_result {
                if let Some(cache) = cache_result {
                    return HttpResponse::Ok()
                        .content_encoding(CONTENT_ENCODING)
                        .content_type(ATOM_MIME)
                        .body(cache);
                }
            } else if let Err(error) = get_cache_result {
                error!("{:?}", error);
                return HttpResponse::InternalServerError()
                    .content_encoding(CONTENT_ENCODING)
                    .content_type(TEXT_PLAIN_UTF_8)
                    .body("Query cache failed");
            }
        }

        let update_result = Self::update(&info);
        if let Err(error) = update_result {
            error!("{:?}", error);
            return HttpResponse::InternalServerError()
                .content_encoding(CONTENT_ENCODING)
                .content_type(TEXT_PLAIN_UTF_8)
                .body("Update feed failed");
        }

        let mut feed = update_result.unwrap();

        let generator = Generator {
            value: "Nabu".to_string(),
            uri: Some("https://github.com/DCjanus/nabu".to_string()),
            version: None,
        };
        feed.set_generator(Some(generator));

        // Because of copyright, remove content
        feed.entries.iter_mut().for_each(|x| x.set_content(None));

        let content = feed.to_string();

        // below: logic for cache
        if let Err(error) = Self::set_cache(request.path(), &info, &content) {
            error!("{:?}", error);
            return HttpResponse::InternalServerError()
                .content_encoding(CONTENT_ENCODING)
                .content_type(TEXT_PLAIN_UTF_8)
                .body("Set cache failed");
        }
        // above: logic for cache

        HttpResponse::Created()
            .content_encoding(CONTENT_ENCODING)
            .content_type(ATOM_MIME)
            .body(&content)
    }

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
