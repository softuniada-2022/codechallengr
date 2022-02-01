use crate::models::exercise::{CreateExercise, Exercise, NewExercise, UpdateExercise};
use crate::models::schema::exercises;
use crate::utils::establish_connection::establish_connection;
use diesel::prelude::*;

pub fn new_exercise(exercise: CreateExercise) -> bool {
    let conn = establish_connection();
    let new_exercise = NewExercise::from(exercise);
    let affected = diesel::insert_into(exercises::table)
        .values(&new_exercise)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => true,
        Some(0) => false,
        _ => false,
    }
}

pub fn get_exercise(id: i32) -> Option<Exercise> {
    let conn = establish_connection();
    exercises::table
        .filter(exercises::ex_id.eq(id as u64))
        .first(&conn)
        .ok()
}

pub fn update_exercise(id: i32, exercise: UpdateExercise) -> Option<Exercise> {
    let conn = establish_connection();
    let updated_exercise = NewExercise::from(exercise);
    let affected = diesel::update(exercises::table)
        .filter(exercises::ex_id.eq(id as u64))
        .set(&updated_exercise)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => Some(Exercise::from(updated_exercise)),
        Some(0) => None,
        _ => None,
    }
}
