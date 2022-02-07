use crate::db::exercise_manipulation;
use crate::db::user_manipulation;
use crate::models::likes::Like;
use crate::models::users::Claim;
use crate::models::users::UpdateUser;
use crate::models::users::{RegistrationUser, User};
use crate::utils;
use jwt::{decode, DecodingKey, Validation};
use rocket::http::{CookieJar, Status};
use rocket::response::status::{Accepted, Custom, NotFound, Unauthorized};
use rocket::serde::json::Json;
use std::env;

#[get("/user/<username>")]
pub fn get_user(username: String) -> Result<Json<User>, NotFound<String>> {
    println!("{}", username);
    println!("{:?}", user_manipulation::get_perm(&username).unwrap());
    match user_manipulation::get_user(&username) {
        Some(user) => Ok(user.into()),
        None => Err(NotFound(format!("User {} not found", username))),
    }
}

#[post("/user", format = "application/json", data = "<credentials>")]
pub fn post_user(credentials: Json<RegistrationUser>) -> Result<Json<User>, Custom<String>> {
    match user_manipulation::new_user(credentials.into_inner()) {
        Ok(user) => Ok(user.into()),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            "Username already exists".to_string(),
        )),
    }
}

#[put("/user/<username>", format = "application/json", data = "<user>")]
pub fn update_user(
    cookies: &CookieJar<'_>,
    username: String,
    user: Json<UpdateUser>,
) -> Result<Json<User>, Unauthorized<String>> {
    let claim = decode::<Claim>(
        &cookies.get("token").unwrap().value().to_string(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims;
    if utils::verify_permission::verify_sender_self(&claim, &username) {
        let user = user.into_inner();
        let upd_user = RegistrationUser {
            u_name: username.clone(),
            u_email: user.u_email,
            u_password: user.u_password,
        };
        user_manipulation::update_user(upd_user);
        return Ok(user_manipulation::get_user(&username).unwrap().into());
    }
    Err(Unauthorized(Some(
        "You do not have permission to update this user".to_string(),
    )))
}

#[delete("/user/<username>")]
pub fn delete_user(
    cookies: &CookieJar<'_>,
    username: String,
) -> Result<Accepted<String>, Unauthorized<String>> {
    let claim = decode::<Claim>(
        &cookies.get("token").unwrap().value().to_string(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims;
    if utils::verify_permission::verify_sender_self(&claim, &username) {
        user_manipulation::delete_user(username);
        return Ok(Accepted(Some("User deleted".to_string())));
    }
    Err(Unauthorized(Some(
        "You do not have permission to delete this user".to_string(),
    )))
}

#[get("/user?<limit>&<sort_type>&<order>")]
// #[get("/user/limit/<number>")]
pub fn get_users(limit: Option<i32>, sort_type: Option<String>, order: Option<String>) -> Json<Vec<User>> {
    user_manipulation::get_num_users(limit.unwrap_or(50), &sort_type.unwrap_or("score".to_string()), &order.unwrap_or("desc".to_string())).into()
}

#[get("/user/<username>/likes")]
pub fn get_likes(username: String) -> Json<Vec<Like>> {
    exercise_manipulation::get_user_likes(username)
        .unwrap()
        .into()
}
