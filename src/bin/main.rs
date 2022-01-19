extern crate hackerrank_meets_aoc;

use hackerrank_meets_aoc::routes;
use rocket::{self, Build, Rocket};

#[rocket::launch]
fn launch() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/api",
            rocket::routes![
                routes::hello,
                routes::user::get_user,
                routes::user::post_user,
                routes::login::post_login,
            ],
        )
}
