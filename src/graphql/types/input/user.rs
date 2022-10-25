#[derive(Clone, GraphQLInputObject)]
pub struct NewUserInput {
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, GraphQLInputObject)]
pub struct EditUserInput {
    pub email: Option<String>,
    pub password: Option<String>,
}
