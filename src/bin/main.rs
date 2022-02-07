extern crate codechallenger;

use codechallenger::routes;
use dotenv::dotenv;
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
            routes::login::get_login,
            routes::login::post_login,
            routes::login::delete_login,
            routes::exercise::get_exercise,
            routes::exercise::create_exercise,
            routes::exercise::update_exercise,
            routes::exercise::get_input,
            routes::exercise::get_exercises,
            routes::exercise::like_exercise,
            routes::exercise::unlike_exercise,
            routes::solution::get_solution,
            routes::solution::new_solution,
            routes::score::get_scores,
        ],
    )
}
