use bson::DateTime as BsonDateTime;
use chrono::Utc;

pub type Date = bson::DateTime;

pub fn now() -> Date {
    BsonDateTime::from_chrono(Utc::now())
}
