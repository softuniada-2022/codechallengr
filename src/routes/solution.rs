use crate::db::solution_manipulation;
use crate::models::solution::{CreateSolution, Solution};
use rocket::serde::json::Json;

#[get("/solution/<id>")]
pub fn get_solution(id: i32) -> Json<Solution> {
    solution_manipulation::get_solution(id).unwrap().into()
}

#[post("/solution", format = "application/json", data = "<solution>")]
pub fn new_solution(solution: Json<CreateSolution>) -> Json<bool> {
    solution_manipulation::new_solution(solution.into_inner()).into()
}
