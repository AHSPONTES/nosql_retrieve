use mongodb::{
    bson::{doc, Bson},
    sync::Client,
};
use std::io;

fn main() -> mongodb::error::Result<()> {
    let client = Client::with_uri_str("mongodb://localhost:27017")?;
    let collection = client.database("customer_info").collection("people");

    println!("What person would ou like to look up? ");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => input = input.trim().to_string(),
        Err(error) => println!("Error: {}", error),
    }

    let results = collection.find(doc! {"name": input}, None)?;
    for result in results {
        match result {
            Ok(documents) => {
                if let Some(location) = documents.get("location").and_then(Bson::as_str) {
                    println!("location: {}", location);
                } else {
                    println!("no location listed");
                }
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
