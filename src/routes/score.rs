use crate::db::score_manipulation;
use crate::models::score::Score;
use rocket::serde::json::Json;

#[post("/score/<number_score>")]
pub fn get_num_scores(number_score: i32) -> Json<Vec<Score>> {
    score_manipulation::get_num_scores(number_score).into()
}
