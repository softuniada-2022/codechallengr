use crate::db::score_manipulation;
use crate::models::score::Score;
use rocket::serde::json::Json;

#[post("/score", format = "application/json", data = "<number_score>")]
pub fn get_num_scores(number_score: Json<i32>) -> Json<Vec<Score>> {
    Json(score_manipulation::get_num_scores(
        number_score.into_inner(),
    ))
}
