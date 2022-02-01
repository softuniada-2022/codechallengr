use crate::{models::schema::solutions, routes::solution};
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use crate::db::solution_manipulation;

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
    pub u_id: String,
    pub s_answer: String,
}

impl From<CreateSolution> for NewSolution {
    fn from(a: CreateSolution) -> Self {
        let sol = a.s_answer;
        let correct = solution_manipulation::check_solution(a.ex_id, &sol);
        NewSolution {
            ex_id: a.ex_id,
            u_id: a.u_id,
            s_answer: sol,
            s_correct: correct,
            s_submitted_at: chrono::Utc::now().naive_utc(),
        }
    }
}
