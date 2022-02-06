use crate::models::schema::likes;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Like {
    pub u_id: String,
    pub ex_id: i32,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "likes"]
pub struct CreateLike {
    pub u_id: String,
    pub ex_id: i32,
}
