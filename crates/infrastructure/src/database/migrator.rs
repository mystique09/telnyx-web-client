use bb8::Pool;
use bb8_postgres::{PostgresConnectionManager, tokio_postgres::NoTls};
use std::error::Error;
use tracing::info;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/migrations/");
}

pub async fn migrator(db_url: &str) -> Result<(), Box<dyn Error>> {
    let manager = PostgresConnectionManager::new_from_stringlike(db_url, NoTls)?;
    let pool = Pool::builder().max_size(15).build(manager).await?;
    let mut conn = pool.dedicated_connection().await?;

    let report = embedded::migrations::runner().run_async(&mut conn).await?;

    for migrated in report.applied_migrations() {
        info!("Applied migration: {}", migrated);
    }

    Ok(())
}
