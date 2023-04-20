use std::env::args;

use diesel::prelude::*;

use rockets::*;
use rockets::schema::posts::title;

fn main() {
    use self::schema::posts::dsl::*;

    let target = args().nth(1).expect("publish_post requires a target");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");
    println!("Deleted {} posts", num_deleted);
}