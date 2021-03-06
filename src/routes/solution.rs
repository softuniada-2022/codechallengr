use crate::db::exercise_manipulation;
use crate::db::score_manipulation;
use crate::db::solution_manipulation;
use crate::models::permissions::Permission;
use crate::models::solution::NewSolution;
use crate::models::solution::{CreateSolution, Solution, SolutionResult};
use crate::models::users::Claim;
use crate::utils::verify_permission;
use jwt::{decode, DecodingKey, TokenData, Validation};
use rocket::http::{CookieJar, Status};
use rocket::response::status::Custom;
use rocket::response::status::Unauthorized;
use rocket::serde::json::Json;
use std::env;

#[get("/solution/<id>")]
pub fn get_solution(cookies: &CookieJar<'_>, id: i32) -> Json<Option<Solution>> {
    let sln = solution_manipulation::get_solution(id).unwrap();
    let guest_claim = TokenData {
        header: Default::default(),
        claims: Claim {
            username: "".to_string(),
            exp: 0,
            perm: Permission::Guest,
        },
    };
    let claim = match cookies.get("token") {
        Some(cookie) => decode::<Claim>(cookie.value(), &DecodingKey::from_secret(
            env::var("JWT_KEY").unwrap().as_bytes(),
        ), &Validation::default()).unwrap_or(guest_claim),
        None => guest_claim,
    }.claims;
    if sln.u_id == claim.username {
        return Some(sln).into();
    }
    None.into()
}

#[get("/solution?<limit>&<exercise>")]
pub fn get_solutions(
    cookies: &CookieJar<'_>,
    limit: Option<i32>,
    exercise: i32,
) -> Result<Json<Vec<Solution>>, Unauthorized<String>> {
    let guest_claim = TokenData {
        header: Default::default(),
        claims: Claim {
            username: "".to_string(),
            exp: 0,
            perm: Permission::Guest,
        },
    };
    let claim = match cookies.get("token") {
        Some(cookie) => decode::<Claim>(cookie.value(), &DecodingKey::from_secret(
            env::var("JWT_KEY").unwrap().as_bytes(),
        ), &Validation::default()).unwrap_or(guest_claim),
        None => guest_claim,
    }.claims;
    if claim.username == *"" {
        return Err(Unauthorized(Some("You are not logged in.".to_string())));
    }
    let slns = solution_manipulation::get_some_solutions_for_user(
        limit.unwrap_or(0),
        &exercise,
        &claim.username,
    );
    Ok(slns.into())
}

#[post("/solution", format = "application/json", data = "<solution>")]
pub fn new_solution(
    cookies: &CookieJar<'_>,
    solution: Json<CreateSolution>,
) -> Result<Json<SolutionResult>, Custom<String>> {
    let sln = solution.into_inner();
    let guest_claim = TokenData {
        header: Default::default(),
        claims: Claim {
            username: "".to_string(),
            exp: 0,
            perm: Permission::Guest,
        },
    };
    let claim = match cookies.get("token") {
        Some(cookie) => decode::<Claim>(cookie.value(), &DecodingKey::from_secret(
            env::var("JWT_KEY").unwrap().as_bytes(),
        ), &Validation::default()).unwrap_or(guest_claim),
        None => guest_claim,
    }.claims;
    if claim.perm == Permission::Guest {
        return Err(Custom(Status::Unauthorized, "Unauthorized".to_string()));
    }
    let correct = solution_manipulation::check_solution(&sln.ex_id, &sln.s_answer);
    let difficulty = exercise_manipulation::get_difficulty(sln.ex_id).unwrap();

    let prev_scored_up = verify_permission::check_prev_solutions(&claim, sln.ex_id);

    if !prev_scored_up {
        let _scored_up = score_manipulation::increment_score(claim.username.clone(), difficulty);
    }

    let happened = solution_manipulation::new_solution(NewSolution {
        ex_id: sln.ex_id,
        u_id: claim.username,
        s_answer: sln.s_answer,
        s_correct: correct,
        s_submitted_at: chrono::Utc::now().naive_utc(),
    });
    if !happened {
        return Err(Custom(
            Status::InternalServerError,
            "Something went wrong.".to_string(),
        ));
    }
    Ok(SolutionResult {
        prev_scored_up,
        s_correct: correct,
    }
    .into())
}
