use crate::models::schema::solutions;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum SolutionError {
    NotAuthorized,
    NotFound,
}

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Solution {
    pub s_id: u64,
    pub ex_id: i32,
    pub u_id: String,
    pub s_answer: String,
    pub s_correct: bool,
    pub s_submitted_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "solutions"]
pub struct NewSolution {
    pub ex_id: i32,
    pub u_id: String,
    pub s_answer: String,
    pub s_correct: bool,
    pub s_submitted_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateSolution {
    pub ex_id: i32,
    pub s_answer: String,
}

#[derive(Deserialize, Serialize)]
pub struct SolutionResult {
    pub happened: bool,
    pub s_correct: bool,
    pub prev_scored_up: bool,
}
