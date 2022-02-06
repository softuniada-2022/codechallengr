use crate::db::exercise_manipulation;
use crate::db::user_manipulation;
use crate::models::likes::Like;
use crate::models::users::{RegistrationUser, User};
use crate::utils;
use rocket::http::CookieJar;
use rocket::serde::json::Json;

#[get("/user/<username>")]
pub fn get_user(username: String) -> Json<User> {
    user_manipulation::get_user(&username).unwrap().into()
}

#[post("/user", format = "application/json", data = "<credentials>")]
pub fn post_user(credentials: Json<RegistrationUser>) -> Json<bool> {
    user_manipulation::new_user(credentials.into_inner()).into()
}

#[put("/user/<username>", format = "application/json", data = "<user>")]
pub fn update_user(
    cookies: &CookieJar<'_>,
    username: String,
    user: Json<RegistrationUser>,
) -> Json<bool> {
    if utils::verify_permission::verify_sender_self(
        &username,
        cookies.get("username").unwrap().value().to_string(),
    ) {
        return user_manipulation::update_user(user.into_inner())
            .unwrap()
            .into();
    }
    false.into()
}

#[delete("/user/<username>")]
pub fn delete_user(cookies: &CookieJar<'_>, username: String) -> Json<bool> {
    if utils::verify_permission::verify_sender_self(
        &username,
        cookies.get("username").unwrap().value().to_string(),
    ) {
        return user_manipulation::delete_user(username).into();
    }
    false.into()
}

//#[get("/user/limit?<number>&<sort_type>")]
#[get("/user/limit/<number>")]
pub fn get_users(number: i32) -> Json<Vec<User>> {
    user_manipulation::get_num_users(number).unwrap().into()
}

#[get("/user/<username>/likes")]
pub fn get_likes(username: String) -> Json<Vec<Like>> {
    exercise_manipulation::get_user_likes(username)
        .unwrap()
        .into()
}
