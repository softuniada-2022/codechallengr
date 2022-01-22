use crate::models::schema::users;
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
pub struct UpdateUser {
    pub u_name: String,
    pub u_email: String,
    pub u_password: String,
    pub u_created_at: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInformation {
    pub u_name: String,
    pub u_password: String,
}

impl From<UpdateUser> for NewUser {
    fn from(a: UpdateUser) -> Self {
        NewUser {
            u_name: a.u_name,
            u_email: a.u_email,
            u_password: a.u_password,
            u_created_at: Some(chrono::Utc::now().naive_utc()),
            u_updated_at: Some(chrono::Utc::now().naive_utc()),
        }
    }
}

impl From<NewUser> for User {
    fn from(a: NewUser) -> Self {
        User {
            u_name: a.u_name,
            u_email: a.u_email,
            u_password: a.u_password,
            u_created_at: a.u_created_at.unwrap(),
            u_updated_at: a.u_updated_at.unwrap(),
        }
    }
}