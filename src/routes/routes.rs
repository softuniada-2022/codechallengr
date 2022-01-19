use crate::models::models::{LoginInformation, User, RegistrationUser};
use crate::rocket::serde::json::Json;
use crate::user_manipulation::user_manipulation::{check_password, get_user, new_user};
// use rocket;

#[get("/")]
pub fn hello() -> String {
    "Hello, world!".to_string()
}

#[get("/api/user/<username>")]
pub fn view_user(username: String) -> Json<User> {
    get_user(username).into()
}

#[post("/api/login", format = "application/json", data = "<credentials>")]
pub fn login(credentials: Json<LoginInformation>) -> String {
    match check_password(credentials.into_inner()) {
        true => "Success".to_string(),
        false => "Failure".to_string(),
    }
}

#[post("/api/user", format = "application/json", data = "<credentials>")]
pub fn signup(credentials: Json<RegistrationUser>) -> String {
    match new_user(credentials.into_inner()) {
        Some(()) => "Success".to_string(),
        _ => "Failure".to_string(),
    }
}
