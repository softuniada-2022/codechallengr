use crate::models::schema::users;
use crate::models::score::Score;
use crate::models::users::User;
use crate::utils::establish_connection::establish_connection;
use diesel::prelude::*;

// increment user's score
pub fn increment_score(id: String, num: i32) -> bool {
    let conn = establish_connection();
    let affected = diesel::update(users::table)
        .filter(users::u_name.eq(id))
        .set(users::u_score.eq(users::u_score + num))
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => true,
        Some(0) => false,
        _ => false,
    }
}

pub fn decrement_score(id: String) -> bool {
    let conn = establish_connection();
    let affected = diesel::update(users::table)
        .filter(users::u_name.eq(id))
        .set(users::u_score.eq(users::u_score - 1))
        .execute(&conn)
        .ok();
    match affected {
        Some(1) => true,
        Some(0) => false,
        _ => false,
    }
}

// get N scores
pub fn get_num_scores(x: i32) -> Vec<Score> {
    let conn = establish_connection();
    let scores = users::table
        .order(users::u_score.desc())
        .limit(x as i64)
        .load::<User>(&conn)
        .expect("Error loading users");
    let mut score_vec: Vec<Score> = Vec::new();
    for score in scores {
        score_vec.push(Score {
            user: score.u_name,
            score: score.u_score,
        });
    }
    score_vec
}

// get all scores

// pub fn get_all_score() -> Vec<Score> {
//     let conn = establish_connection();
//     let results: Vec<User> = users::table
//         .order(users::u_score.desc())
//         .load::<User>(&conn).ok().unwrap();
//     let mut ret: Vec<Score> = vec![];
//     for result in results {
//         ret.push(Score {
//             user: result.u_name,
//             score: result.u_score,
//         });
//     }
//     ret
// }
