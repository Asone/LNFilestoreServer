#[cfg(test)]
mod tests {
    use diesel::Connection;
    use lazy_static::lazy_static;

    use crate::tests::database::tests::TestDb;

    lazy_static! {
        static ref TEST_DB: TestDb = TestDb::new();
    }

    #[test]
    pub fn test_media_insert() -> () {
        use crate::db::models::media::*;
        use dotenv::dotenv;

        dotenv().ok();

        // let test_db = TestDb::new();

        let conn = &TEST_DB.conn();

        let new_media = NewMedia {
            uuid: uuid::Uuid::new_v4(),
            title: "test".to_string(),
            description: Some("description test".to_string()),
            absolute_path: "nowhere".to_string(),
            published: true,
            price: 100,
        };

        let result = &conn.test_transaction(|| Media::create(new_media, &conn));

        assert_eq!("test".to_string(), result.title);
    }
}
