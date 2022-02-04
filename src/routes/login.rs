use crate::db::user_manipulation;
use crate::models::users::LoginInformation;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;

#[post("/login", format = "application/json", data = "<credentials>")]
pub fn post_login(cookies: &CookieJar<'_>, credentials: Json<LoginInformation>) -> Json<bool> {
    let creds = credentials.into_inner();
    match user_manipulation::check_password(&creds) {
        true => {
            cookies.add_private(Cookie::new("username", creds.u_name));
            true.into()
        }
        false => false.into(),
    }
}

#[post("/logout")]
pub fn post_logout(cookies: &CookieJar<'_>) -> Json<bool> {
    cookies.remove_private(Cookie::named("username"));
    true.into()
}
