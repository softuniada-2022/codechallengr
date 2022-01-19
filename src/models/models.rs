use crate::models::schema::{exercises, solutions, users};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub u_name: String,
    pub u_email: String,
    pub u_password: String,
    pub u_created_at: NaiveDateTime,
    pub u_updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub u_name: String,
    pub u_email: String,
    pub u_password: String,
    pub u_created_at: Option<NaiveDateTime>,
    pub u_updated_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct RegistrationUser {
    pub u_name: String,
    pub u_email: String,
    pub u_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInformation {
    pub u_name: String,
    pub u_password: String,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Solution {
    pub s_id: i32,
    pub ex_id: i32,
    pub u_id: i32,
    pub submitted_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "solutions"]
pub struct NewSolution {
    pub ex_id: i32,
    pub u_id: i32,
    pub submitted_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Exercise {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "exercises"]
pub struct NewExercise {
    pub ex_name: String,
    pub ex_description: String,
}
