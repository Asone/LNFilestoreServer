
pub use crate::db::schema::file;

use crate::graphql::types::input::file::FileInput;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use std::path::PathBuf;
 

#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[primary_key(uuid)]
#[table_name = "file"]
pub struct File{
    pub uuid: uuid::Uuid,
    pub absolute_path: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}