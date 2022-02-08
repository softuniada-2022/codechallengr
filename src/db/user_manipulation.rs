use crate::models::permissions::Permission;
use crate::models::schema::users;
use crate::models::users::{Claim, LoginInformation, NewUser, RegistrationUser, User, UserError};
use crate::utils::establish_connection::establish_connection;
use chrono;
use diesel::prelude::*;

pub fn new_user(user: RegistrationUser) -> Result<User, UserError> {
    let conn = establish_connection();
    let usr = NewUser::from(user);
    let affected = diesel::insert_into(users::table)
        .values(&usr)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => Ok(User::from(usr)),
        Some(_) => Err(UserError::UserAlreadyExists),
        _ => Err(UserError::InternalError),
    }
}

pub fn get_user(username: &str) -> Option<User> {
    let conn = establish_connection();
    let a: Option<User> = users::table
        .filter(users::u_name.eq(&username))
        .first(&conn)
        .ok();
    match a {
        Some(a) => Some(User {
            u_name: a.u_name,
            u_email: a.u_email,
            u_password: a.u_password,
            u_score: a.u_score,
            u_permission: get_perm(username).unwrap(),
            u_created_at: a.u_created_at,
            u_updated_at: a.u_updated_at,
        }),
        None => None,
    }
}

pub fn get_perm(username: &str) -> Option<Permission> {
    let conn = establish_connection();
    users::table
        .filter(users::u_name.eq(username))
        .select(users::u_permission)
        .first(&conn)
        .ok()
}

pub fn check_password(info: &LoginInformation) -> bool {
    let conn = establish_connection();
    let user = users::table
        .filter(users::u_name.eq(&info.u_name))
        .first::<User>(&conn)
        .expect("Error loading user");
    bcrypt::verify(&info.u_password, &user.u_password).unwrap()
}

pub fn try_login(info: &LoginInformation) -> Option<Claim> {
    let conn = establish_connection();
    let user = users::table
        .filter(users::u_name.eq(&info.u_name))
        .first::<User>(&conn)
        .expect("Error loading user");
    if bcrypt::verify(&info.u_password, &user.u_password).unwrap() {
        let claims = Claim {
            exp: (chrono::Utc::now().timestamp() + 30 * 24 * 60 * 60) as usize,
            username: user.u_name.clone(),
            perm: get_perm(&user.u_name).unwrap(),
        };
        Some(claims)
    } else {
        None
    }
}

pub fn update_user(user: RegistrationUser) -> Option<bool> {
    let conn = establish_connection();
    let updated_user = NewUser::from(user);
    let affected = diesel::update(users::table.filter(users::u_name.eq(&updated_user.u_name)))
        .set(&updated_user)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => Some(true),
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

pub fn get_num_users(limit: i32, sort_by: &str, order: &str) -> Vec<User> {
    let conn = establish_connection();
    let mut query = users::table.into_boxed();
    if sort_by == "name" {
        query = match order {
            "desc" => query.order(users::u_name.desc()),
            "asc" => query.order(users::u_name.asc()),
            _ => query.order(users::u_name.asc()),
        }
    } else if sort_by == "age" {
        query = match order {
            "desc" => query.order(users::u_created_at.desc()),
            "asc" => query.order(users::u_created_at.asc()),
            _ => query.order(users::u_created_at.asc()),
        }
    } else if sort_by == "score" {
        query = match order {
            "desc" => query.order(users::u_score.desc()),
            "asc" => query.order(users::u_score.asc()),
            _ => query.order(users::u_score.asc()),
        }
    } else {
        query = query.order(users::u_score.asc());
    }
    query.limit(limit as i64).load::<User>(&conn).unwrap()
}
