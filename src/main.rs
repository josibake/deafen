// src/main.rs
use deafen::{
    api,
    compute::LocalCompute,
    config::Config,
    services::{ClientService, ScanService, UtxoService},
    storage::MdbxDatabase,
};
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;

    let db = Arc::new(MdbxDatabase::new(config.db_path)?);
    let compute = Arc::new(LocalCompute::new());

    let utxo_service = Arc::new(UtxoService::new(db.clone()));
    let client_service = Arc::new(ClientService::new(db.clone()));
    let scan_service = Arc::new(ScanService::new(
        utxo_service.clone(),
        client_service.clone(),
        compute,
    ));

    let routes = api::routes(scan_service, client_service)
        .with(warp::cors().allow_any_origin())
        .recover(api::handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], config.port)).await;

    Ok(())
}
