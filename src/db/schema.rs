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

joinable!(payment -> post (post_uuid));

allow_tables_to_appear_in_same_query!(payment, post,);
