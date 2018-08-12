extern crate actix_web;
extern crate atom_syndication;
extern crate chrono;
extern crate failure;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

use actix_web::server;
use atom_hub::AtomHub;
use routes::github::GitHubSource;

pub mod atom_hub;
pub mod feed_generator;
pub mod routes;
pub mod source;
pub mod utils;

fn main() {
    server::new(|| AtomHub::new().register(GitHubSource).apps)
        .bind("0.0.0.0:8000") // TODO read config from env
        .unwrap()
        .run();
}
