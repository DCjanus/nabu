extern crate actix_web;
extern crate atom_syndication;
extern crate chrono;
extern crate env_logger;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
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
use config::{local_address, serve_mode};
use routes::atom_hub;

pub mod atom_hub;
pub mod config;
pub mod database;
pub mod errors;
pub mod feed_generator;
pub mod feed_worker;
pub mod responses;
pub mod routes;
pub mod source;
pub mod utils;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();

    info!("Current serve mode is {:?}", serve_mode());

    server::new(|| atom_hub().into_apps())
        .bind(local_address().as_ref())
        .unwrap()
        .run();
}
