use actix_web::server;
use chrono::Local;
use crate::{
    config::{local_address, serve_mode},
    routes::atom_hub,
};
use flexi_logger::Logger;
use log::{error, info, Record};
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

    if let Err(init_database_error) = crate::database::init() {
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
