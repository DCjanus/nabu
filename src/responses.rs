use crate::utils::{ATOM_MIME, TEXT_PLAIN_UTF_8};
use actix_web::{http::ContentEncoding, HttpResponse};

const CONTENT_ENCODING: ContentEncoding = ContentEncoding::Identity;

pub fn parse_query_string_failed() -> HttpResponse {
    HttpResponse::BadRequest()
        .content_encoding(CONTENT_ENCODING)
        .content_type(TEXT_PLAIN_UTF_8)
        .body("Wrong parameters in query string")
}

pub fn unexpected_error() -> HttpResponse {
    HttpResponse::InternalServerError()
        .content_encoding(CONTENT_ENCODING)
        .content_type(TEXT_PLAIN_UTF_8)
        .body("Unexpected error in server")
}

pub fn cache_hit(cache: String) -> HttpResponse {
    HttpResponse::Ok()
        .content_encoding(CONTENT_ENCODING)
        .content_type(ATOM_MIME)
        .body(cache)
}

pub fn feed_created(feed: String) -> HttpResponse {
    HttpResponse::Created()
        .content_encoding(CONTENT_ENCODING)
        .content_type(ATOM_MIME)
        .body(feed)
}
