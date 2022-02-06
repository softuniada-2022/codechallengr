table! {
    exercises (ex_id) {
        ex_id -> Unsigned<Bigint>,
        u_id -> Varchar,
        ex_name -> Varchar,
        ex_description -> Text,
        ex_input -> Text,
        ex_answer -> Text,
        ex_difficulty -> Integer,
        ex_likes -> Integer,
        ex_created_at -> Datetime,
        ex_updated_at -> Datetime,
    }
}

table! {
    likes (u_id, ex_id) {
        u_id -> Varchar,
        ex_id -> Integer,
    }
}

table! {
    solutions (s_id) {
        s_id -> Unsigned<Bigint>,
        ex_id -> Integer,
        u_id -> Text,
        s_answer -> Text,
        s_correct -> Bool,
        s_submitted_at -> Datetime,
    }
}

table! {
    users (u_name) {
        u_name -> Varchar,
        u_email -> Varchar,
        u_password -> Varchar,
        u_permission -> Integer,
        u_score -> Integer,
        u_created_at -> Datetime,
        u_updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(exercises, likes, solutions, users,);
