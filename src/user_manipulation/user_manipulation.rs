use crate::models::models::{NewUser, User};
use crate::models::schema::users;
use crate::utils::establish_connection::establish_connection;
use diesel::prelude::*;

// pub fn new_user(name: String, email: String, password: String) {
//     let conn = establish_connection();
//     let hashed_password =
//         bcrypt::hash(&password, bcrypt::DEFAULT_COST).expect("Something happened while hashing");
//     let new_user = NewUser {
//         u_name: name,
//         u_email: email,
//         u_password: hashed_password,
//         u_created_at: chrono::Utc::now().naive_utc(),
//         u_updated_at: chrono::Utc::now().naive_utc(),
//     };
//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .execute(&conn)
//         .expect("Error saving new user");
// }

pub fn new_user(user: NewUser) -> Option<()>{
    let conn = establish_connection();
    let hashed_password =
        bcrypt::hash(&user.u_password, bcrypt::DEFAULT_COST).expect("Something happened while hashing");
    let usr = NewUser {
        u_name: user.u_name,
        u_email: user.u_email,
        u_password: hashed_password,
        u_created_at: Some(chrono::Utc::now().naive_utc()),
        u_updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    let affected = diesel::insert_into(users::table)
        .values(&usr)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => Some(()),
        Some(_) => panic!("something in the database fucked up very seriously"),
        _ => None
    }
}

pub fn get_user(username: String) -> User {
    let conn = establish_connection();
    let user = users::table
        .filter(users::u_name.eq(username))
        .first(&conn)
        .expect("Error loading user");
    user
}

pub fn get_all_users() -> Vec<User> {
    let conn = establish_connection();
    let usrs = users::table
        .load::<User>(&conn)
        .expect("Error loading users");
    usrs
}

pub fn check_password(username: String, password: String) -> bool {
    let conn = establish_connection();
    let user = users::table
        .filter(users::u_name.eq(username))
        .first::<User>(&conn)
        .expect("Error loading user");
    let is_correct = bcrypt::verify(&password, &user.u_password).unwrap();
    is_correct
}

pub fn updated_user(username: String) {
    let conn = establish_connection();
    let user: User = users::table
        .filter(users::u_name.eq(&username))
        .first(&conn)
        .expect("Error loading user");
    let updated_user = NewUser {
        u_name: user.u_name,
        u_email: user.u_email,
        u_password: user.u_password,
        u_created_at: Some(user.u_created_at),
        u_updated_at: Some(chrono::Utc::now().naive_utc()),
    };
    diesel::update(users::table.filter(users::u_name.eq(&username)))
        .set(&updated_user)
        .execute(&conn)
        .expect("Error updating user");
}
