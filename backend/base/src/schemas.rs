table! {
    folders (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        state -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    notes (id) {
        id -> Int4,
        folder_id -> Int4,
        commit -> Int4,
        title -> Nullable<Text>,
        text -> Text,
        state -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(folders -> users (user_id));
joinable!(notes -> folders (folder_id));

allow_tables_to_appear_in_same_query!(
    folders,
    notes,
    users,
);
