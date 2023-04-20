pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
use rocket::serde::{Serialize, Deserialize};
use rocket_sync_db_pools::database;

#[database("rockets")]
pub struct PgConnection(diesel::PgConnection);

#[derive(Queryable, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct ApiError {
    pub details: String,
}