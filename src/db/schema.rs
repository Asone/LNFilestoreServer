table! {
    api_payment (uuid) {
        uuid -> Uuid,
        request -> Text,
        state -> Nullable<Text>,
        hash -> Text,
        expires_at -> Timestamptz,
    }
}

table! {
    media (uuid) {
        uuid -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        absolute_path -> Text,
        price -> Int4,
        payment_duration -> Nullable<Int4>,
        published -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    media_payment (uuid) {
        uuid -> Uuid,
        request -> Text,
        state -> Nullable<Text>,
        hash -> Text,
        media_uuid -> Uuid,
        expires_at -> Timestamptz,
        valid_until -> Nullable<Timestamptz>,
    }
}

table! {
    session (uuid) {
        uuid -> Uuid,
        token -> Text,
        user_uuid -> Uuid,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::models::user::UserRoleEnumMapping;

    user (id) {
        id -> Int4,
        uuid -> Uuid,
        login -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        role -> UserRoleEnumMapping,
    }
}

joinable!(media_payment -> media (media_uuid));

allow_tables_to_appear_in_same_query!(api_payment, media, media_payment, session, user,);
