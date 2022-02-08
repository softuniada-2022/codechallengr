use crate::db::exercise_manipulation;
use crate::models::exercise::{Exercise, UpdateExercise};
use crate::models::exercise::{LoggedInExercise, NewExercise};
use crate::models::likes::Like;
use crate::models::permissions::Permission;
use crate::models::users::Claim;
use crate::utils::verify_permission;
use dotenv::dotenv;
use jwt::TokenData;
use jwt::{decode, DecodingKey, Validation};
use rocket::http::CookieJar;
use rocket::response::status::{Accepted, Unauthorized};
use rocket::serde::json::Json;
use std::env;

#[get("/exercise/<id>")]
pub fn get_exercise(
    cookies: &CookieJar<'_>,
    id: i32,
) -> Result<Json<Exercise>, Json<LoggedInExercise>> {
    let claim = decode::<Claim>(
        cookies.get("token").unwrap().value(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap_or(TokenData {
        header: Default::default(),
        claims: Claim {
            username: "".to_string(),
            exp: 0,
            perm: Permission::Guest,
        },
    })
    .claims;
    if claim.perm == Permission::Guest {
        return Ok(exercise_manipulation::get_exercise(id).unwrap().into());
    }
    let ex = exercise_manipulation::get_exercise(id).unwrap();
    Err(LoggedInExercise {
        liked_by_me: verify_permission::verify_like_owner(&claim, id),
        solved_by_me: verify_permission::check_prev_solutions(&claim, id),
        ..ex.into()
    }
    .into())
}

#[get("/exercise?<limit>&<sort_by>&<order>")]
pub fn get_exercises(
    cookies: &CookieJar<'_>,
    limit: Option<i32>,
    sort_by: Option<String>,
    order: Option<String>,
) -> Result<Json<Vec<Exercise>>, Json<Vec<LoggedInExercise>>> {
    let claim = decode::<Claim>(
        cookies.get("token").unwrap().value(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap_or(TokenData {
        header: Default::default(),
        claims: Claim {
            username: "".to_string(),
            exp: 0,
            perm: Permission::Guest,
        },
    })
    .claims;
    let exs = exercise_manipulation::filter_exercise(
        limit.unwrap_or(0),
        &sort_by.unwrap_or("name".to_string()),
        &order.unwrap_or("asc".to_string()),
    );
    if claim.perm == Permission::Guest {
        return Ok(exs.into());
    }
    Err(exs
        .into_iter()
        .map(|e| e.into())
        .map(|e: LoggedInExercise| LoggedInExercise {
            liked_by_me: verify_permission::verify_like_owner(&claim, e.ex_id as i32),
            ..e
        })
        .collect::<Vec<_>>()
        .into())
}

#[post("/exercise", format = "application/json", data = "<exercise>")]
pub fn create_exercise(
    cookies: &CookieJar<'_>,
    exercise: Json<UpdateExercise>,
) -> Result<Json<Exercise>, Unauthorized<String>> {
    dotenv().ok();
    let ex = exercise.into_inner();
    let claim = decode::<Claim>(
        cookies.get("token").unwrap().value(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims;
    if verify_permission::verify_allowed_author(&claim) {
        let exercise = NewExercise {
            ex_name: ex.ex_name,
            u_id: claim.username,
            ex_description: ex.ex_description,
            ex_input: ex.ex_input,
            ex_answer: ex.ex_answer,
            ex_difficulty: ex.ex_difficulty,
            ex_likes: 0,
            ex_created_at: chrono::Utc::now().naive_utc(),
            ex_updated_at: chrono::Utc::now().naive_utc(),
        };
        Ok(exercise_manipulation::new_exercise(&exercise).unwrap().into())
    } else {
        Err(Unauthorized(Some(
            "You do not have permission to create challenges".to_string(),
        )))
    }
}

#[put("/exercise/<id>", format = "application/json", data = "<exercise>")]
pub fn update_exercise(
    cookies: &CookieJar<'_>,
    id: String,
    exercise: Json<UpdateExercise>,
) -> Result<Json<Exercise>, Unauthorized<String>> {
    let ex = exercise.into_inner();
    let claim = decode::<Claim>(
        cookies.get("token").unwrap().value(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims;
    if verify_permission::verify_author(&claim, id.parse::<i32>().unwrap()) {
        if verify_permission::verify_allowed_author(&claim) {
            let current = exercise_manipulation::get_exercise(id.parse::<i32>().unwrap()).unwrap();
            let exercise = NewExercise {
                ex_name: ex.ex_name,
                u_id: current.u_id,
                ex_description: ex.ex_description,
                ex_input: ex.ex_input,
                ex_answer: ex.ex_answer,
                ex_difficulty: ex.ex_difficulty,
                ex_likes: current.ex_likes,
                ex_created_at: current.ex_created_at,
                ex_updated_at: chrono::Utc::now().naive_utc(),
            };
            exercise_manipulation::update_exercise(id.parse::<i32>().unwrap(), exercise.clone());
            return Ok(Exercise::from(exercise).into());
        } else {
            return Err(Unauthorized(Some(
                "You are not the author of this challenge".to_string(),
            )));
        }
    }
    Err(Unauthorized(Some(
        "You do not have permission to create challenges".to_string(),
    )))
}

#[get("/exercise/<id>/input")]
pub fn get_input(id: String) -> Json<String> {
    exercise_manipulation::get_input(id.parse::<i32>().unwrap())
        .unwrap()
        .into()
}

#[post("/exercise/<id>/like")]
pub fn like_exercise(
    cookies: &CookieJar<'_>,
    id: i32,
) -> Result<Accepted<String>, Unauthorized<String>> {
    let claim = decode::<Claim>(
        cookies.get("token").unwrap().value(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims;
    if verify_permission::verify_like_owner(&claim, id) {
        return Err(Unauthorized(Some(
            "You have already liked this challenge".to_string(),
        )));
    }
    // if verify_permission::verify_like_owner(&claim, id) {
    exercise_manipulation::like_exercise(Like {
        u_id: claim.username,
        ex_id: id,
    });
    Ok(Accepted(Some("You liked this exercise".to_string())))
    // }
}

#[post("/exercise/<id>/unlike")]
pub fn unlike_exercise(
    cookies: &CookieJar<'_>,
    id: i32,
) -> Result<Accepted<String>, Unauthorized<String>> {
    let claim = decode::<Claim>(
        cookies.get("token").unwrap().value(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims;
    if verify_permission::verify_like_owner(&claim, id) {
        exercise_manipulation::unlike_exercise(Like {
            u_id: claim.username,
            ex_id: id,
        });
        return Ok(Accepted(Some("You unliked this exercise".to_string())));
    }
    Err(Unauthorized(Some(
        "You have not liked this exercise".to_string(),
    )))
}
