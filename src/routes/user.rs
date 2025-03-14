use axum::{Json, Router};
use axum::http::StatusCode;
use axum::routing::post;
use serde::Deserialize;
use crate::errors::Error;
use crate::models::user::{PublicUser, User};
use crate::models::user;
use crate::utils::custom_response::{CustomResponse, CustomResponseBuilder};
use crate::utils::models::ModelExt;

pub fn create_route() -> Router{
    Router::new()
        .route("/users", post(create_user))
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
    password: String,
}

async fn create_user(Json(body): Json<CreateUser>) -> Result<CustomResponse<PublicUser>, Error> {
    let password_hash = user::hash_password(body.password).await?;
    let user = User::new(body.name, body.email, password_hash);
    let user = User::create(user).await?;
    let public_user = PublicUser::from(user);

    let res = CustomResponseBuilder::new()
        .body(public_user)
        .status_code(StatusCode::CREATED)
        .build();
    Ok(res)
}