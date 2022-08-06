pub use crate::db::schema::media;

use crate::graphql::types::input::file::FileInput;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use std::path::PathBuf;

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
}
