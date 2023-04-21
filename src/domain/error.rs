#[macro_use]
use rocket::serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
#[serde(crate="rocket::serde")]
pub struct ApiError {
    pub details: String,
}