use crate::models::models::LoginInformation;
use crate::user_manipulation::user_manipulation;
use rocket::serde::json::Json;

#[post("/api/login", format = "application/json", data = "<credentials>")]
pub fn post_login(credentials: Json<LoginInformation>) -> String {
    match user_manipulation::check_password(credentials.into_inner()) {
        true => "Success".to_string(),
        false => "Failure".to_string(),
    }
}
