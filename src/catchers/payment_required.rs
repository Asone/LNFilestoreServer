use crate::db::models::api_payment::ApiPayment;
use crate::db::PostgresConn;
use crate::lnd::client::LndClient;
use rocket::response::content::Json;
use rocket::response::status;
use rocket::serde::Serialize;
use rocket::{catch, http::Status, Request};

/**
   Provides a catcher for default 402 Response.
   It allows us to populate the response body with custom data
*/
#[catch(402)]
pub async fn payment_required<'r>(
    _: Status,
    request: &'r Request<'_>,
) -> Result<status::Custom<Json<String>>, Status> {
    let pool = request.guard::<PostgresConn>().await.succeeded();
    let lnd_client_result = request.guard::<LndClient>().await.succeeded();

    match pool {
        Some(db) => {
            match lnd_client_result {
                Some(lnd_client) => {
                    let api_payment = ApiPayment::create_from_client(lnd_client, db, None).await;
                    match api_payment {
                        Ok(api_payment_result) => {
                            //    see https://api.rocket.rs/v0.5-rc/rocket/request/macro.local_cache.html

                            let payment_request = request.local_cache(|| api_payment_result);
                            let json_state = format!(
                                r#"{{\"payment\": \"{:?}\"}}"#,
                                payment_request.request.as_str()
                            );

                            // todo: use serde json and handle response with structs

                            Ok(status::Custom(Status::PaymentRequired, Json(json_state)))
                        }
                        Err(_) => Err(Status::InternalServerError),
                    }
                }
                None => Err(Status::InternalServerError),
            }
        }
        None => Err(Status::InternalServerError),
    }
}
