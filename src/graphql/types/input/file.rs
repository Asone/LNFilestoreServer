#[derive(Clone, GraphQLInputObject)]
pub struct FileInput {
    pub filename: String,
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub published: bool,
    // We expect this to be always `null` as per the spec
    // see : https://github.com/jaydenseric/graphql-multipart-request-spec
    pub file: Option<String>,
}
