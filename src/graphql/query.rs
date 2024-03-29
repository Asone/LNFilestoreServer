use super::queries::get_files_relay::get_files_list_relay;
use super::queries::get_media::get_media;
use super::queries::request_invoice_for_media::request_invoice_for_media;
use super::queries::users_relay::users_relay;
use super::types::output::media::MediaType;
use super::{queries::get_files_list::get_files_list, types::output::user::UserType};
use crate::db::models::media::Media;
use crate::graphql::context::GQLContext;
use crate::graphql::types::output::invoices::MediaInvoice;
use juniper::{FieldError, FieldResult};
use juniper_relay_connection::RelayConnection;
use uuid::Uuid;
pub struct Query;

#[juniper::graphql_object(context = GQLContext)]
impl Query {
    #[graphql(description = "Requests list of files")]
    async fn get_files_list(context: &'a GQLContext) -> Result<Vec<MediaType>, FieldError> {
        get_files_list(context).await
    }

    #[graphql(description = r#"
        Requests an invoice for a media.
        If a payment_request is provided, the query will check
        for the provided payment_request status and provide a new onee
        if necessary.
    "#)]
    async fn request_invoice_for_media(
        context: &'a GQLContext,
        uuid: uuid::Uuid,
        payment_request: Option<String>,
    ) -> Result<MediaInvoice, FieldError> {
        request_invoice_for_media(context, uuid, payment_request).await
    }

    #[graphql(description = "Gets a specific post. The query is protected through a paywall")]
    async fn get_media<'a, 'b>(
        context: &'a GQLContext,
        uuid: Uuid,
        payment_request: Option<String>,
    ) -> Result<MediaType, FieldError> {
        get_media(context, uuid, payment_request).await
    }

    #[graphql(description = "Gets the list of available medias")]
    async fn get_medias_list(context: &'a GQLContext) -> Result<Vec<MediaType>, FieldError> {
        let connection = context.get_db_connection();
        let db_results = connection.run(move |c| Media::find_all_published(c)).await;

        Ok(db_results
            .into_iter()
            .map(|media| MediaType::from(media))
            .collect::<Vec<MediaType>>())
    }

    #[graphql(description = "Gets available files with relay pagination")]
    async fn get_files_relay(context: &'a GQLContext) -> FieldResult<RelayConnection<MediaType>> {
        get_files_list_relay(context, None, None, None, None).await
    }

    #[graphql(description = "Get a relay pagination of users")]
    async fn users_relay(
        context: &'a GQLContext,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> FieldResult<RelayConnection<UserType>> {
        users_relay(context, first, after, last, before).await
    }
}
