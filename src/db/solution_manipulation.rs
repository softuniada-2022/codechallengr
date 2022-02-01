use crate::models::schema::{solutions, exercises};
use crate::models::solution::{CreateSolution, NewSolution, Solution};
use crate::models::exercise::Exercise;
use crate::utils::establish_connection::establish_connection;
use diesel::prelude::*;

// GET EXERCISE'S SOLUTION && CHECK IF USER'S SOLUTION IS CORRECT && RETURN BOOL
pub fn check_solution(ex_id: i32, s_answer: &String) -> bool {
    let conn = establish_connection();
    let exercise_solution = exercises::table
        .filter(exercises::ex_id.eq(ex_id as u64))
        .first::<Exercise>(&conn)
        .ok().unwrap().ex_answer;
    if exercise_solution == *s_answer {
        true
    } else {
        false
    }
}

pub fn new_solution(solution: CreateSolution) -> bool {
    let conn = establish_connection();
    let solut = NewSolution::from(solution);
    let affected = diesel::insert_into(solutions::table)
        .values(&solut)
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => true,
        Some(_) => false,
        _ => false,
    }
}

pub fn get_solution(id: i32) -> Option<Solution> {
    let conn = establish_connection();
    solutions::table
        .filter(solutions::s_id.eq(id as u64))
        .first::<Solution>(&conn)
        .ok()
}
