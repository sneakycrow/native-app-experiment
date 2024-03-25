use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::env;
use tokio::time::Duration;

// Get the database string from the environment
fn get_dsn_from_env() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL not set")
}

// Get a database connection
pub async fn get_db_connection() -> DatabaseConnection {
    let dsn = get_dsn_from_env();
    let mut opt = ConnectOptions::new(dsn);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public");

    match Database::connect(opt).await {
        Ok(conn) => conn,
        Err(err) => match err {
            _ => panic!("Could not get db connection\n Err: {}", err),
        },
    }
}
