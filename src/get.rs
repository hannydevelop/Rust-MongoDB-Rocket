use rocket::{get, routes};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request, State};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use mongodb::bson::{doc};
use std::error::Error;
use crate::{db::DB};
use rocket::response::status::BadRequest;
#[derive(Serialize, Deserialize, Debug)]
pub struct Movie {
   pub title: String,
   pub year: String,
   pub plot: String,
}


#[get("/<movie_title>")]
pub fn get_data(movie_title: String) -> Result<Json<Option<Movie>>, BadRequest<mongodb::error::Error>> {
   let db = DB::init().map_err(|e| BadRequest(Some(e)))?;
   let client = db.client;

    // Get the 'movies' collection from the 'sample_mflix' database:
    let movies = client.database("rust_mongo").collection_with_type::<Movie>("movies");
    let movie = movies
    .find_one(
       doc! {
             "title": movie_title
       },
       None,
    )
    .expect("Missing 'Parasite' document.");

    format!("Movie: {:?}", movie);

   Ok(Json(movie))
}

