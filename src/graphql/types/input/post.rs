/**
 This should provide a generic Input type for payable
 data.
 The input should expect an optional `payment` field
 which would indicate a payment related to accessing
 The requested piece of data.

 If the field is not provided, the server should then
 build an invoice that would be used to return a payment_request
 to the client.
*/
#[derive(Clone, GraphQLInputObject)]
pub struct PayablePostInput {
    pub payment_request: Option<String>,
    pub uuid: uuid::Uuid,
}

impl PayablePostInput {
}

#[derive(GraphQLInputObject)]
pub struct CreatePostInput {
    pub title: String,
    pub content: String,
    pub excerpt: String,
    pub published: bool,
    pub price: i32,
}