use crate::settings::SETTINGS;
use mongodb::Database;
use tokio::sync::OnceCell;
use wither::mongodb;

static CONNECTION: OnceCell<Database> = OnceCell::const_new();

pub async fn connection() -> &'static Database {
    CONNECTION
        .get_or_init(|| async {
            let db_url = SETTINGS.database.url.as_str();
            let db_name = SETTINGS.database.name.as_str();
            mongodb::Client::with_uri_str(db_url)
                .await
                .expect("Failed to initialize MongoDB connection")
                .database(db_name)
        })
        .await
}
