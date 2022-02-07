use crate::db::score_manipulation;
use crate::models::score::Score;
use rocket::serde::json::Json;

#[get("/score?<limit>")]
pub fn get_scores(limit: Option<i32>) -> Json<Vec<Score>> {
    score_manipulation::get_lim_scores(limit.unwrap_or(0)).into()
}
