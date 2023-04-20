use diesel::{AsChangeset, Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(Queryable, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "posts"]
pub struct NewPost {
    pub title: String,
    pub body: String,
}

#[derive(AsChangeset, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
#[table_name = "posts"]
pub struct UpdatePost {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}