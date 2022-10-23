use juniper::{FieldError, FieldResult};

use crate::db::models::{
    media::{Media, NewMedia},
    user::User,
};

use super::{
    context::GQLContext,
    types::{input::file::FileInput, output::media::MediaType},
    types::input::media::EditMediaInput
};
use crate::graphql::mutations::update_password;
use crate::graphql::mutations::edit_media;
pub struct Mutation;

impl Mutation {
    pub fn is_authenticated(user: &Option<User>) -> bool {
        match user {
            Some(_) => true,
            None => false,
        }
    }
}

#[juniper::graphql_object(context = GQLContext)]
impl Mutation {
    #[graphql(description = "Upload and stores a payable media onto the server")]
    async fn upload_file<'a>(
        context: &'a GQLContext,
        file_input: FileInput,
    ) -> FieldResult<MediaType> {
        if Self::is_authenticated(&context.user) == false {
            return Err(FieldError::new(
                "You need to be authenticated to use this mutation",
                graphql_value!(""),
            ));
        }

        let files_map = context.get_files();
        let connection = context.get_db_connection();

        match files_map {
            Some(files_map) => {

                if files_map.len() == 0 {
                    return Err(FieldError::new(
                        "Current mutation requires a single file for upload. No file provided",
                        graphql_value!("")
                    ))
                }

                let file = files_map.into_iter().next();

                match file {
                    Some(file) => {
                        let persisted_path = file.1.persist_file();

                        match persisted_path {
                            Ok(path) => {
                                let new_media = NewMedia::from((&path,file_input));
                                let media =  connection.run(move |c| Media::create(new_media,c)).await;
                                match media {
                                    Ok(media) => Ok(MediaType::from(media)),
                                    Err(_) => Err(FieldError::new(
                                        "Error while persisting file. Aborting",
                                    graphql_value!("")
                                    ))
                                }
                            },
                            Err(_) => Err(FieldError::new("Error while writing file on filesystem.",
                    graphql_value!("")
                            ))
                        }

                    },
                    None => Err(FieldError::new(
                        "Current mutation requires a single file for upload. No file provided",
                        graphql_value!("")
                    ))
                }
            },
            None => Err(FieldError::new(
                        "Current mutation accepts a single file for upload. Multiple files uploaded provided",
                        graphql_value!("")
                    ))
        }
    }

    /// Changes password for current user
    async fn change_password<'a>(context: &'a GQLContext, password: String) -> FieldResult<bool> {
        update_password::update_password(context, password).await
    }

    #[graphql(description = "Edit a media")]
    async fn edit_media<'a>(
        context: &'a GQLContext,
        uuid: uuid::Uuid,
        media: EditMediaInput,
    ) -> FieldResult<MediaType> {
        //  FieldResult<MediaType>
        if Self::is_authenticated(&context.user) == false {
            return Err(FieldError::new(
                "You need to be authenticated to use this mutation",
                graphql_value!(""),
            ));
        }

        edit_media::edit_media(context, uuid, media).await

    }
}
