use rocket::response::content;

#[rocket::get("/")]
pub fn graphiql() -> content::RawHtml<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

#[rocket::get("/")]
pub fn static_index() -> content::RawHtml<String> {
    let content = r#"
    <html>
        <head>
        </head>
        <body>
            LN File Store
        </body>
    </html>
    "#;
    content::RawHtml(content.to_string())
}
