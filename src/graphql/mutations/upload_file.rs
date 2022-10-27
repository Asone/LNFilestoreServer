use juniper::{FieldError, FieldResult, Value};

use crate::{
    db::models::media::{Media, NewMedia},
    graphql::{
        context::GQLContext,
        types::{input::file::FileInput, output::media::MediaType},
    },
};

pub async fn upload_file<'a>(
    context: &'a GQLContext,
    file_input: FileInput,
) -> FieldResult<MediaType> {
    let files_map = context.get_files();
    let connection = context.get_db_connection();

    match files_map {
        Some(files_map) => {
            if files_map.len() == 0 {
                return Err(FieldError::new(
                    "Current mutation requires a single file for upload. No file provided",
                    Value::null(),
                ));
            }

            let file = files_map.into_iter().next();

            match file {
                Some(file) => {
                    let persisted_path = file.1.persist_file();

                    match persisted_path {
                        Ok(path) => {
                            let new_media = NewMedia::from((&path, file_input));
                            let media = connection.run(move |c| Media::create(new_media, c)).await;
                            match media {
                                Ok(media) => Ok(MediaType::from(media)),
                                Err(_) => Err(FieldError::new(
                                    "Error while persisting file. Aborting",
                                    Value::null(),
                                )),
                            }
                        }
                        Err(_) => Err(FieldError::new(
                            "Error while writing file on filesystem.",
                            Value::null(),
                        )),
                    }
                }
                None => Err(FieldError::new(
                    "Current mutation requires a single file for upload. No file provided",
                    Value::null(),
                )),
            }
        }
        None => Err(FieldError::new(
            "Current mutation accepts a single file for upload. Multiple files uploaded provided",
            Value::null(),
        )),
    }
}
