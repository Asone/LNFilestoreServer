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
    audio_metadata (uuid) {
        uuid -> Uuid,
        codec -> Nullable<Text>,
        length -> Nullable<Text>,
        artist -> Nullable<Text>,
    }
}

table! {
    epub_metadata (uuid) {
        uuid -> Uuid,
    }
}

table! {
    file (uuid) {
        uuid -> Uuid,
        absolute_path -> Text,
        uploaded_by -> Uuid,
        checksum -> Text,
        size -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    image_metadata (uuid) {
        uuid -> Uuid,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::media_type_enum::MediaTypeEnumMapping;

    media (uuid) {
        uuid -> Uuid,
        title -> Text,
        description -> Nullable<Text>,
        price -> Int4,
        published -> Bool,
        file_uuid -> Uuid,
        #[sql_name = "type"]
        type_ -> MediaTypeEnumMapping,
        metadata -> Uuid,
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

table! {
    video_metadata (uuid) {
        uuid -> Uuid,
        codec -> Nullable<Text>,
        length -> Nullable<Text>,
    }
}

joinable!(media -> file (file_uuid));
joinable!(media_payment -> media (media_uuid));
joinable!(session -> user (user_uuid));

allow_tables_to_appear_in_same_query!(
    api_payment,
    audio_metadata,
    epub_metadata,
    file,
    image_metadata,
    media,
    media_payment,
    session,
    user,
    video_metadata,
);
