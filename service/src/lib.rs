use actix_web::{get, App, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use sea_orm::{Database, EntityTrait};
use std::env;

use entity::events::Entity as Event;

#[get("/events")]
async fn get_all_events() -> Result<HttpResponse> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let db = Database::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Example query (replace with your actual query)
    let result = Event::find().all(&db).await.expect("Query failed");

    Ok(HttpResponse::Ok().body(format!("events: {:?}", result)))
}

#[actix_web::main]
async fn start() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_all_events))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
