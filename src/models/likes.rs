use crate::models::schema::likes;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize)]
pub struct Like {
    pub u_id: String,
    pub ex_id: i32,
}
