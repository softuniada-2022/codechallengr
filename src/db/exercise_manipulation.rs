use crate::models::exercise::{Exercise, ExerciseError, NewExercise};
use crate::models::likes::Like;
use crate::models::schema::{exercises, likes};
use crate::utils::establish_connection::establish_connection;
use cain::cain;
use diesel::prelude::*;

pub fn new_exercise(exercise: &NewExercise) -> bool {
    let conn = establish_connection();
    let affected = diesel::insert_into(exercises::table)
        .values(exercise)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => true,
        Some(0) => false,
        _ => false,
    }
}

pub fn get_input(id: i32) -> Option<String> {
    let conn = establish_connection();
    exercises::table
        .filter(exercises::ex_id.eq(id as u64))
        .select(exercises::ex_input)
        .first::<String>(&conn)
        .ok()
}

pub fn get_difficulty(id: i32) -> Option<i32> {
    let conn = establish_connection();
    exercises::table
        .filter(exercises::ex_id.eq(id as u64))
        .select(exercises::ex_difficulty)
        .first::<i32>(&conn)
        .ok()
}

pub fn get_exercise(id: i32) -> Option<Exercise> {
    let conn = establish_connection();
    exercises::table
        .filter(exercises::ex_id.eq(id as u64))
        .first(&conn)
        .ok()
}

pub fn update_exercise(id: i32, exercise: NewExercise) -> bool {
    let conn = establish_connection();
    let affected = diesel::update(exercises::table)
        .filter(exercises::ex_id.eq(id as u64))
        .set(exercise)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => true,
        Some(0) => false,
        _ => false,
    }
}

pub fn like_exercise(like: Like) -> bool {
    let conn = establish_connection();
    let affected = diesel::insert_into(likes::table)
        .values(&like)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => true,
        Some(0) => false,
        _ => false,
    }
}

pub fn unlike_exercise(like: Like) -> bool {
    let conn = establish_connection();
    let affected = diesel::delete(likes::table)
        .filter(likes::u_id.eq(like.u_id))
        .filter(likes::ex_id.eq(like.ex_id))
        .execute(&conn)
        .ok();
    let _a = dec_exercise_likes(like.ex_id);
    match affected {
        Some(1) => true,
        Some(0) => false,
        _ => false,
    }
}

pub fn inc_exercise_likes(id: i32) -> Result<bool, ExerciseError> {
    let conn = establish_connection();
    let affected = diesel::update(exercises::table)
        .filter(exercises::ex_id.eq(id as u64))
        .set(exercises::ex_likes.eq(exercises::ex_likes + 1))
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => Result::Ok(true),
        Some(0) => Result::Err(ExerciseError::ExistsAlready),
        _ => Result::Err(ExerciseError::InternalError),
    }
}

pub fn dec_exercise_likes(id: i32) -> Result<bool, ExerciseError> {
    let conn = establish_connection();
    let affected = diesel::update(exercises::table)
        .filter(exercises::ex_id.eq(id as u64))
        .set(exercises::ex_likes.eq(exercises::ex_likes - 1))
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => Result::Ok(true),
        Some(0) => Result::Err(ExerciseError::ExistsAlready),
        _ => Result::Err(ExerciseError::InternalError),
    }
}

pub fn get_user_likes(username: String) -> Option<Vec<Like>> {
    let conn = establish_connection();
    likes::table
        .filter(likes::u_id.eq(username))
        .load::<Like>(&conn)
        .ok()
}

pub fn check_user_likes(username: String, exercise: i32) -> bool {
    let conn = establish_connection();
    likes::table
        .filter(likes::u_id.eq(username))
        .filter(likes::ex_id.eq(exercise))
        .first::<Like>(&conn)
        .is_ok()
}

pub fn get_like(like: Like) -> Option<Like> {
    let conn = establish_connection();
    likes::table
        .filter(likes::u_id.eq(like.u_id))
        .filter(likes::ex_id.eq(like.ex_id))
        .first(&conn)
        .ok()
}

pub fn get_exercise_likes(id: i32) -> i32 {
    let conn = establish_connection();
    exercises::table
        .filter(exercises::ex_id.eq(id as u64))
        .select(exercises::ex_likes)
        .first::<i32>(&conn)
        .unwrap_or(0)
}

pub fn filter_exercise(limit: i32, sort_by: &str, order: &str) -> Vec<Exercise> {
    let conn = establish_connection();
    let query = exercises::table.into_boxed();
    let query = cain! {
        let col = match sort_by {
            "likes" => exercises::ex_likes,
            "name" => exercises::ex_name,
            "difficulty" => exercises::ex_difficulty,
            _ => exercises::ex_id,
        };
        let expr = match order {
            "desc" => col.desc(),
            "asc" => col.asc(),
            _ => col.asc(),
        };
        query.order(expr)
    };
    if limit > 0 {
        query.limit(limit as i64).load::<Exercise>(&conn).unwrap()
    } else {
        query.load::<Exercise>(&conn).unwrap()
    }
}
