use crate::errors::{AuthenticateError, Error};
use crate::models::user;
use crate::models::user::{PublicUser, User};
use crate::settings::SETTINGS;
use crate::utils::custom_response::{CustomResponse, CustomResponseBuilder};
use crate::utils::models::ModelExt;
use crate::utils::{date, token};
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Json, Router};
use bson::doc;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub fn create_route() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/authenticate", post(authenticate_user))
        .route("/users", delete(soft_delete_by_token))
}

#[derive(Debug, Deserialize)]
struct CreateBody {
    name: String,
    email: String,
    password: String,
}

async fn create_user(Json(body): Json<CreateBody>) -> Result<CustomResponse<PublicUser>, Error> {
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

#[derive(Debug, Deserialize)]
pub struct AuthorizeBody {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticateResponse {
    pub access_token: String,
    pub user: PublicUser,
}

async fn authenticate_user(
    Json(body): Json<AuthorizeBody>,
) -> Result<Json<AuthenticateResponse>, Error> {
    let email = &body.email;
    let password = &body.password;

    if email.is_empty() {
        debug!("Missing email, returning 400 status code");
        return Err(Error::bad_request());
    }

    if password.is_empty() {
        debug!("Missing password, returning 400 status code");
        return Err(Error::bad_request());
    }

    let user = User::find_one(doc! { "email": email}, None).await?;
    let user = match user {
        Some(user) => user,
        None => return Err(Error::not_found()),
    };

    if !user.is_password_match(password) {
        debug!("Invalid password, returning 400 status code");
        return Err(Error::Authenticate(AuthenticateError::WrongCredentials));
    }

    if user.locked_at.is_some() {
        debug!("User is locked, returning 401 status code");
        return Err(Error::Authenticate(AuthenticateError::Locked));
    }

    let secret = SETTINGS.auth.secret.as_str();
    let token = token::create(user.clone(), secret)
        .map_err(|_| Error::Authenticate(AuthenticateError::TokenCreation))?;
    let res = AuthenticateResponse {
        access_token: token,
        user: PublicUser::from(user),
    };

    Ok(Json(res))
}

#[derive(Debug, Serialize, Deserialize)]
pub  struct DeleteBody {
    pub access_token: String,
}

pub async fn soft_delete_by_token(Json(body): Json<DeleteBody>) -> Result<CustomResponse<PublicUser>, Error> {
    let access_token = &body.access_token;
    let secret = SETTINGS.auth.secret.as_str();
    let token_data = token::decode(access_token, secret)
        .map_err(|_| Error::Authenticate(AuthenticateError::InvalidToken))?;
    let user_id = token_data.claims.user.id;
    let filter = doc! { "_id": user_id };
    let now = date::now();
    let update = doc! { "$set": { "locked_at": Some(now) } }; // 更新 locked_at 字段

    // 使用 ModelExt trait 中的 update_one 方法
    User::update_one(filter, update, None).await?;

    let res = CustomResponseBuilder::new()
        .status_code(StatusCode::NO_CONTENT)
        .build();

    Ok(res)
}
