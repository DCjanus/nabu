use crate::{config::pg_url, utils::NabuResult};
use lazy_static::lazy_static;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};

lazy_static! {
    static ref DB_POOL: Pool<PostgresConnectionManager> = init().unwrap();
}

pub fn init() -> NabuResult<Pool<PostgresConnectionManager>> {
    let manager = PostgresConnectionManager::new(pg_url().as_ref(), TlsMode::None)?;
    let pool = Pool::new(manager)?;

    pool.get()?.batch_execute(include_str!("./init.sql"))?;

    Ok(pool)
}

pub fn get_connection() -> NabuResult<PooledConnection<PostgresConnectionManager>> {
    Ok(DB_POOL.get()?)
}
