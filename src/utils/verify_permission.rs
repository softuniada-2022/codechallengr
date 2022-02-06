use crate::db::exercise_manipulation;
use crate::db::user_manipulation;
use bcrypt;

pub fn verify_permission(user: &String, hashed_perm: &String) -> bool {
    bcrypt::verify(
        user_manipulation::get_perm(&user).unwrap().to_string(),
        &hashed_perm,
    )
    .ok()
    .unwrap()
        || bcrypt::verify(
            user_manipulation::get_perm(&user).unwrap().to_string(),
            &hashed_perm,
        )
        .ok()
        .unwrap()
}

pub fn verify_author(username: String, exercise: i32) -> bool {
    exercise_manipulation::get_exercise(exercise).unwrap().u_id == username
}

pub fn verify_sender_self(username: &String, sender: String) -> bool {
    *username == sender
}

pub fn verify_hash(username: String, hash: String) -> bool {
    let pw = user_manipulation::get_user(&username).unwrap().u_password;
    bcrypt::verify(pw, &hash).ok().unwrap()
}
