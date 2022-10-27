pub use crate::db::schema::media;

use crate::graphql::types::input::file::FileInput;
use crate::graphql::types::input::media::EditMediaInput;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[primary_key(uuid)]
#[table_name = "media"]
pub struct Media {
    pub uuid: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub absolute_path: String,
    pub price: i32,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "media"]
pub struct NewMedia {
    pub uuid: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub absolute_path: String,
    pub published: bool,
    pub price: i32,
}

#[derive(Debug, Queryable, AsChangeset)]
#[table_name = "media"]
pub struct EditMedia {
    pub title: Option<String>,
    pub description: Option<String>,
    pub published: Option<bool>,
    pub price: Option<i32>,
}

impl From<EditMediaInput> for EditMedia {
    fn from(edited_media: EditMediaInput) -> Self {
        Self {
            title: edited_media.title,
            description: edited_media.description,
            published: edited_media.published,
            price: edited_media.price,
        }
    }
}

impl From<(&PathBuf, FileInput)> for NewMedia {
    fn from(file_data: (&PathBuf, FileInput)) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
            title: file_data.1.title,
            description: file_data.1.description,
            absolute_path: file_data.0.to_string_lossy().to_owned().to_string(),
            price: file_data.1.price,
            published: file_data.1.published,
        }
    }
}

impl Media {
    pub fn create(new_media: NewMedia, connection: &PgConnection) -> QueryResult<Media> {
        use crate::db::schema::media::dsl::*;

        diesel::insert_into::<media>(media)
            .values(&new_media)
            .get_result(connection)
    }

    pub fn find_all_published(connection: &PgConnection) -> Vec<Media> {
        use crate::db::schema::media::dsl::*;
        media.filter(published.eq(true)).load(connection).unwrap()
    }

    pub fn find_one_by_uuid(
        media_uuid: uuid::Uuid,
        connection: &PgConnection,
    ) -> QueryResult<Option<Media>> {
        use crate::db::schema::media::dsl::*;

        media
            .filter(uuid.eq(media_uuid))
            .first::<Media>(connection)
            .optional()
    }

    pub fn update(
        media_uuid: Uuid,
        edited_media_input: EditMediaInput,
        connection: &PgConnection,
    ) -> QueryResult<Media> {
        use crate::db::schema::media::dsl::*;

        diesel::update(media.filter(uuid.eq(media_uuid)))
            .set(EditMedia::from(edited_media_input))
            .get_result::<Media>(connection)
    }

    pub fn delete(media_uuid: Uuid, connection: &PgConnection) -> QueryResult<usize> {
        use crate::db::schema::media::dsl::*;
        diesel::delete(media)
            .filter(uuid.eq(media_uuid))
            .execute(connection)
    }
}
