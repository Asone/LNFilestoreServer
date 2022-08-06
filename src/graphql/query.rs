use super::queries::get_files_list::get_files_list;
use super::queries::get_media::get_media;
use super::queries::get_post::get_post;
use super::queries::get_posts_list::get_posts_list;
use super::queries::request_invoice_for_media::request_invoice_for_media;
use super::queries::request_invoice_for_post::request_invoice_for_post;
use super::types::input::post::PayablePostInput;
use super::types::output::media::MediaType;
use super::types::output::payment::PaymentType;
use super::types::output::post::PostType;
use super::types::output::post::PreviewPostType;
use crate::db::models::media::Media;
use crate::graphql::context::GQLContext;
use crate::graphql::types::output::invoices::MediaInvoice;
use juniper::FieldError;
use uuid::Uuid;
pub struct Query;

#[juniper::graphql_object(context = GQLContext)]
impl Query {
    /*
       Retrieves the list of posts from DB.
       We use a specific post type here to limit the accessible information of posts through
       the request
    */
    #[graphql(description = "Retrieves the list of posts")]
    async fn get_posts_list(context: &'a GQLContext) -> Result<Vec<PreviewPostType>, FieldError> {
        get_posts_list(context).await
    }

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

    #[graphql(description = "Requests a ln query paywall invoice for a given post")]
    async fn request_invoice_for_post(
        context: &'a GQLContext,
        post_id: uuid::Uuid,
    ) -> Result<PaymentType, FieldError> {
        request_invoice_for_post(context, post_id).await
    }

    /*
     *
     */
    #[graphql(description = "Gets a specific post. The query is protected through a paywall")]
    async fn get_media<'a, 'b>(
        context: &'a GQLContext,
        uuid: Uuid,
        payment_request: Option<String>,
    ) -> Result<MediaType, FieldError> {
        get_media(context, uuid, payment_request).await
    }
    /*
       Gets a post.
       This is the main request where paywall shall be applied.
    */
    #[graphql(description = "Gets a specific post. The query is protected through a paywall")]
    async fn get_post<'a, 'b>(
        context: &'a GQLContext,
        post: PayablePostInput,
    ) -> Result<PostType, FieldError> {
        get_post(context, post).await
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
}
