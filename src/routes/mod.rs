#[get("/")]
pub fn hello() -> String {
    "Hello, world!".to_string()
}

pub mod exercise;
pub mod login;
pub mod score;
pub mod solution;
pub mod user;
