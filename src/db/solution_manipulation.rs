use crate::models::exercise::Exercise;
use crate::models::schema::{exercises, solutions};
use crate::models::solution::{NewSolution, Solution};
use crate::utils::establish_connection::establish_connection;
use diesel::prelude::*;

// GET EXERCISE'S SOLUTION && CHECK IF USER'S SOLUTION IS CORRECT && RETURN BOOL
pub fn check_solution(ex_id: &i32, s_answer: &String) -> bool {
    let conn = establish_connection();
    exercises::table
        .filter(exercises::ex_id.eq(*ex_id as u64))
        .first::<Exercise>(&conn)
        .ok()
        .unwrap()
        .ex_answer
        .eq(s_answer)
}

pub fn new_solution(solution: NewSolution) -> bool {
    let conn = establish_connection();
    let affected = diesel::insert_into(solutions::table)
        .values(&solution)
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

pub fn get_all_solutions_for_user(ex_id: &i32, u_name: &String) -> Vec<Solution> {
    let conn = establish_connection();
    solutions::table
        .filter(solutions::u_id.eq(u_name))
        .filter(solutions::ex_id.eq(*ex_id))
        .load::<Solution>(&conn)
        .unwrap()
}
