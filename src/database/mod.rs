use config::pg_url;
use r2d2::{Pool, PooledConnection};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use utils::NabuResult;

lazy_static! {
    static ref DB_POOL: Pool<PostgresConnectionManager> = init().unwrap();
}

fn init() -> NabuResult<Pool<PostgresConnectionManager>> {
    let manager = PostgresConnectionManager::new(pg_url().as_ref(), TlsMode::None)?;
    let pool = Pool::new(manager)?;

    pool.get()?.batch_execute(include_str!("./init.sql"))?;

    Ok(pool)
}

pub fn get_connection() -> NabuResult<PooledConnection<PostgresConnectionManager>> {
    DB_POOL.get().map_err(|x| x.into())
}
