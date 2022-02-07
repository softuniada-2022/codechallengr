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
        &cookies.get("token").unwrap().value().to_string(),
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
    if claim.username == "".to_string() {
        return Ok(exercise_manipulation::get_exercise(id).unwrap().into());
    }
    let ex = exercise_manipulation::get_exercise(id).unwrap();
    Err(LoggedInExercise {
        ex_id: ex.ex_id,
        u_id: ex.u_id,
        ex_name: ex.ex_name,
        ex_description: ex.ex_description,
        ex_input: ex.ex_input,
        ex_answer: ex.ex_answer,
        ex_difficulty: ex.ex_difficulty,
        ex_likes: ex.ex_likes,
        liked_by_me: verify_permission::verify_like_owner(&claim, id),
        ex_created_at: ex.ex_created_at,
        ex_updated_at: ex.ex_updated_at,
    }
    .into())
}

#[get("/exercise?<limit>&<sort_by>&<order>")]
pub fn filter_exercise(
    cookies: &CookieJar<'_>,
    limit: i32,
    sort_by: String,
    order: String,
) -> Result<Json<Vec<Exercise>>, Json<Vec<LoggedInExercise>>> {
    let claim = decode::<Claim>(
        &cookies.get("token").unwrap().value().to_string(),
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
    let a = exercise_manipulation::filter_exercise(limit, &sort_by, &order);
    if claim.username == "".to_string() {
        return Ok(a.into());
    }
    let mut out = vec![];
    for exercise in a {
        out.push(LoggedInExercise {
            ex_id: exercise.ex_id,
            u_id: exercise.u_id,
            ex_name: exercise.ex_name,
            ex_description: exercise.ex_description,
            ex_input: exercise.ex_input,
            ex_answer: exercise.ex_answer,
            ex_difficulty: exercise.ex_difficulty,
            ex_likes: exercise.ex_likes,
            liked_by_me: verify_permission::verify_like_owner(&claim, exercise.ex_id as i32),
            ex_created_at: exercise.ex_created_at,
            ex_updated_at: exercise.ex_updated_at,
        });
    }
    Err(out.into())
}

#[post("/exercise", format = "application/json", data = "<exercise>")]
pub fn create_exercise(
    cookies: &CookieJar<'_>,
    exercise: Json<UpdateExercise>,
) -> Result<Json<Exercise>, Unauthorized<String>> {
    dotenv().ok();
    let ex = exercise.into_inner();
    let claim = decode::<Claim>(
        &cookies.get("token").unwrap().value().to_string(),
        &DecodingKey::from_secret(env::var("JWT_KEY").unwrap().as_bytes()),
        &Validation::default(),
    )
    .unwrap()
    .claims;
    if verify_permission::verify_allowed_author(&claim) {
        let exercise = NewExercise {
            ex_name: ex.ex_name,
            u_id: claim.username.clone(),
            ex_description: ex.ex_description,
            ex_input: ex.ex_input,
            ex_answer: ex.ex_answer,
            ex_difficulty: ex.ex_difficulty,
            ex_likes: 0,
            ex_created_at: chrono::Utc::now().naive_utc(),
            ex_updated_at: chrono::Utc::now().naive_utc(),
        };
        exercise_manipulation::new_exercise(&exercise);
        return Ok(Exercise::from(exercise).into());
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
        &cookies.get("token").unwrap().value().to_string(),
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

#[get("/exercise/<id>/like")]
pub fn like_exercise(
    cookies: &CookieJar<'_>,
    id: i32,
) -> Result<Accepted<String>, Unauthorized<String>> {
    let claim = decode::<Claim>(
        &cookies.get("token").unwrap().value().to_string(),
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
    return Ok(Accepted(Some("You liked this exercise".to_string())));
    // }
}

#[get("/exercise/<id>/unlike")]
pub fn unlike_exercise(
    cookies: &CookieJar<'_>,
    id: i32,
) -> Result<Accepted<String>, Unauthorized<String>> {
    let claim = decode::<Claim>(
        &cookies.get("token").unwrap().value().to_string(),
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
    return Err(Unauthorized(Some(
        "You have not liked this exercise".to_string(),
    )));
}
