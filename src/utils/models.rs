use async_trait::async_trait;
use crate::database;
use validator::Validate;
use crate::errors::Error;
use wither::Model as WitherModel;

#[async_trait]
pub trait ModelExt where Self: WitherModel + Validate {
    async fn create(mut model: Self) -> Result<Self, Error> {
        let connection = database::connection().await;
        model.validate().map_err(|_error| Error::bad_request())?;
        model.save(connection, None).await.map_err(Error::Wither)?;
        Ok(model)
    }
}
