use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{db::models::file::File, graphql::context::GQLContext};

#[derive(Debug)]
pub struct FileType {
    pub uuid: uuid::Uuid,
    pub absolute_path: String,
    pub uploaded_by: uuid::Uuid,
    pub checksum: String,
    pub size: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[graphql_object(
    name = "File", 
    description = "File output type"
    context = GQLContext
)]
impl FileType {
    #[graphql(description = "The file internal id")]
    pub fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    #[graphql(description = "The file checksum")]
    pub fn checksum(&self) -> &String {
        &self.checksum
    }

    #[graphql(description = "The file size")]
    pub fn size(&self) -> i32 {
        self.size
    }

    #[graphql(description = "The creation date on server")]
    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    #[graphql(description = "The creation date on server")]
    pub fn uploaded_by(&self) -> Uuid {
        self.uploaded_by
    }
}

impl From<File> for FileType {
    fn from(item: File) -> Self {
        Self {
            uuid: item.uuid,
            absolute_path: item.absolute_path,
            uploaded_by: item.uploaded_by,
            checksum: item.checksum,
            size: item.size,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
