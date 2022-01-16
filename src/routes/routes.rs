use crate::models::models::{NewUser, User};
use crate::rocket::serde::json::Json;
use crate::user_manipulation::user_manipulation::{get_user, new_user};
use rocket;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, world!"
}

#[post("/signup", format = "application/json", data = "<user>")]
pub fn register(user: Json<NewUser>) -> &'static str {
    // match new_user(user.into_inner()) {
    //     true => "User created",
    //     false => "User already exists",
    // }
    new_user(user.into_inner());
    "User created"
    // TODO: implement a true/false return for the user creation
}

#[get("/user/<username>")]
pub fn view_user(username: String) -> rocket::serde::json::Json<User> {
    get_user(username).into()
}
