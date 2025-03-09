diesel::table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        company_id -> Int4,
        full_name -> Varchar,
        initial -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    stores (id) {
        id -> Int4,
        name -> Varchar,
        company_id -> Int4,
        initial -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_stores (user_id, store_id) {
        user_id -> Int4,
        store_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(user_stores -> users (user_id));
diesel::joinable!(user_stores -> stores (store_id));

diesel::allow_tables_to_appear_in_same_query!(users, stores, user_stores);
