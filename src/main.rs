use rockets::{
    PgConnection,
    controller::post::post_controller::{list, retrieve, create, update, destroy},
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