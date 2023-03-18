use mongodb::options::ClientOptions;
use mongodb::Database;

pub async fn init(db_name: &str) -> Database {
    let MONGO_URL = std::env::var("MONGO_URL").unwrap_or("mongodb://localhost:27017".to_string());
    let mut options = ClientOptions::parse(&MONGO_URL)
        .await
        .unwrap();
    options.app_name = Some(db_name.to_string());
    let client = mongodb::Client::with_options(options).unwrap();
    client.database(db_name)
}

#[cfg(test)]
mod tests {
    use crate::adapters::mongo::database::init;

    #[actix_web::test]
    async fn test_init() {
        let db = init("test").await;
        assert_eq!(db.name(), "test");
    }
}
