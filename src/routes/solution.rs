use crate::db::exercise_manipulation;
use crate::db::score_manipulation;
use crate::db::solution_manipulation;
use crate::models::solution::NewSolution;
use crate::models::solution::{CreateSolution, Solution, SolutionResult};
use rocket::http::CookieJar;
use rocket::serde::json::Json;

#[get("/solution/<id>")]
pub fn get_solution(cookies: &CookieJar<'_>, id: i32) -> Json<Option<Solution>> {
    let sln = solution_manipulation::get_solution(id).unwrap();
    let u_id = cookies.get("username").unwrap().value().to_string();
    if sln.u_id == u_id {
        return Some(sln).into();
    }
    None.into()
}

#[post("/solution", format = "application/json", data = "<solution>")]
pub fn new_solution(
    cookies: &CookieJar<'_>,
    solution: Json<CreateSolution>,
) -> Json<SolutionResult> {
    let sln = solution.into_inner();
    let u_id = cookies.get("username").unwrap().value().to_string();
    let correct = solution_manipulation::check_solution(&sln.ex_id, &sln.s_answer);
    let difficulty = exercise_manipulation::get_difficulty(sln.ex_id).unwrap();

    let prev_slns = solution_manipulation::get_all_solutions_for_user(&sln.ex_id, &u_id);
    let mut prev_scored_up = false;
    for prev_sln in prev_slns {
        if prev_sln.s_correct == true {
            prev_scored_up = true;
            break;
        }
    }

    if prev_scored_up == false {
        let _scored_up = score_manipulation::increment_score(u_id.clone(), difficulty);
    }

    let happened = solution_manipulation::new_solution(NewSolution {
        ex_id: sln.ex_id,
        u_id: u_id,
        s_answer: sln.s_answer,
        s_correct: correct,
        s_submitted_at: chrono::Utc::now().naive_utc(),
    });
    SolutionResult {
        happened: happened,
        s_correct: correct,
        prev_scored_up: prev_scored_up,
    }
    .into()
}
