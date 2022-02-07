use crate::models::schema::users;
use crate::models::score::Score;
use crate::models::users::User;
use crate::utils::establish_connection::establish_connection;
use cain::cain;
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
pub fn get_lim_scores(lim: i32) -> Vec<Score> {
    let conn = establish_connection();
    let scores = users::table.order(users::u_score.desc());
    cain! {
        let scores = if lim == 0 {
            scores
        } else {
            scores.limit(lim as i64)
        };
        let scores = scores.limit(lim as i64)
            .load::<User>(&conn)
            .expect("Error loading users");
        scores.into_iter().map(|s| Score {
            user: s.u_name,
            score: s.u_score,
        }).collect()
    }
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
