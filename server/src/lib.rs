mod pb;
mod service;
mod record;
mod database;

use std::collections::HashMap;
use tokio::sync::RwLock;
use tonic::transport::Server;
use pb::xkcd_explorer::v1::{xkcd_explorer_server, *};
use crate::service::XkcdExplorerService;
use crate::xkcd_explorer_server::XkcdExplorerServer;

pub async fn server(
    port: u64
) -> Result<(), Box<dyn std::error::Error>> {
    let database = HashMap::new();
    let lock = RwLock::new(database);

    // Main service
    let xkcd_explorer_service = XkcdExplorerService::new(lock);

    // Liveness service
    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<XkcdExplorerServer<XkcdExplorerService>>()
        .await;

    // gRPC reflection
    let file_descriptor_set: &[u8] = tonic::include_file_descriptor_set!("descriptor");
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(file_descriptor_set)
        .build()?;

    // Create gRPC server
    let server_addr = format!("0.0.0.0:{}", port).parse()?;
    tracing::info!("Starting server: {}", &server_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(reflection_service)
        .add_service(XkcdExplorerServer::new(xkcd_explorer_service))
        .serve(server_addr)
        .await?;
    Ok(())
}