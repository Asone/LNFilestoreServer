use crate::{
    db::models::{
        media::Media,
        media_payment::{MediaPayment, NewMediaPayment},
    },
    graphql::context::GQLContext,
    lnd::invoice::{InvoiceParams, InvoiceUtils},
};
use base64;
use chrono::NaiveDateTime;
use infer::Infer;
use juniper::Value;
use juniper::{FieldError, FieldResult};
use juniper_relay_connection::RelayConnectionNode;
use std::fs::File;

/// To be deleted
// #[derive(Clone, Serialize, Deserialize)]
// pub struct MediaPreviewType {
//     pub uuid: uuid::Uuid,
//     pub description: Option<String>,
//     pub price: i32,
//     pub published: bool,
//     pub created_at: NaiveDateTime,
// }

#[derive(Clone)]
pub struct MediaType {
    pub uuid: uuid::Uuid,
    pub title: String,
    pub description: Option<String>,
    pub price: i32,
    pub published: bool,
    pub created_at: NaiveDateTime,
    absolute_path: String,
}

impl From<Media> for MediaType {
    fn from(item: Media) -> Self {
        Self {
            uuid: item.uuid,
            title: item.title,
            description: item.description,
            price: item.price,
            published: item.published,
            created_at: item.created_at,
            absolute_path: item.absolute_path,
        }
    }
}

impl From<(Media, String)> for MediaType {
    fn from(item: (Media, String)) -> Self {
        let media = item.0;

        Self {
            uuid: media.uuid,
            title: media.title,
            description: media.description,
            price: media.price,
            published: media.published,
            created_at: media.created_at,
            absolute_path: media.absolute_path,
        }
    }
}

impl MediaType {
    // This method builds a json object with payment requirements details.
    // The json object will be provided as an error's extension of graphql response
    async fn _generate_invoiced_error(&self, context: &GQLContext, message: &str) -> FieldError {
        let connection = context.get_db_connection();
        let params = InvoiceParams::new(Some(self.price.into()), None, None);
        let invoice =
            InvoiceUtils::generate_invoice(context.get_lnd_client().clone(), params).await;
        let uuid = self.uuid.clone();
        let media_payment = connection
            .run(move |c| MediaPayment::create(NewMediaPayment::from((invoice, uuid)), c))
            .await;

        match media_payment {
            Ok(media_payment) => {
                let request = media_payment.request.as_str();
                let hash = media_payment.hash.as_str();

                FieldError::new(
                    format!("{} Use provided payment request.", message),
                    graphql_value!({"state": "open",
                         "payment_request": request, 
                         "r_hash": hash}),
                )
            }
            Err(_) => FieldError::new(
                format!(
                    "{}. An error happened while trying to generate payment request",
                    message
                ),
                Value::null(),
            ),
        }
    }
}

#[graphql_object(
    name = "Media", 
    description = "Full Media output type"
    context = GQLContext
)]
impl MediaType {
    #[graphql(description = "The media internal id")]
    fn uuid(&self) -> uuid::Uuid {
        self.uuid
    }

    #[graphql(description = "Media's title")]
    fn title(&self) -> &String {
        &self.title
    }

    #[graphql(description = "Description of media")]
    fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    #[graphql(description = "Price of media access in satoshis. If free is 0")]
    fn price(&self) -> i32 {
        self.price
    }

    #[graphql(description = "Creation date of media")]
    fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }

    #[graphql(description = "the public URL to a media")]
    fn public_url<'a>(&self, _context: &'a GQLContext) -> FieldResult<String> {
        let uri = format!("/file/{}", &self.uuid);
        Ok(uri)
    }
    #[graphql(description = "The file type")]
    fn file_type(&self) -> Option<&str> {
        let info = Infer::new();
        let kind = info.get_from_path(&self.absolute_path);

        match kind {
            Ok(result) => match result {
                Some(t) => return Some(t.extension()),
                None => return None,
            },
            Err(_) => return None,
        }
    }

    #[graphql(description = "The file size")]
    fn file_size(&self) -> Option<i32> {
        let file = File::open(&self.absolute_path);

        match file {
            Ok(file) => {
                let size = file.metadata().unwrap().len().to_string();
                Some(size.parse::<i32>().unwrap_or(0))
            }
            Err(_) => None,
        }
    }
}

/// Implements relay connection for Medias
/// It allows using obscure cursors for pagination
impl RelayConnectionNode for MediaType {
    type Cursor = String;

    fn cursor(&self) -> Self::Cursor {
        let cursor = format!("media:{}", self.uuid);
        base64::encode(cursor)
    }

    fn connection_type_name() -> &'static str {
        "MediaConnection"
    }

    fn edge_type_name() -> &'static str {
        "MediaConnectionEdge"
    }
}
