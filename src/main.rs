use diesel::prelude::*;
use rocket::{
    response::status::{Created, NoContent, NotFound},
    serde::json::Json,
};

use rockets::{
    ApiError,
    models::post::{NewPost, Post, UpdatePost},
    PgConnection,
    schema::posts,
};

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .attach(PgConnection::fairing())
        .mount(
            "/posts",
            rocket::routes![list, retrieve, create, update, destroy],
        )
}

#[rocket::get("/")]
async fn list(conn: PgConnection) -> Json<Vec<Post>> {
    conn.run(|c| posts::table.load(c)).await.map(Json).expect("Error loading posts")
}

#[rocket::get("/<id>")]
async fn retrieve(conn: PgConnection, id: i32) -> Result<Json<Post>, NotFound<Json<ApiError>>> {
    conn.run(move |c| posts::table.filter(posts::id.eq(id)).first(c))
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[rocket::post("/", data = "<post>")]
async fn create(
    conn: PgConnection,
    post: Json<NewPost>,
) -> Result<Created<Json<Post>>, Json<ApiError>> {
    conn.run(move |c| {
        diesel::insert_into(posts::table)
            .values(&post.into_inner())
            .get_result(c)
    })
        .await
        .map(|a| Created::new("/posts").body(Json(a)))
        .map_err(|e| {
            Json(ApiError {
                details: e.to_string(),
            })
        })
}

#[rocket::put("/<id>", data = "<post>")]
async fn update(
    conn: PgConnection,
    id: i32,
    post: Json<UpdatePost>,
) -> Result<Json<Post>, NotFound<Json<ApiError>>> {
    conn.run(move |c| {
        diesel::update(posts::table.find(id))
            .set(&post.into_inner())
            .get_result(c)
    })
        .await
        .map(Json)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}

#[rocket::delete("/<id>")]
async fn destroy(conn: PgConnection, id: i32) -> Result<NoContent, NotFound<Json<ApiError>>> {
    conn.run(move |c| {
        let affected = diesel::delete(posts::table.filter(posts::id.eq(id)))
            .execute(c)
            .expect("Error deleting post");

        match affected {
            1 => Ok(NoContent),
            0 => Err("No post found"),
            _ => Err("Multiple posts found"),
        }
    })
        .await
        .map(|_| NoContent)
        .map_err(|e| {
            NotFound(Json(ApiError {
                details: e.to_string(),
            }))
        })
}