//! Main entry point for the API server
//!
//! This module sets up the API server using poem and poem-openapi,
//! configures the routes, and starts the server.

use std::env;
use std::net::SocketAddr;

use anyhow::Result;
use db::establish_connection;
use dotenv::dotenv;
use log::{info, warn};
use poem::{listener::TcpListener, middleware::Cors, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

mod api;
mod error;

use api::{IndexerApi, MarketplaceApi, StarAtlasApi};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize environment
    dotenv().ok();

    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .init();

    // Get configuration from environment variables
    let host = env::var("API_HOST").unwrap_or_else(|_| {
        warn!("API_HOST not set, using default 127.0.0.1");
        "127.0.0.1".to_string()
    });
    let port = env::var("API_PORT")
        .unwrap_or_else(|_| {
            warn!("API_PORT not set, using default 3000");
            "3000".to_string()
        })
        .parse::<u16>()
        .expect("API_PORT must be a valid port number");

    // Establish database connection
    let db_pool = establish_connection().await?;

    // Create API instances
    let indexer_api = IndexerApi::new(db_pool.clone());
    let marketplace_api = MarketplaceApi::new(db_pool.clone());
    let staratlas_api = StarAtlasApi::new(db_pool);

    // Create OpenAPI service
    let api_service = OpenApiService::new(
        (indexer_api, marketplace_api, staratlas_api),
        "Rogue Data Hub API",
        env!("CARGO_PKG_VERSION"),
    )
    .server(format!("http://{}:{}", host, port));

    // Get the OpenAPI specification
    let spec = api_service.spec();

    // Create the API routes
    let ui = api_service.swagger_ui();
    let spec_json = api_service.spec_endpoint();

    // Set up the routes
    let app = Route::new()
        .nest("/", api_service)
        .nest("/doc", ui)
        .at("/spec", poem::endpoint::make_sync(move |_| spec.clone()))
        .with(Cors::new());

    // Start the server
    let addr = format!("{}:{}", host, port).parse::<SocketAddr>()?;
    info!("Starting API server at http://{}", addr);
    Server::new(TcpListener::bind(addr)).run(app).await?;

    Ok(())
}
