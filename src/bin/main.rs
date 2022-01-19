extern crate hackerrank_meets_aoc;

use hackerrank_meets_aoc::routes::routes;
use rocket::{self, Build, Rocket};

#[rocket::launch]
fn launch() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            rocket::routes![
                routes::hello,
                routes::signup,
                routes::view_user,
                routes::login
            ],
        )
}
