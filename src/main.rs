extern crate actix_web;
extern crate atom_syndication;
extern crate chrono;
extern crate failure;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_qs;

use actix_web::server;
use atom_hub::AtomHub;

pub mod atom_hub;
pub mod feed_generator;
pub mod source;
pub mod utils;

fn main() {
    server::new(|| AtomHub::new().apps)
        .bind("0.0.0.0:8000") // TODO read config from env
        .unwrap()
        .run();
}
