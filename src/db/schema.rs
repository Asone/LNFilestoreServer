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
    payment (uuid) {
        uuid -> Uuid,
        request -> Text,
        state -> Nullable<Text>,
        hash -> Text,
        post_uuid -> Uuid,
        expires_at -> Timestamptz,
    }
}

table! {
    post (uuid) {
        uuid -> Uuid,
        title -> Text,
        excerpt -> Text,
        content -> Text,
        published -> Bool,
        created_at -> Timestamptz,
        price -> Int4,
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
    user (uuid) {
        uuid -> Uuid,
        login -> Text,
        email -> Text,
        password -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(payment -> post (post_uuid));
joinable!(session -> user (user_uuid));

allow_tables_to_appear_in_same_query!(api_payment, payment, post, session, user,);
