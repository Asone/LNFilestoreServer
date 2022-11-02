#[derive(Clone, GraphQLEnum)]
pub enum UserRoleInputType {
    Admin,
    Moderator,
    Publisher,
}

#[derive(GraphQLInputObject, Clone)]
pub struct NewUserInput {
    pub login: String,
    pub email: String,
    pub password: String,
    pub role: Option<UserRoleInputType>,
}

#[derive(Clone, GraphQLInputObject)]
pub struct EditUserInput {
    pub email: Option<String>,
    pub role: Option<UserRoleInputType>,
}
