use crate::errors::Error;
use crate::utils::date;
use crate::utils::date::Date;
use crate::utils::models::ModelExt;
use bcrypt::{hash_with_salt, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use tokio::task;
use validator::Validate;
use wither::bson::oid::ObjectId;
use wither::Model as WitherModel;

#[derive(Debug, Clone, Serialize, Deserialize, WitherModel, Validate)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
    pub updated_at: Date,
    pub created_at: Date,
    pub locked_at: Option<Date>,
}

impl ModelExt for User {}

impl User {
    pub fn new<A, B, C>(name: A, email: B, password: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        let now = date::now();
        Self {
            id: None,
            name: name.into(),
            email: email.into(),
            password: password.into(),
            updated_at: now,
            created_at: now,
            locked_at: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub updated_at: Date,
    pub created_at: Date,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id.unwrap(),
            name: user.name.clone(),
            email: user.email.clone(),
            updated_at: user.updated_at,
            created_at: user.created_at,
        }
    }
}

pub async fn hash_password<P>(password: P) -> Result<String, Error>
where
    P: AsRef<str> + Send + 'static,
{
    #[cfg(not(test))]
    let cost = DEFAULT_COST;
    #[cfg(test)]
    let cost = 4;

    let salt = [0; 16]; // 自定义盐值
    task::spawn_blocking(move || {
        hash_with_salt(password.as_ref().as_bytes(), cost, salt).map(|parts| parts.to_string())
    })
    .await
    .map_err(Error::RunSyncTask)?
    .map_err(Error::HashPassword)
}
