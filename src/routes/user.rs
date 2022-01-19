use rocket::serde::json::Json;
use crate::models::models::{User, RegistrationUser};
use crate::user_manipulation::user_manipulation;

#[get("/user/<username>")]
pub fn get_user(username: String) -> Json<User> {
    user_manipulation::get_user(username).into()
}

#[post("/user", format = "application/json", data = "<credentials>")]
pub fn post_user(credentials: Json<RegistrationUser>) -> String {
    match user_manipulation::new_user(credentials.into_inner()) {
        Some(()) => "Success".to_string(),
        _ => "Failure".to_string(),
    }
}