use mongodb::bson::{doc};
use std::error::Error;
use crate::{db::DB};

pub async fn update_data() -> Result<(), Box<dyn Error>> {
   let client = DB::init().await?;
   let client = client.client;

   // Get the 'movies' collection from the 'sample_mflix' database:
   let movies = client.database("rust_mongo").collection("movies");

   // Look up one document:
   let movie = movies
      .find_one(
         doc! {
               "title": "Parasite"
         },
         None,
      )
      .await?
      .expect("Missing 'Parasite' document.");
   println!("Movie: {}", movie);

   Ok(())
}