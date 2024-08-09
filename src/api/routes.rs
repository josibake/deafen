// src/api/routes.rs
use super::handlers;
use crate::services::{ClientService, ScanService};
use crate::{
    compute::Compute,
    storage::{ClientStore, UtxoStore},
};
use std::sync::Arc;
use warp::Filter;

pub fn routes<S: UtxoStore + ClientStore + Send + Sync + 'static, C: Compute + 'static>(
    scan_service: Arc<ScanService<S, C>>,
    client_service: Arc<ClientService<S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    query_route(scan_service.clone())
        .or(register_route(client_service))
        .or(tweak_route(scan_service))
}

fn query_route<S: UtxoStore + ClientStore + Send + Sync + 'static, C: Compute + 'static>(
    scan_service: Arc<ScanService<S, C>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("query")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_scan_service(scan_service))
        .and_then(handlers::handle_query)
}

fn tweak_route<S: UtxoStore + ClientStore + Send + Sync + 'static, C: Compute + 'static>(
    scan_service: Arc<ScanService<S, C>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("tweak")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_scan_service(scan_service))
        .and_then(handlers::handle_tweak)
}

fn with_scan_service<S: UtxoStore + ClientStore + Send + Sync + 'static, C: Compute + 'static>(
    scan_service: Arc<ScanService<S, C>>,
) -> impl Filter<Extract = (Arc<ScanService<S, C>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || scan_service.clone())
}

fn register_route<S: ClientStore + Send + Sync + 'static>(
    client_service: Arc<ClientService<S>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_client_service(client_service))
        .and_then(handlers::handle_register)
}

fn with_client_service<S: ClientStore + Send + Sync + 'static>(
    client_service: Arc<ClientService<S>>,
) -> impl Filter<Extract = (Arc<ClientService<S>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client_service.clone())
}
