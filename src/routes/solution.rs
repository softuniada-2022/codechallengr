use crate::db::score_manipulation;
use crate::db::solution_manipulation;
use crate::models::solution::NewSolution;
use crate::models::solution::{CreateSolution, Solution, SolutionResult};
use rocket::serde::json::Json;

#[get("/solution/<id>")]
pub fn get_solution(id: i32) -> Json<Solution> {
    solution_manipulation::get_solution(id).unwrap().into()
}

#[post("/solution", format = "application/json", data = "<solution>")]
pub fn new_solution(solution: Json<CreateSolution>) -> Json<SolutionResult> {
    let sln = solution.into_inner();

    let correct = solution_manipulation::check_solution(&sln.ex_id, &sln.s_answer);

    let prev_slns = solution_manipulation::get_all_solutions_for_user(&sln.ex_id, &sln.u_id);
    let mut prev_scored_up = false;
    for prev_sln in prev_slns {
        if prev_sln.s_correct == true {
            prev_scored_up = score_manipulation::increment_score(sln.u_id.clone());
        }
    }

    let happened = solution_manipulation::new_solution(NewSolution {
        ex_id: sln.ex_id,
        u_id: sln.u_id,
        s_answer: sln.s_answer,
        s_correct: correct,
        s_submitted_at: chrono::Utc::now().naive_utc(),
    });
    SolutionResult {
        happened: happened,
        s_correct: correct,
        scored_up: prev_scored_up,
    }
    .into()
}
