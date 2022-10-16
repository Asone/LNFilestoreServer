use super::file::File;
use crate::db::media_type_enum::MediaTypeEnum;
use crate::db::media_type_enum::MediaTypeEnumMapping;
pub use crate::db::schema::media;
use crate::graphql::types::input::file::FileInput;
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
    pub price: i32,
    pub published: bool,
    pub file_uuid: uuid::Uuid,
    pub type_: MediaTypeEnum,
    pub metadata: uuid::Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[primary_key(uuid)]
#[table_name = "media"]
pub struct MediaDbModel {
    pub uuid: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub published: bool,
    pub file_uuid: uuid::Uuid,
    pub type_: MediaTypeEnum,
    pub metadata: uuid::Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Associations)]
#[table_name = "media"]
#[belongs_to(parent = File, foreign_key = "file_uuid")]
// #[belongs_to(parent = Metadata, foreign_key = "file_uuid")]
pub struct NewMedia {
    pub uuid: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub published: bool,
    pub price: i32,
    pub file_uuid: uuid::Uuid,
    pub type_: MediaTypeEnum,
    pub metadata: uuid::Uuid,
}

#[derive(Debug, Queryable, Identifiable, PartialEq, AsChangeset)]
#[primary_key(uuid)]
#[table_name = "media"]
pub struct EditMedia {
    pub uuid: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub published: bool,
    pub price: i32,
}

impl From<(&PathBuf, FileInput)> for NewMedia {
    fn from(file_data: (&PathBuf, FileInput)) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4(),
            title: file_data.1.title,
            description: file_data.1.description,
            price: file_data.1.price,
            published: file_data.1.published,
            file_uuid: uuid::Uuid::new_v4(),
            type_: MediaTypeEnum::Default,
            metadata: uuid::Uuid::new_v4(),
        }
    }
}

// impl From<MediaDbModel> for Media {
//     fn from(media: MediaDbModel) -> Self {
//         Self { uuid: (), title: (), description: (), price: (), published: (), file_uuid: (), type_: (), metadata: (), created_at: (), updated_at: () }
//     }
// }

impl Media {
    pub fn create(new_media: NewMedia, connection: &PgConnection) -> QueryResult<Media> {
        use crate::db::schema::media::dsl::*;

        diesel::insert_into::<media>(media)
            .values(&new_media)
            .get_result(connection)
    }

    pub fn edit(edit_media: EditMedia, connection: &PgConnection) -> QueryResult<Media> {
        use crate::db::schema::media::dsl::*;
        diesel::update(media)
            .set(&edit_media)
            .get_result(connection)
    }

    pub fn delete() -> () {
        use crate::db::schema::media::dsl::*;

        // diesel::delete<media>(media.filter(id))
        ()
    }

    pub fn find_all_published(connection: &PgConnection) -> Vec<Media> {
        use crate::db::schema::media::dsl::*;
        media.filter(published.eq(true)).load(connection).unwrap()
    }

    pub fn find_all(connection: &PgConnection) -> Vec<Media> {
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
