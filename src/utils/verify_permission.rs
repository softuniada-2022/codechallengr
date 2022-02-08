use crate::db::{exercise_manipulation, solution_manipulation, user_manipulation};
use crate::models::likes::Like;
use crate::models::permissions::Permission;
use crate::models::users::Claim;

pub fn verify_allowed_author(claim: &Claim) -> bool {
    match claim.perm {
        Permission::Admin => true,
        Permission::AuthAuthor => true,
        Permission::User => false,
        Permission::Guest => false,
    }
}

pub fn verify_sender_self(claim: &Claim, receiver: &String) -> bool {
    claim.username == user_manipulation::get_user(receiver).unwrap().u_name
}

pub fn verify_author(claim: &Claim, exercise: i32) -> bool {
    exercise_manipulation::get_exercise(exercise).unwrap().u_id == claim.username
}

pub fn verify_like_owner(claim: &Claim, target: i32) -> bool {
    claim.username
        == exercise_manipulation::get_like(Like {
            u_id: claim.username.clone(),
            ex_id: target,
        })
        .unwrap_or(Like {
            u_id: "".to_string(),
            ex_id: 0,
        })
        .u_id
}

pub fn check_prev_solutions(claim: &Claim, target: i32) -> bool {
    for solution in solution_manipulation::get_all_solutions_for_user(&target, &claim.username) {
        if solution.s_correct == true {
            return true;
        }
    }
    false
}
