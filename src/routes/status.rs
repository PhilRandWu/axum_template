use axum::{Json, Router};
use axum::routing::get;
use serde::{Deserialize, Serialize};
use log::debug;
use crate::errors::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    status: String
}

pub fn create_route() -> Router {
    Router::new().route("/status", get(get_status))
}

async fn get_status() -> Result<Json<Status>, Error> {
    debug!("Returning status");
    Ok(Json(Status {
        status: "ok".to_owned()
    }))
}