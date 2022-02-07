pub mod db;
pub mod models;
pub mod routes;
pub mod utils;

#[macro_use]
extern crate diesel;
extern crate bcrypt;
#[macro_use]
extern crate rocket;
extern crate diesel_derive_enum;
extern crate jsonwebtoken as jwt;
extern crate serde;
extern crate serde_json;

#[cfg(test)]
mod tests;
