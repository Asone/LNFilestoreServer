use rocket::response::content;

#[rocket::get("/")]
pub fn graphiql() -> content::RawHtml<String> {
    
    juniper_rocket::graphiql_source("/graphql", None)
}
