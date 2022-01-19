#[get("/")]
pub fn hello() -> String {
    "Hello, world!".to_string()
}

pub mod login;
pub mod user;
