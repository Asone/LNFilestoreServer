use chrono::NaiveDateTime;

use crate::db::models::Post;

#[derive(GraphQLObject)]
#[graphql(description = "Full Post output type")]
pub struct PostType {
    pub uuid: uuid::Uuid,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub published: bool,
    pub price: i32,
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
    pub uuid: uuid::Uuid,
    pub title: String,
    pub excerpt: String,
    pub created_at: NaiveDateTime,
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
