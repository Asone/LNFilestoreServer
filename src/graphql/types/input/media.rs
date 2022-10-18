#[derive(Clone, GraphQLInputObject)]
pub struct EditMediaInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub published: Option<bool>,
}