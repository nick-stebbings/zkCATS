use std::time::Duration;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tracing::info;

const PG_DEV_POSTGRES_URL: &str = "postgres://postgres:welcome@localhost/postgres";
const PG_DEV_APP_URL: &str = "postgres://";


type Db = Pool<Postgres>;

pub async fn init_dev_db() -> Result<(), Box<dyn std::error::Error>> {
    info!("{:<12} - init_dev_db()", "FOR-DEV-ONLY");

    // -- Create app_db/app_user with the postgres user

    {
        let root_db = new_db_pool(PG_DEV_POSTGRES_URL);
    }
    Ok(())
}

async fn new_db_pool(db_conn_url: &str) -> Result<Db, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_conn_url)
        .await
}