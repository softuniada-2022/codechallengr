use crate::models::models::{Exercise};
use crate::db::exercise_manipulation;
use rocket::serde::json::Json;

#[get("/exercise/<id>")]
pub fn get_exercise(id: String) -> Json<Exercise> {
    
}
