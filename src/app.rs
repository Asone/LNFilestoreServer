use std::collections::HashMap;

use crate::{
    graphql::{context::GQLContext, mutation::Mutation, query::Query},
    lnd::client::LndClient,
};
use lightning_invoice::Sha256;
use rocket::{response::content, State, Data, data::ToByteUnit, form::Form, fs::TempFile, http::ContentType};
use rocket_multipart_form_data::{MultipartFormDataOptions, MultipartFormData, MultipartFormDataField, FileField};
pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<GQLContext>>;

use crate::db::PostgresConn;
use crate::requests::header::PaymentRequestHeader;
use juniper::{EmptySubscription, RootNode};
use juniper_rocket::GraphQLResponse;
use serde_json::{Value};

#[rocket::get("/")]
pub fn graphiql() -> content::Html<String> {
    juniper_rocket::graphiql_source("/graphql", None)
}

/*
    This is a void handler that will return a 200 empty response
    for browsers that intends to check pre-flight for CORS rules.
*/
#[rocket::options("/graphql")]
pub async fn options_handler() {}

/**
   Calls the GraphQL API from a HTTP GET Request.
   It does nothing special but a paywall mechanism through
   a payment_request param could be implemented later.
*/
#[rocket::get("/graphql?<request>")]
pub async fn get_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
) -> GraphQLResponse {
    request
        .execute(&*schema, &GQLContext { pool: db, lnd: lnd })
        .await
}

/**
   Calls the API with a query specific paywall protected mechanism.
*/
#[rocket::post("/graphql", data = "<request>")]
pub async fn post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
) -> GraphQLResponse {
    request
        .execute(&*schema, &GQLContext { pool: db, lnd: lnd })
        .await
}

/**
   Calls the API through an API-scoped paywall
*/
#[rocket::post("/payable", data = "<request>")]
pub async fn payable_post_graphql_handler(
    request: juniper_rocket::GraphQLRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient,
    _payment_request: PaymentRequestHeader,
) -> GraphQLResponse {
    request
        .execute(&*schema, &GQLContext { pool: db, lnd: lnd })
        .await
}
#[rocket::post("/upload", data = "<request>")]
pub async fn upload<'r>(
    request: crate::graphql::multipart::upload_request::GraphQLUploadRequest,
    schema: &State<Schema>,
    db: PostgresConn,
    lnd: LndClient) -> GraphQLResponse {
    

    request.execute(&*schema, &GQLContext { pool: db, lnd: lnd })
        .await
}
// #[derive(Debug, FromForm)]
// pub struct UploadedFile {
//     operations: String,
//     map: String
// }

//   Uploads a file to the server.
// #[rocket::post("/upload", data = "<data>")]
// pub async fn upload<'r>(content_type: &ContentType, data: Data<'_>) -> &'static str {
        // MultipartFormData expects us to provide a mapping of the multipart form data
        // in order to be able to parse the boundaries from the request body.
     
        // As we do not know how many files are uploaded initially in the request nor their names
        // what we'll do here is to to first parse the Data aknowledging the mapping object for files
        // and parse the request content a second time once the mapping applied to the Content Form Data.
     
//     let mut options = MultipartFormDataOptions::new();
//     // println!("{}",data.as_str());
//     options.allowed_fields.push(MultipartFormDataField::text("map"));
//     options.allowed_fields.push(MultipartFormDataField::text("operations"));
    
//     let multipart_form_data = MultipartFormData::parse(content_type, data, options).await;
    
    
//     match multipart_form_data {
//         Ok(mfp) => {
        
//             let mappings = mfp.texts.get("map").unwrap();
//             // let files = mfp.files.get("file").unwrap(); // .get(&"file".to_string());
           
//         //    for (i,f) in mapping.iter().enumerate() {
                
//                 // println!("index key : {}; data: {}",i,&f.text);

                
// //                 for item in &v {
// //                     println!("{:?}\n", item);
// // }           
// //                 }
//             for mapping in mappings {
//                 let d = mapping.text.as_str();
//                 println!("{}",d);
//                 let hmap: HashMap<&str, Value> = serde_json::from_str(d).unwrap();
//                 let mut options2 = MultipartFormDataOptions::new();
//                 // Gets the files mapping from request
//                 for (k,v) in hmap.iter() {
//                     options2.allowed_fields.push(MultipartFormDataField::file(k));
//                 }
//                 // // println!("{}",data);
//                 // let multipart_form_data_2 = MultipartFormData::parse(content_type, data, options2).await;
//                 // match multipart_form_data_2 {
//                 //     Ok(mfp2) => {
//                 //         let t = mfp2;
//                 //     },
//                 //     Err(_) => {}
//                 // }
//             }

//             // for f in files {
//             //     let t = f.content_type.as_ref().unwrap();
//             //     println!("{}",t);
//             // }
//             // if let Some(file) = file {
//             //     match file {
//             //         FileField::Single(f) => {
//             //         let _content_type = &file.content_type;
//             //         let _file_name = &file.file_name;
//             //         let _path = &file.path;
//             //         }
//             //     }
//             // }
//         },
//         Err(_) => {}
//     };
//     // // println!("{:?}", w);
//     // let byt = file.open(10.megabytes());
//     // let o = byt.into_bytes().await.unwrap().to_vec();
//     // // println!("{:?}", o);
//     // let we = String::from_utf8(o.clone()).unwrap();
//     // println!("utf8-> {}", we);
//     // let hasher = Sha256::new().chain(&o);
//     // let result = hasher.finalize();
//     // println!("hash sha256: {:x}", &result);
//     // Ok(())
//     "ok"
// }