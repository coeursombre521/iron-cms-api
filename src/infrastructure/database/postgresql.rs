use diesel::pg::PgConnection;
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;

use crate::domain::constants::POSTGRESQL_DB_URI;
use crate::services::utils::envutil::get_env_var_as_str;

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
pub type PostgresPool = Pool<diesel::pg::PgConnection>;
pub type DBConn = PostgresPool;

pub fn db_pool() -> DBConn {
    dotenv().ok();
    let database_url = get_env_var_as_str(POSTGRESQL_DB_URI).unwrap_or_else(|_| {
        panic!("{} must be set in .env file", POSTGRESQL_DB_URI);
    });
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}
