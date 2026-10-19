use anyhow::Context;
use sea_orm::{Database, DatabaseConnection};

pub mod api;
// pub mod db;
pub mod entities;
#[derive(Clone, Debug)]
pub struct State {
    pub db_conn: DatabaseConnection,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let db_url = std::env::var("DATABASE_URL")?;
    let db = Database::connect(db_url).await?;

    db.get_schema_registry(module_path!().split("::").next().unwrap())
        .sync(&db)
        .await?;

    let state = State { db_conn: db };
    let app = api::create_app(state).await?;
    let address = String::from("127.0.0.1:8080");
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .context("Failed to listen")?;

    tracing::info!("Listening on http://{address}");

    axum::serve(listener, app)
        .await
        .context("Failed to serve app")?;

    // TODO: close connection on graceful shutdown

    Ok(())
}
