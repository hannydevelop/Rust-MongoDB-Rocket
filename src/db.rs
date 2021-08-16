use mongodb::{options::ClientOptions, Client};
use std::error::Error;

pub struct DB {
   pub client: Client,
}

impl DB {
   pub async fn init() -> Result<Self, Box<dyn Error>> {
       let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
       client_options.app_name = Some("rust_mongo".to_string());

       Ok(Self {
           client: Client::with_options(client_options)?,
       })
   }
}