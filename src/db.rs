use mongodb::{options::ClientOptions, sync::Client};
use std::error::Error;

pub struct DB {
   pub client: Client,
}

impl DB {
   pub fn init() -> Result<Self, mongodb::error::Error> {
       let mut client_options = ClientOptions::parse("mongodb://0.0.0.0:27017")?;
       client_options.app_name = Some("rust_mongo".to_string());

       Ok(Self {
           client: Client::with_options(client_options)?,
       })
   }
}