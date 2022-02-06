use crate::db::user_manipulation;
use crate::models::users::LoginInformation;
use rocket::http::{Cookie, CookieJar};
use rocket::serde::json::Json;

#[post("/login", format = "application/json", data = "<credentials>")]
pub fn post_login(cookies: &CookieJar<'_>, credentials: Json<LoginInformation>) -> Json<bool> {
    let creds = credentials.into_inner();
    match user_manipulation::check_password(&creds) {
        true => {
            cookies.add(Cookie::new(
                "password",
                bcrypt::hash(creds.u_name.clone(), bcrypt::DEFAULT_COST).unwrap(),
            ));
            cookies.add(Cookie::new(
                "perm",
                bcrypt::hash(
                    user_manipulation::get_perm(&creds.u_name)
                        .unwrap()
                        .to_string(),
                    bcrypt::DEFAULT_COST,
                )
                .unwrap(),
            ));
            cookies.add(Cookie::new("username", creds.u_name.clone()));
            true.into()
        }
        false => false.into(),
    }
}

#[delete("/login")]
pub fn post_logout(cookies: &CookieJar<'_>) -> Json<bool> {
    cookies.remove(Cookie::named("username"));
    cookies.remove(Cookie::named("password"));
    cookies.remove(Cookie::named("perm"));
    true.into()
}
