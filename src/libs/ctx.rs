use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

use crate::libs::config::CONFIG;
use sqlx::ConnectOptions;

#[derive(Clone)]
pub struct Context {
    pub db: SqlitePool,
}

impl Context {
    pub(crate) async fn new() -> Self {
        let db_uri = CONFIG.db_uri.as_str();
        debug!("Connect to database: {}", db_uri);

        let mut options = SqliteConnectOptions::from_str(db_uri)
            .unwrap().create_if_missing(true);
        options.log_statements(log::LevelFilter::Debug);
        let db = SqlitePool::connect_with(options).await.unwrap();

        sqlx::migrate!("./migrations").run(&db).await.unwrap();

        Context { db }
    }
}
