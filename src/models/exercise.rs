use crate::models::schema::exercises;
use chrono;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Exercise {
    pub ex_id: u64,
    pub ex_name: String,
    pub u_id: String,
    pub ex_description: String,
    pub ex_answer: String,
    pub ex_created_at: NaiveDateTime,
    pub ex_updated_at: NaiveDateTime,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "exercises"]
pub struct NewExercise {
    pub ex_name: String,
    pub u_id: String,
    pub ex_description: String,
    pub ex_answer: String,
    pub ex_created_at: NaiveDateTime,
    pub ex_updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateExercise {
    pub ex_name: String,
    pub u_id: String,
    pub ex_description: String,
    pub ex_answer: String,
    pub ex_created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct CreateExercise {
    pub ex_name: String,
    pub u_id: String,
    pub ex_description: String,
    pub ex_answer: String,
}

impl From<NewExercise> for Exercise {
    fn from(a: NewExercise) -> Self {
        Exercise {
            ex_id: 0,
            ex_name: a.ex_name,
            u_id: a.u_id,
            ex_description: a.ex_description,
            ex_answer: a.ex_answer,
            ex_created_at: a.ex_created_at,
            ex_updated_at: a.ex_updated_at,
        }
    }
}

impl From<CreateExercise> for NewExercise {
    fn from(a: CreateExercise) -> Self {
        NewExercise {
            ex_name: a.ex_name,
            u_id: a.u_id,
            ex_description: a.ex_description,
            ex_answer: a.ex_answer,
            ex_created_at: Some(chrono::Utc::now().naive_utc()).unwrap(),
            ex_updated_at: Some(chrono::Utc::now().naive_utc()).unwrap(),
        }
    }
}

impl From<UpdateExercise> for NewExercise {
    fn from(a: UpdateExercise) -> Self {
        NewExercise {
            ex_name: a.ex_name,
            u_id: a.u_id,
            ex_description: a.ex_description,
            ex_answer: a.ex_answer,
            ex_created_at: a.ex_created_at,
            ex_updated_at: Some(chrono::Utc::now().naive_utc()).unwrap(),
        }
    }
}
