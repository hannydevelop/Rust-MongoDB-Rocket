#![feature(proc_macro_hygiene, decl_macro)]

use crate::get::Movie;

#[macro_use]
extern crate rocket;

mod get;
mod db;
mod update;

fn main() {
   // first insert the new movie
   update::update_data(Movie {
      title: "Parasite".to_string(),
      year: "2020".to_string(),
      plot: "me bumble me".to_string(),
   }).expect("Should insert Movie");

   // can't run anything after this call
   rocket::ignite()
   .mount("/", routes![get::get_data]).launch();

   
}