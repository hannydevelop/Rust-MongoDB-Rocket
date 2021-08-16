#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod get;
mod db;
mod update;

#[tokio::main]
async fn main() {
   get::run();
   update::update_data().await.ok();
}