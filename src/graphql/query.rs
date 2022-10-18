use super::queries::get_files_list::get_files_list;
use super::queries::get_files_relay::get_files_list_relay;
use super::queries::get_media::get_media;
use super::queries::request_invoice_for_media::request_invoice_for_media;
use super::types::output::media::MediaType;
use crate::graphql::context::GQLContext;
use crate::graphql::types::output::invoices::MediaInvoice;
use crate::{db::models::media::Media, graphql::types::output::media::MediaUnion};
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

    #[graphql(description = "
        Requests an invoice for a media. \n
        If a payment_request is provided, the query will check
        for the provided payment_request status and provide a new onee
        if necessary.
    ")]
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
    async fn get_medias_list(context: &'a GQLContext) -> Result<Vec<MediaUnion>, FieldError> {
        let connection = context.get_db_connection();
        let db_results = connection.run(move |c| Media::find_all_published(c)).await;

        Ok(db_results
            .into_iter()
            .map(|media| MediaUnion::from(media))
            .collect::<Vec<MediaUnion>>())
    }

    #[graphql(description = "Gets available files with relay pagination")]
    async fn get_files_relay(context: &'a GQLContext) -> FieldResult<RelayConnection<MediaUnion>> {
        get_files_list_relay(context, None, None, None, None).await
    }
}
