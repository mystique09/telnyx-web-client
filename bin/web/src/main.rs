use std::sync::Arc;

use actix_web::{HttpServer, dev::ServerHandle, rt::signal};
use infrastructure::{
    config::{database::DatabaseConfig, web::WebConfig},
    database::{migrator::migrator, pool::create_db_pool},
    repositories::user_repository_impl::UserRepositoryImpl,
    security::argon2_hasher::Argon2Hasher,
    security::paseto_tokenizer::PasetoAuthenticationTokenService,
};
use tracing::{info, subscriber::set_global_default};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
use web::server::create_web_service;

#[actix_web::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    setup_tracing()?;
    dotenvy::dotenv()?;

    let config = WebConfig::from_env()?;
    let db_config = DatabaseConfig::from_env()?;

    let pool = create_db_pool(&db_config.url).await?;
    let _ = migrator(&db_config.url).await;

    let user_repository = Arc::new(UserRepositoryImpl::builder().pool(pool).build());
    let password_hasher = Arc::new(Argon2Hasher::new());
    let token_service = Arc::new(PasetoAuthenticationTokenService::new(
        &config.paseto_symmetric_key,
    )?);

    let server = HttpServer::new(move || {
        create_web_service(
            user_repository.clone(),
            password_hasher.clone(),
            token_service.clone(),
        )
    })
    .workers(5)
    .bind(&config.addrs())?
    .shutdown_timeout(5)
    .run();

    let handle = server.handle();

    actix::spawn(async move {
        shutdown_signal(handle).await;
    });

    info!("listening on {}", config.addrs());

    server.await?;

    Ok(())
}

fn setup_tracing() -> eyre::Result<()> {
    let crate_name = env!("CARGO_CRATE_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    LogTracer::init()?;

    let default_filter = format!("info,{}=debug,tokio=info", crate_name);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_filter));

    let subscriber = Registry::default()
        .with(tracing_subscriber::fmt::layer())
        .with(env_filter);

    set_global_default(subscriber)?;

    info!("[IDATE-EMU] {} v{}", crate_name, crate_version);
    Ok(())
}

pub async fn shutdown_signal(handle: ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to initialize Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to initialize signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("gracefully shutting down server...");
    info!("server shutdown complete");
    info!("goodbye!");

    handle.stop(true).await;
}
