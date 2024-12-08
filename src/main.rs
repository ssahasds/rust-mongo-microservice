
use std::env;
use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    extract::Json, Router,
};
use bson::Document;
use serde::Serialize;
use std::error::Error;
use mongodb::options::ResolverConfig;
use mongodb::options::ClientOptions;
use mongodb::Client;
use serde::Deserialize;
use chrono::Utc;
use mongodb::bson::doc;

#[derive(Deserialize,Serialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Handler for /create-user
async fn create_user(Json(payload): Json<User>) -> impl IntoResponse {
    
    let id = payload.id;
    let name = payload.name;
    let email = payload.email;
    let dt = Utc::now().date_naive().to_string();

    let new_user = doc! {
        "id": id.to_string(),
        "name": name.to_string(),
        "email": email.to_string(),
        "createdTS": dt,
     };
     
    let _ = insert_one(new_user).await;

    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from(format!("id = {}, name = {}, age = {}", id, name, email)))
        .unwrap()
        
}

async fn connect() -> Result<Client, Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
       ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
          .await?;
    let client = Client::with_options(options)?;
    Ok(client)
 }

#[tokio::main]
async fn main() {
    
    // Define Routes
    let app = Router::new()
        .route("/create-user", post(create_user));

    println!("Running on http://localhost:3000");
    // Start Server
    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


async fn insert_one(user_doc: Document) -> Result<(), Box<dyn Error>> {
    let user = connect().await.unwrap().database("rust-example").collection("users");
    let insert_result = user.insert_one(user_doc.clone(), None).await?;
    println!("New document ID: {}", insert_result.inserted_id);
    Ok(())
}