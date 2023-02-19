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
        name -> Text,
        text -> Text,
    }
}

diesel::table! {
    devices (id) {
        id -> Int4,
        user_id -> Int4,
        pubkey -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    folder_requests (folder_id, device_id) {
        folder_id -> Int4,
        device_id -> Int4,
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
    note_requests (note_id, device_id) {
        note_id -> Int4,
        device_id -> Int4,
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
    pending_devices (id) {
        id -> Int4,
        email -> Varchar,
        pubkey -> Varchar,
        password -> Varchar,
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
diesel::joinable!(folder_requests -> devices (device_id));
diesel::joinable!(folder_requests -> folders (folder_id));
diesel::joinable!(folders -> users (user_id));
diesel::joinable!(note_requests -> devices (device_id));
diesel::joinable!(note_requests -> notes (note_id));
diesel::joinable!(notes -> folders (folder_id));

diesel::allow_tables_to_appear_in_same_query!(
    device_folders,
    device_notes,
    devices,
    folder_requests,
    folders,
    note_requests,
    notes,
    pending_devices,
    pending_users,
    users,
);
