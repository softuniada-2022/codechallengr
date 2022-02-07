extern crate hackerrank_meets_aoc;

use dotenv::dotenv;
use hackerrank_meets_aoc::routes;
use rocket::{self, Build, Rocket};

#[rocket::launch]
fn launch() -> Rocket<Build> {
    dotenv().ok();
    rocket::build().mount(
        "/api",
        rocket::routes![
            routes::hello,
            routes::user::get_user,
            routes::user::post_user,
            routes::user::update_user,
            routes::user::delete_user,
            routes::user::get_likes,
            routes::user::get_users,
            routes::login::post_login,
            routes::login::post_logout,
            routes::exercise::get_exercise,
            routes::exercise::create_exercise,
            routes::exercise::update_exercise,
            routes::exercise::get_input,
            routes::exercise::filter_exercise,
            routes::exercise::like_exercise,
            routes::exercise::unlike_exercise,
            routes::solution::get_solution,
            routes::solution::new_solution,
            routes::score::get_num_scores,
        ],
    )
}
