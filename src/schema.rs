table! {
    exercises (ex_id) {
        ex_id -> Unsigned<Bigint>,
        t_name -> Varchar,
        t_description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    solutions (s_id) {
        s_id -> Unsigned<Bigint>,
        ex_id -> Integer,
        u_id -> Integer,
        submitted_at -> Timestamp,
    }
}

table! {
    users (u_id) {
        u_id -> Unsigned<Bigint>,
        u_name -> Varchar,
        email -> Varchar,
        u_password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    exercises,
    solutions,
    users,
);
