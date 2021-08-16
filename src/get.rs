use rocket::{get, routes};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request, State};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use mongodb::bson::{doc};
use std::error::Error;
use crate::{db::DB};

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
   pub title: String,
   pub year: String,
   pub plot: String,
}

#[get("/<movie>")]
pub async fn get_data(movie: String) -> Result<Json<Data>, Box<dyn Error>> {
   let client = DB::init().await?;
   let client = client.client;

   let data = Data {
      title: "Parasite".to_string(),
      year: "2020".to_string(),
      plot: "me bumble me".to_string(),
   };

    // Get the 'movies' collection from the 'sample_mflix' database:
    let movies = client.database("rust_mongo").collection("movies");
    let movie = movies
    .find_one(
       doc! {
             "title": data.title.clone()
       },
       None,
    )
    .await?
    .expect("Missing 'Parasite' document.");
    format!("Movie: {}", movie);


   Ok(Json(data))
}

pub fn run() -> rocket::Rocket{
   rocket::ignite()
   .mount("/", routes![get_data])
}

