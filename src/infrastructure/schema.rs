// @generated automatically by Diesel CLI.

diesel::table! {
    admin_permissions (id) {
        id -> Uuid,
        user_id -> Uuid,
        permission -> Int4,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        role -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        reset_token -> Nullable<Varchar>,
        reset_token_expiry -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}

diesel::joinable!(admin_permissions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    admin_permissions,
    users,
);
