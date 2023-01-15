use std::time::Instant;

pub use crate::db::schema::media_payment;
use crate::lnd::invoice::LndInvoice;
use chrono::Duration;
use chrono::NaiveDateTime;
use chrono::Utc;
use diesel;
use diesel::prelude::*;
use diesel::PgConnection;
use uuid::Uuid;

use super::media::MediaModelType;

#[derive(Queryable, PartialEq, Associations, Debug, Clone)]
#[table_name = "media_payment"]
#[belongs_to(parent = Media, foreign_key = "media_uuid")]
pub struct MediaPayment {
    pub uuid: Uuid,
    pub request: String,
    pub state: Option<String>,
    pub hash: String,
    pub media_uuid: Uuid,
    pub expires_at: NaiveDateTime,
    pub valid_until: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable)]
#[table_name = "media_payment"]
pub struct NewMediaPayment {
    uuid: Uuid,
    hash: String,
    request: String,
    media_uuid: Uuid,
    expires_at: NaiveDateTime,
    valid_until: Option<NaiveDateTime>,
}

impl From<(LndInvoice, uuid::Uuid)> for NewMediaPayment {
    fn from(data: (LndInvoice, uuid::Uuid)) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            hash: data.0.r_hash,
            request: data.0.payment_request,
            media_uuid: data.1.to_owned(),
            expires_at: data.0.expires_at,
            valid_until: None,
        }
    }
}

impl From<(LndInvoice, uuid::Uuid, MediaModelType)> for NewMediaPayment {
    fn from(data: (LndInvoice, uuid::Uuid, MediaModelType)) -> Self {
        let valid_until = match data.2 {
            MediaModelType::Media(e) => match e.payment_duration {
                Some(duration) => Some(
                    Utc::now()
                        .checked_add_signed(Duration::minutes(duration.into()))
                        .unwrap()
                        .naive_utc(),
                ),
                None => None,
            },
            MediaModelType::NewMedia(e) => match e.payment_duration {
                Some(duration) => Some(
                    Utc::now()
                        .checked_add_signed(Duration::minutes(duration.into()))
                        .unwrap()
                        .naive_utc(),
                ),
                None => None,
            },
        };

        Self {
            uuid: Uuid::new_v4(),
            hash: data.0.r_hash,
            request: data.0.payment_request,
            media_uuid: data.1.to_owned(),
            expires_at: data.0.expires_at,
            valid_until,
        }
    }
}

impl MediaPayment {
    pub fn find_one_by_request(
        payment_request: String,
        connection: &PgConnection,
    ) -> QueryResult<Option<MediaPayment>> {
        use crate::db::schema::media_payment::dsl::*;
        media_payment
            .filter(request.eq(payment_request))
            .first::<MediaPayment>(connection)
            .optional()
    }

    pub fn create(
        new_payment: NewMediaPayment,
        connection: &PgConnection,
    ) -> QueryResult<MediaPayment> {
        use crate::db::schema::media_payment::dsl::*;

        diesel::insert_into::<media_payment>(media_payment)
            .values(&new_payment)
            .get_result(connection)
    }

    pub fn is_expired(&self) -> bool {
        match &self.valid_until {
            Some(valid_until) => valid_until < &Utc::now().naive_utc(),
            None => false,
        }
    }
}
