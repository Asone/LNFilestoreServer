#[derive(Clone, GraphQLEnum)]
pub enum MediaType {
    Default,
    Audio,
    Video,
    Epub,
    Pdf,
    Image,
}

#[derive(Clone, GraphQLInputObject)]
pub struct FileInput {
    pub filename: String,
    pub title: String,
    pub price: i32,
    pub description: Option<String>,
    pub published: bool,
    pub kind: MediaType,
    // We expect this to be always `null` as per the spec
    // see : https://github.com/jaydenseric/graphql-multipart-request-spec
    pub file: Option<String>,
}
