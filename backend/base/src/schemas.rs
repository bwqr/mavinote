table! {
    folders (id) {
        id -> Int4,
        user_id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

table! {
    notes (id) {
        id -> Int4,
        folder_id -> Int4,
        title -> Varchar,
        text -> Text,
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

allow_tables_to_appear_in_same_query!(
    folders,
    notes,
    users,
);
