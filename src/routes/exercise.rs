use crate::db::exercise_manipulation;
use crate::models::exercise::{CreateExercise, Exercise, UpdateExercise};
use rocket::serde::json::Json;

#[get("/exercise/<id>")]
pub fn get_exercise(id: String) -> Json<Exercise> {
    exercise_manipulation::get_exercise(id.parse::<i32>().unwrap())
        .unwrap()
        .into()
}

#[post("/exercise", format = "application/json", data = "<exercise>")]
pub fn create_exercise(exercise: Json<CreateExercise>) -> Json<bool> {
    exercise_manipulation::new_exercise(exercise.into_inner()).into()
}

#[put("/exercise/<id>", format = "application/json", data = "<exercise>")]
pub fn update_exercise(id: String, exercise: Json<UpdateExercise>) -> Json<Exercise> {
    exercise_manipulation::update_exercise(id.parse::<i32>().unwrap(), exercise.into_inner())
        .unwrap()
        .into()
}
