use r2d2::{Config, Pool, PooledConnection as R2PooledConnection};
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

pub use diesel::pg::PgConnection as DieselPgConnection;
pub use diesel::result::Error as DBError;

pub type PooledConnection = R2PooledConnection<ConnectionManager<DieselPgConnection>>;

pub fn establish() -> PooledConnection {
    dotenv().ok();
    let config = Config::default();
    let manager = ConnectionManager::<DieselPgConnection>::new(connection_url());
    let pool = Pool::new(config, manager).expect("Failed to create pool.");
    pool.get().unwrap()
}

fn connection_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}