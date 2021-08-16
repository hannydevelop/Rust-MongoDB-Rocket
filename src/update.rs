use mongodb::bson::{doc};
use std::error::Error;
use crate::{db::DB, get::Movie};

pub fn update_data(movie: Movie) -> Result<(), mongodb::error::Error> {
   let client = DB::init()?;
   let client = client.client;

   // Get the 'movies' collection from the 'sample_mflix' database:
   let movies = client.database("rust_mongo").collection_with_type::<Movie>("movies");

   // Insert one Movie
   let insert_result = movies
      .insert_one(movie, None)?;
   println!("Movie mongodb _id: {}", insert_result.inserted_id);

   Ok(())
}