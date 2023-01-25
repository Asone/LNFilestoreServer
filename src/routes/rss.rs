use crate::db::{models::media::Media, PostgresConn};
use rocket::http::ContentType;
use rocket::response::{content, Response};
use rss::{Channel, ChannelBuilder, Item, ItemBuilder};
use std::io::Cursor;

/// A route to retrieve files behind the paywall.
#[rocket::get("/rss")]
pub async fn get_rss(db: PostgresConn) -> content::RawXml<String> {
    let results = db.run(move |c| Media::find_all_published(c)).await;
    let items: Vec<Item> = results
        .iter()
        .map(|media| {
            let mut item = Item::default();
            item.set_title(media.title.clone());
            item.set_description(media.description.clone());
            // .set_description(media.description)
            // .link(item.link.as_str())
            item
        })
        .collect();

    let channel = ChannelBuilder::default()
        .title("LNFileStore RSS Feed".to_string())
        .description("This RSS feed provides the list of available medias".to_string())
        // .link("https://example.com".to_string())
        .items(items)
        .build()
        .to_string();

    content::RawXml(channel)
}
