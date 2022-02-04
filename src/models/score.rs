use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Score {
    pub user: String,
    pub score: i32,
}
