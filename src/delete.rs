#[tokio]
async fn delete() -> Result<(), Box<dyn Error>> {
   // Load the MongoDB connection string from an environment variable:
   let client_uri = "mongodb://localhost:27017";
   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await?;
   let client = mongodb::Client::with_options(options)?;

   // Print the databases in our MongoDB cluster:
   println!("Databases:");
   for name in client.list_database_names(None, None).await? {
      println!("- {}", name);
   }

   // Get the 'movies' collection from the 'sample_mflix' database:
   let movies = client.database("sample_mflix").collection("movies");

   // Delete all documents for movies called "Parasite":
   let delete_result = movies
      .delete_many(
         doc! {
            "title": "Parasite"
         },
         None,
      )
      .await?;
   println!("Deleted {} documents", delete_result.deleted_count);

   Ok(())
}
