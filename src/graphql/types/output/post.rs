use chrono::NaiveDateTime;

use crate::db::models::Post;

#[derive(GraphQLObject)]
#[graphql(description = "Full Post output type")]
pub struct PostType {
    #[graphql(description = "The post id")]
    pub uuid: uuid::Uuid,
    #[graphql(description = "Title of post")]
    pub title: String,
    #[graphql(description = "Short overview of post")]
    pub excerpt: String,
    #[graphql(description = "Full content of post")]
    pub content: String,
    #[graphql(description = "Publish status of post")]
    pub published: bool,
    #[graphql(description = "Price of post access in satoshis. If free is 0")]
    pub price: i32,
    #[graphql(description = "Creation date of post")]
    pub created_at: NaiveDateTime,
}

impl From<Post> for PostType {
    fn from(item: Post) -> Self {
        Self {
            uuid: item.uuid,
            title: item.title,
            excerpt: item.excerpt,
            content: item.content,
            published: item.published,
            price: item.price,
            created_at: item.created_at,
        }
    }
}

#[derive(GraphQLObject)]
#[graphql(description = "Preview Post output type")]
pub struct PreviewPostType {
    #[graphql(description = "The post id")]
    pub uuid: uuid::Uuid,
    #[graphql(description = "The title of the post")]
    pub title: String,
    #[graphql(description = "Short overview of the post")]
    pub excerpt: String,
    #[graphql(description = "Creation date of the post")]
    pub created_at: NaiveDateTime,
    #[graphql(description = "ln value to pay for content access")]
    pub price: i32,
}

impl From<Post> for PreviewPostType {
    fn from(item: Post) -> Self {
        Self {
            uuid: item.uuid,
            title: item.title,
            excerpt: item.excerpt,
            price: item.price,
            created_at: item.created_at,
        }
    }
}
