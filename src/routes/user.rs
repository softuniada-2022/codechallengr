use crate::db::user_manipulation;
use crate::models::users::{RegistrationUser, User};
use rocket::serde::json::Json;

#[get("/user/<username>")]
pub fn get_user(username: String) -> Json<User> {
    user_manipulation::get_user(username).unwrap().into()
}

#[post("/user", format = "application/json", data = "<credentials>")]
pub fn post_user(credentials: Json<RegistrationUser>) -> String {
    match user_manipulation::new_user(credentials.into_inner()) {
        true => "Success".to_string(),
        false => "Failure".to_string(),
    }
}
