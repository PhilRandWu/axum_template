use crate::database;
use crate::errors::Error;
use async_trait::async_trait;
use bson::Document;
use validator::Validate;
use wither::mongodb::options::{FindOneOptions, UpdateOptions};
use wither::mongodb::results::UpdateResult;
use wither::Model as WitherModel;

#[async_trait]
pub trait ModelExt
where
    Self: WitherModel + Validate,
{
    async fn create(mut model: Self) -> Result<Self, Error> {
        let connection = database::connection().await;
        model.validate().map_err(|_error| Error::bad_request())?;
        model.save(connection, None).await.map_err(Error::Wither)?;
        Ok(model)
    }

    async fn find_one<O>(query: Document, options: O) -> Result<Option<Self>, Error>
    where
        O: Into<Option<FindOneOptions>> + Send,
    {
        let connection = database::connection().await;
        <Self as WitherModel>::find_one(connection, query, options)
            .await
            .map_err(Error::Wither)
    }

        async fn update_one<O>(
        query: Document,
        update: Document,
        options: O,
    ) -> Result<UpdateResult, Error>
    where
        O: Into<Option<UpdateOptions>> + Send,
    {
        let connection = database::connection().await;
        Self::collection(connection)
            .update_one(query, update, options)
            .await
            .map_err(Error::Mongo)
    }
}
