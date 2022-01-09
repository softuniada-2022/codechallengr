use crate::models::models::{User, NewUser};
use crate::models::schema::{users, users::dsl::*};
use crate::utils::establish_connection::establish_connection;
use diesel::prelude::*;

pub fn new_user(name: String, email: String, password: String) {
    let conn = establish_connection();
    let new_user = NewUser {
        u_name: name,
        u_email: email,
        u_password: password,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&conn)
        .expect("Error saving new user");
}

// pub fn get_user(id: i32) -> User {
//     let conn = establish_connection();
//     let user = users::table.filter(users::u_id.eq(id)).first(&conn).expect("Error loading user");
//     user
// }
