extern crate hackerrank_meets_aoc;

use rocket::{self, Build, Rocket};
use hackerrank_meets_aoc::routes::routes;
// use crate::rocket::launch;

#[rocket::launch]
fn launcc() -> Rocket<Build>{
    rocket::build().mount("/", rocket::routes![routes::hello, routes::register, routes::view_user])
}

// fn main() {
//     new_user(
//         "admin".to_string(),
//         "admin@admin.admin".to_string(),
//         "toor".to_string(),
//     );
// }
