use crate::db::user_manipulation;
use crate::models::users::Claim;
use crate::models::users::LoginInformation;
use jwt::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::http::{Cookie, CookieJar};
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;

use dotenv::dotenv;
use std::env;

#[get("/login")]
pub fn get_login(cookies: &CookieJar<'_>) -> Result<String, Unauthorized<String>> {
    match cookies.get("token") {
        Some(token) => Ok(token.value().to_string()),
        None => Err(Unauthorized(Some("You are not logged in".to_string()))),
    }
}

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

#[post("/login?renew")]
pub fn renew_login(cookies: &CookieJar<'_>) -> String {
    let claim = decode::<Claim>(
        cookies.get("token").unwrap().value(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims;
    let renewed_claim = Claim {
        exp: (chrono::Utc::now().timestamp() * 30 * 24 * 60 * 60) as usize,
        ..claim
    };
    let token = encode(
        &Header::new(Algorithm::HS256),
        &renewed_claim,
        &EncodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
    )
    .ok()
    .unwrap();
    cookies.add(Cookie::new("token", token.clone()));
    token
}

// send HTTP code instead of true false on login

#[delete("/login")]
pub fn delete_login(cookies: &CookieJar<'_>) -> Json<bool> {
    cookies.remove(Cookie::named("token"));
    true.into()
}
