use crate::db::user_manipulation;
use crate::models::users::{RegistrationUser, UpdateUser, User};
use rocket::serde::json::Json;

#[get("/user/<username>")]
pub fn get_user(username: String) -> Json<User> {
    user_manipulation::get_user(username).unwrap().into()
}

#[post("/user", format = "application/json", data = "<credentials>")]
pub fn post_user(credentials: Json<RegistrationUser>) -> Json<bool> {
    user_manipulation::new_user(credentials.into_inner()).into()
}

#[put("/user/<username>", format = "application/json", data = "<user>")]
pub fn update_user(username: String, user: Json<UpdateUser>) -> Json<User> {
    user_manipulation::update_user(user.into_inner())
        .unwrap()
        .into()
}

#[delete("/user/<username>")]
pub fn delete_user(username: String) -> Json<bool> {
    user_manipulation::delete_user(username).into()
}