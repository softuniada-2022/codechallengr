use crate::db::user_manipulation;
use crate::models::users::LoginInformation;
use jwt::{encode, Algorithm, EncodingKey, Header};
use rocket::http::{Cookie, CookieJar};
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;

use dotenv::dotenv;
use std::env;

#[post("/login", format = "application/json", data = "<credentials>")]
pub fn post_login(
    cookies: &CookieJar<'_>,
    credentials: Json<LoginInformation>,
) -> Result<String, Unauthorized<String>> {
    let creds = credentials.into_inner();
    match user_manipulation::try_login(&creds) {
        Some(claim) => {
            dotenv().ok();
            let token = encode(
                &Header::new(Algorithm::HS256),
                &claim,
                &EncodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
            )
            .ok()
            .unwrap();
            cookies.add(Cookie::new("token", token.clone()));
            Ok(token)
        }
        None => Err(Unauthorized(Some(
            "Invalid username or password".to_string(),
        ))),
    }
}
// send HTTP code instead of true false on login

#[delete("/login")]
pub fn post_logout(cookies: &CookieJar<'_>) -> Json<bool> {
    cookies.remove(Cookie::named("token"));
    true.into()
}
