use crate::bcrypt;
use crate::diesel::{Insertable, Queryable};
use crate::models::permissions::Permission;
use crate::models::schema::users;
use crate::serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct User {
    pub u_name: String,
    pub u_email: String,
    #[serde(skip_serializing)]
    pub u_password: String,
    pub u_score: i32,
    pub u_permission: Permission,
    pub u_created_at: NaiveDateTime,
    pub u_updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct NewUser {
    pub u_name: String,
    pub u_email: String,
    pub u_password: String,
    pub u_permission: Permission,
    pub u_created_at: NaiveDateTime,
    pub u_updated_at: NaiveDateTime,
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
    pub u_created_at: NaiveDateTime,
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
            u_password: bcrypt::hash(a.u_password.to_string(), bcrypt::DEFAULT_COST)
                .expect("Something happened while hashing"),
            u_permission: Permission::User,
            u_created_at: a.u_created_at,
            u_updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<RegistrationUser> for NewUser {
    fn from(a: RegistrationUser) -> Self {
        NewUser {
            u_name: a.u_name,
            u_email: a.u_email,
            u_password: bcrypt::hash(a.u_password.to_string(), bcrypt::DEFAULT_COST)
                .expect("Something happened while hashing"),
            u_permission: Permission::User,
            u_created_at: chrono::Utc::now().naive_utc(),
            u_updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<NewUser> for User {
    fn from(a: NewUser) -> Self {
        User {
            u_name: a.u_name,
            u_email: a.u_email,
            u_password: a.u_password,
            u_score: 0,
            u_permission: a.u_permission,
            u_created_at: a.u_created_at,
            u_updated_at: a.u_updated_at,
        }
    }
}
