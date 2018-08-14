extern crate actix_web;
extern crate atom_syndication;
extern crate chrono;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate lazy_static;
extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

use actix_web::server;
use atom_hub::AtomHub;
use config::local_address;
use routes::github::GitHubSource;

pub mod atom_hub;
pub mod config;
pub mod database;
pub mod feed_generator;
pub mod routes;
pub mod source;
pub mod utils;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    server::new(|| AtomHub::new().register(GitHubSource).apps)
        .bind(local_address().as_ref())
        .unwrap()
        .run();
}
