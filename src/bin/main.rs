extern crate hackerrank_meets_aoc;

use hackerrank_meets_aoc::routes::routes;
use rocket::{self, Build, Rocket};
// use crate::rocket::launch;

#[rocket::launch]
fn launcc() -> Rocket<Build> {
    rocket::build().mount(
        "/",
        rocket::routes![routes::hello, routes::register, routes::view_user],
    )
}

// fn main() {
//     new_user(
//         "admin".to_string(),
//         "admin@admin.admin".to_string(),
//         "toor".to_string(),
//     );
// }
