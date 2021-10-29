pub use crate::db::schema::post;
use crate::graphql::types::input::post::CreatePostInput;
use chrono::NaiveDateTime;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

// Represents the object as described in Database
#[derive(Identifiable, Queryable, PartialEq)]
#[primary_key(uuid)]
#[table_name = "post"]
pub struct Post {
    pub uuid: uuid::Uuid,
    pub title: String,
    pub excerpt: String,
    pub content: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub price: i32,
}


impl Post {

    // Allow to check if post has a price 
    // so we can build and call the paywall
    // when someone requests the content of post
    pub fn is_payable(&self) -> bool {
        if &self.price > &0 {
            return true;
        } else {
            return false;
        }
    }
}

#[derive(Debug, Insertable)]
#[table_name = "post"]
pub struct NewPost{
    pub uuid: uuid::Uuid,
    pub title: String,
    pub content: String,
    pub excerpt: String,
    pub published: bool,
    pub price: i32,
}

impl  From<CreatePostInput> for NewPost{
    fn from(item: CreatePostInput) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            title: item.title,
            excerpt: item.excerpt,
            content: item.content,
            published: item.published, // Default value is false
            price: item.price,
        }
    }
}

impl Post {

    // Creates a post
    pub fn create(new_post: NewPost, connection: &PgConnection) -> QueryResult<Post> {
        use crate::db::schema::post::dsl::*;

        diesel::insert_into::<post>(post)
            .values(&new_post)
            .get_result(connection)
    }

    // Finds a post based on the provided id
    pub fn find_one_by_id(post_id: uuid::Uuid, connection: &PgConnection) -> Option<Post> {
        use crate::db::schema::post::dsl::*;
        post.filter(uuid.eq(post_id))
            .first::<Post>(connection)
            .optional()
            .unwrap()
    }

    pub fn find_all_published(connection: &PgConnection) -> Vec<Post> {
        use crate::db::schema::post::dsl::*;
        post.filter(published.eq(true)).load(connection).unwrap()
    }
}
