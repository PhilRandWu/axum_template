mod settings;

use std::net::SocketAddr;
use tokio::net::TcpListener;
// use tracing::info;
//
mod app;
// mod database;
// mod errors;
// mod logger;
// mod models;
mod routes;
mod errors;
mod logger;
// mod settings;
// mod utils;

// #[cfg(test)]
// mod tests;

// use errors::Error;
use settings::SETTINGS;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = SETTINGS.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    let app = app::create_app().await;

    let listener = TcpListener::bind(address).await?;
    // info!("Server listening on {}", &address);

    axum::serve(listener, app).await
}
