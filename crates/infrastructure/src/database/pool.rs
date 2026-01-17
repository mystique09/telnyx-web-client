use rbatis::{DefaultPool, RBatis, rbdc};
use rbdc_pg::{driver::PgDriver, options::PgConnectOptions};
use std::str::FromStr;
use std::sync::Arc;

pub async fn create_db_pool(db_url: &str) -> Result<Arc<RBatis>, rbdc::Error> {
    let db = RBatis::new();
    let db = Arc::new(db);

    let driver = rbdc_pg::driver::PgDriver {};

    let conn_options = PgConnectOptions::from_str(db_url)?;

    db.init_option::<PgDriver, PgConnectOptions, DefaultPool>(driver, conn_options)?;

    let _ = db.acquire().await?;

    Ok(db)
}
