extern crate hackerrank_meets_aoc;

use hackerrank_meets_aoc::routes;
use rocket::{self, Build, Rocket};

#[rocket::launch]
fn launch() -> Rocket<Build> {
    rocket::build().mount(
        "/api",
        rocket::routes![
            routes::hello,
            routes::user::get_user,
            routes::user::post_user,
            routes::user::update_user,
            routes::user::delete_user,
            routes::login::post_login,
            routes::login::post_logout,
            routes::exercise::get_exercise,
            routes::exercise::create_exercise,
            routes::exercise::update_exercise,
            routes::solution::get_solution,
            routes::solution::new_solution,
            routes::score::get_num_scores,
        ],
    )
}
