use crate::models::schema::solutions;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Solution {
    pub s_id: i32,
    pub ex_id: i32,
    pub u_id: i32,
    pub s_answer: String,
    pub s_correct: bool,
    pub s_submitted_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "solutions"]
pub struct NewSolution {
    pub ex_id: i32,
    pub u_id: i32,
    pub s_answer: String,
    pub s_correct: bool,
    pub s_submitted_at: NaiveDateTime,
}
