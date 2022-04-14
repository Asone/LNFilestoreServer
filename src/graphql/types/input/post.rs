/// This should provide a generic Input type for payable
/// data.
/// The input should expect an optional `payment` field
/// which would indicate a payment related to accessing
/// The requested piece of data.

/// If the field is not provided, the server should then
/// build an invoice that would be used to return a payment_request
/// to the client.
#[derive(Clone, GraphQLInputObject)]
pub struct PayablePostInput {
    #[graphql(description = "The ln paywall payment request string")]
    pub payment_request: Option<String>,
    #[graphql(description = "The requested post id")]
    pub uuid: uuid::Uuid,
}

impl PayablePostInput {}

#[derive(GraphQLInputObject)]
pub struct CreatePostInput {
    #[graphql(description = "The title of post")]
    pub title: String,
    #[graphql(description = "The content of the post")]
    pub content: String,
    #[graphql(description = "A short overview of the post")]
    pub excerpt: String,
    #[graphql(description = "Publishing status of the post")]
    pub published: bool,
    #[graphql(description = "The access price for post")]
    pub price: i32,
}
