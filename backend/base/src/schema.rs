// @generated automatically by Diesel CLI.

pub mod sql_types {
    use diesel::query_builder::QueryId;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "state"))]
    pub struct State;

    impl QueryId for State {
        type QueryId = diesel::sql_types::Text;

        const HAS_STATIC_QUERY_ID: bool = true;
    }
}

diesel::table! {
    device_folders (folder_id, receiver_device_id) {
        folder_id -> Int4,
        sender_device_id -> Int4,
        receiver_device_id -> Int4,
        name -> Text,
    }
}

diesel::table! {
    device_notes (note_id, receiver_device_id) {
        note_id -> Int4,
        sender_device_id -> Int4,
        receiver_device_id -> Int4,
        title -> Nullable<Text>,
        text -> Text,
    }
}

diesel::table! {
    devices (id) {
        id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::State;

    folders (id) {
        id -> Int4,
        user_id -> Int4,
        state -> State,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::State;

    notes (id) {
        id -> Int4,
        folder_id -> Int4,
        commit -> Int4,
        state -> State,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    pending_users (email) {
        code -> Varchar,
        email -> Varchar,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(device_folders -> folders (folder_id));
diesel::joinable!(device_notes -> notes (note_id));
diesel::joinable!(devices -> users (user_id));
diesel::joinable!(folders -> users (user_id));
diesel::joinable!(notes -> folders (folder_id));

diesel::allow_tables_to_appear_in_same_query!(
    device_folders,
    device_notes,
    devices,
    folders,
    notes,
    pending_users,
    users,
);
