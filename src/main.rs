extern crate actix_web;
extern crate atom_syndication;
extern crate chrono;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate flexi_logger;
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
use chrono::Local;
use config::{local_address, serve_mode};
use flexi_logger::Logger;
use log::Record;
use routes::atom_hub;
use std::io;

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
    Logger::with_env_or_str("actix_web=info,info")
        .format(logger_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    if let Err(init_database_error) = ::database::init() {
        error!("{:?}", init_database_error)
    } else {
        info!("init database success")
    }

    info!("Current serve mode is {:?}", serve_mode());

    server::new(|| atom_hub().into_apps())
        .bind(local_address().as_ref())
        .unwrap()
        .run();
}

pub fn logger_format(w: &mut io::Write, record: &Record) -> Result<(), io::Error> {
    write!(
        w,
        "[{}] {} [{}:{}] {}",
        Local::now().format("%Y-%m-%d %H:%M:%S %:z"),
        record.level(),
        record.module_path().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        &record.args()
    )
}
