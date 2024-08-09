// src/api/handlers.rs
use crate::{
    compute::Compute,
    storage::{ClientStore, UtxoStore},
    Error,
};
use crate::{
    models::{RegistrationRequest, ScanRequest, TweakRequest},
    services::{ClientService, ScanService},
};
use std::sync::Arc;
use warp::{http::StatusCode, reply::json, Reply};

pub async fn handle_query<
    S: UtxoStore + ClientStore + Send + Sync + 'static,
    C: Compute + 'static,
>(
    query: ScanRequest,
    scan_service: Arc<ScanService<S, C>>,
) -> Result<impl Reply, warp::Rejection> {
    let utxos = scan_service
        .scan_utxos(query)
        .await
        .map_err(warp::reject::custom)?;
    Ok(json(&utxos))
}

pub async fn handle_tweak<
    S: UtxoStore + ClientStore + Send + Sync + 'static,
    C: Compute + 'static,
>(
    tweak_request: TweakRequest,
    scan_service: Arc<ScanService<S, C>>,
) -> Result<impl Reply, warp::Rejection> {
    let utxos = scan_service
        .get_tweaks(tweak_request)
        .await
        .map_err(warp::reject::custom)?;
    Ok(json(&utxos))
}

pub async fn handle_register<S: ClientStore + Send + Sync + 'static>(
    registration: RegistrationRequest,
    client_service: Arc<ClientService<S>>,
) -> Result<impl Reply, warp::Rejection> {
    let response = client_service
        .register_client(registration)
        .await
        .map_err(warp::reject::custom)?;
    Ok(json(&response))
}

pub async fn handle_rejection(
    err: warp::Rejection,
) -> Result<impl Reply, std::convert::Infallible> {
    let (code, message) = if let Some(e) = err.find::<Error>() {
        match e {
            Error::ClientNotFound => (StatusCode::NOT_FOUND, "Client Not Found"),
            Error::InvalidInput(_) => (StatusCode::BAD_REQUEST, "Invalid Input"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        }
    } else if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else if err.find::<warp::reject::MethodNotAllowed>().is_some() {
        (StatusCode::METHOD_NOT_ALLOWED, "Method Not Allowed")
    } else {
        eprintln!("unhandled error: {:?}", err);
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    };

    Ok(warp::reply::with_status(message.to_string(), code))
}
