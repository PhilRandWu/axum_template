use crate::errors::Error;
use axum::routing::get;
use axum::{Json, Router};
use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Status {
    status: String,
}

pub fn create_route() -> Router {
    Router::new().route("/status", get(get_status))
}

async fn get_status() -> Result<Json<Status>, Error> {
    debug!("Returning status");
    Ok(Json(Status {
        status: "ok".to_owned(),
    }))
}
