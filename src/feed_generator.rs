use actix_web::{http::ContentEncoding, HttpRequest, HttpResponse};
use atom_syndication::{Feed, Generator};
use serde::de::DeserializeOwned;
use utils::{NabuResult, ATOM_MIME, TEXT_PLAIN_UTF_8};

pub trait FeedGenerator {
    type Info: DeserializeOwned + PartialEq;
    fn path() -> &'static str;
    fn update(info: Self::Info) -> NabuResult<Feed>;

    fn actix_web_handler(request: &HttpRequest) -> HttpResponse {
        let parse_result = ::serde_qs::from_str::<Self::Info>(request.query_string());
        if parse_result.is_err() {
            return HttpResponse::BadRequest()
                .content_encoding(ContentEncoding::Gzip)
                .content_type(TEXT_PLAIN_UTF_8)
                .body("Wrong parameters in query string");
        }

        let info = parse_result.unwrap();
        let update_result = Self::update(info);
        if let Err(_error) = update_result {
            // TODO add log
            return HttpResponse::InternalServerError()
                .content_encoding(ContentEncoding::Gzip)
                .content_type(TEXT_PLAIN_UTF_8)
                .body("Update feed failed");
        }

        let mut feed = update_result.unwrap();

        let mut generator = Generator::default();
        generator.set_uri(Some("https://github.com/DCjanus/nabu".to_string()));
        generator.set_value("Nabu".to_string());
        feed.set_generator(Some(generator));

        // Because of copyright, remove content
        let mut entries = feed.entries().to_owned();
        entries.iter_mut().for_each(|x| x.set_content(None));
        feed.set_entries(entries);

        HttpResponse::Ok()
            .content_encoding(ContentEncoding::Gzip)
            .content_type(ATOM_MIME)
            .body(feed.to_string())
    }
}
