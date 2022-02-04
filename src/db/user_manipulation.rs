use crate::models::schema::users;
use crate::models::users::{LoginInformation, NewUser, RegistrationUser, UpdateUser, User};
use crate::utils::establish_connection::establish_connection;
use diesel::prelude::*;

pub fn new_user(user: RegistrationUser) -> bool {
    let conn = establish_connection();
    let usr = NewUser::from(user);
    let affected = diesel::insert_into(users::table)
        .values(&usr)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => true,
        Some(_) => false,
        _ => false,
    }
}

pub fn get_user(username: String) -> Option<User> {
    let conn = establish_connection();
    users::table
        .filter(users::u_name.eq(username))
        .first(&conn)
        .ok()
}

pub fn get_all_users() -> Option<Vec<User>> {
    let conn = establish_connection();
    users::table.load::<User>(&conn).ok()
}

pub fn check_password(info: &LoginInformation) -> bool {
    let conn = establish_connection();
    let user = users::table
        .filter(users::u_name.eq(&info.u_name))
        .first::<User>(&conn)
        .expect("Error loading user");
    bcrypt::verify(&info.u_password, &user.u_password).unwrap()
}

pub fn update_user(user: UpdateUser) -> Option<User> {
    let conn = establish_connection();
    let updated_user = NewUser::from(user);
    let affected = diesel::update(users::table.filter(users::u_name.eq(&updated_user.u_name)))
        .set(&updated_user)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => Some(User::from(updated_user)),
        Some(0) => None,
        _ => None,
    }
}

pub fn delete_user(user: String) -> bool {
    let conn = establish_connection();
    let affected = diesel::delete(users::table.filter(users::u_name.eq(user)))
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => true,
        Some(0) => false,
        _ => false,
    }
}
