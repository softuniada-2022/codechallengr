pub mod db;
pub mod models;
pub mod routes;
pub mod utils;

#[macro_use]
extern crate diesel;
extern crate bcrypt;

#[macro_use]
// #[feature(decl_macro)]
extern crate rocket;
// #[macro_use]
// extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
