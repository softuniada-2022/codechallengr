use crate::db::exercise_manipulation;
use crate::db::user_manipulation;
use crate::models::exercise::NewExercise;
use crate::models::exercise::{Exercise, UpdateExercise};
use crate::models::likes::CreateLike;
use crate::models::users::LoginInformation;
use crate::utils::verify_permission;
use rocket::http::CookieJar;
use rocket::serde::json::Json;

#[get("/exercise/<id>")]
pub fn get_exercise(id: String) -> Json<Exercise> {
    exercise_manipulation::get_exercise(id.parse::<i32>().unwrap())
        .unwrap()
        .into()
}

#[post("/exercise", format = "application/json", data = "<exercise>")]
pub fn create_exercise(
    cookies: &CookieJar<'_>,
    exercise: Json<UpdateExercise>,
) -> Json<bool> {
    let ex = exercise.into_inner();
    let user_cookie = cookies.get("username").unwrap().value().to_string();
    let perm_cookie = cookies.get("perm").unwrap().value().to_string();
    if verify_permission::verify_permission(&user_cookie, &perm_cookie) {
        exercise_manipulation::new_exercise(NewExercise {
            ex_name: ex.ex_name,
            u_id: user_cookie,
            ex_description: ex.ex_description,
            ex_input: ex.ex_input,
            ex_answer: ex.ex_answer,
            ex_difficulty: ex.ex_difficulty,
            ex_likes: 0,
            ex_created_at: chrono::Utc::now().naive_utc(),
            ex_updated_at: chrono::Utc::now().naive_utc(),
        })
        .into()
    } else {
        false.into()
    }
}

#[put("/exercise/<id>", format = "application/json", data = "<exercise>")]
pub fn update_exercise(
    cookies: &CookieJar<'_>,
    id: String,
    exercise: Json<UpdateExercise>,
) -> Json<bool> {
    let ex = exercise.into_inner();
    if verify_permission::verify_author(
        cookies.get("username").unwrap().value().to_string(),
        id.parse::<i32>().unwrap(),
    ) {
        let current = exercise_manipulation::get_exercise(id.parse::<i32>().unwrap()).unwrap();
        return exercise_manipulation::update_exercise(
            id.parse::<i32>().unwrap(),
            NewExercise {
                ex_name: ex.ex_name,
                u_id: current.u_id,
                ex_description: ex.ex_description,
                ex_input: ex.ex_input,
                ex_answer: ex.ex_answer,
                ex_difficulty: ex.ex_difficulty,
                ex_likes: current.ex_likes,
                ex_created_at: current.ex_created_at,
                ex_updated_at: chrono::Utc::now().naive_utc(),
            },
        )
        .into();
    }
    false.into()
}

#[get("/exercise/<id>/input")]
pub fn get_input(id: String) -> Json<String> {
    exercise_manipulation::get_input(id.parse::<i32>().unwrap())
        .unwrap()
        .into()
}

#[post("/exercise/<id>/like")]
pub fn like_exercise(cookies: &CookieJar<'_>, id: String) -> Json<bool> {
    let user = cookies.get("username").unwrap().value().to_string();
    if user_manipulation::check_password(&LoginInformation {
        u_name: user.clone(),
        u_password: cookies.get("password").unwrap().value().to_string(),
    }) {
        return exercise_manipulation::like_exercise(CreateLike {
            u_id: user,
            ex_id: id.parse::<i32>().unwrap(),
        })
        .into();
    }
    false.into()
}

#[post("/exercise/<id>/unlike")]
pub fn unlike_exercise(cookies: &CookieJar<'_>, id: String) -> Json<bool> {
    let user = cookies.get("username").unwrap().value().to_string();
    if verify_permission::verify_sender_self(
        &user,
        cookies.get("password").unwrap().value().to_string(),
    ) {
        return exercise_manipulation::unlike_exercise(CreateLike {
            u_id: user,
            ex_id: id.parse::<i32>().unwrap(),
        })
        .into();
    }
    false.into()
}
