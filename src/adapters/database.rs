use mongodb::Database;
use mongodb::options::ClientOptions;

pub async fn init(db_name: &str) -> Database {
    let mut options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    options.app_name = Some(db_name.to_string());
    let client = mongodb::Client::with_options(options).unwrap();
    client.database(db_name)
}