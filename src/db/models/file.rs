pub use crate::db::schema::file;

use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[primary_key(uuid)]
#[table_name = "file"]
pub struct File {
    pub uuid: uuid::Uuid,
    pub absolute_path: String,
    pub uploaded_by: uuid::Uuid,
    pub checksum: String,
    pub size: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "file"]
pub struct NewFile {
    pub absolute_path: String,
    pub uploaded_by: uuid::Uuid,
    pub checksum: String,
    pub size: i32,
}

impl File {
    pub fn create(new_file: NewFile, connection: &PgConnection) -> QueryResult<File> {
        use crate::db::schema::file::dsl::*;

        diesel::insert_into::<file>(file)
            .values(&new_file)
            .get_result(connection)
    }

    pub fn delete(uuid: uuid::Uuid) -> () {}

    pub fn find_one_by_uuid(
        file_uuid: uuid::Uuid,
        connection: &PgConnection,
    ) -> QueryResult<Option<File>> {
        use crate::db::schema::file::dsl::*;

        file.filter(uuid.eq(file_uuid))
            .first::<File>(connection)
            .optional()
    }
}
