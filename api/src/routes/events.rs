use actix_web::{get, web, HttpResponse, Result};
use entity::events::Entity as Event;
use sea_orm::{Database, EntityTrait};
use serde::Serialize;
use std::env;

const MODEL_NAME: &str = "events";

#[derive(Serialize)]
pub struct EventResponse {
    pub event_id: i32,
    pub name: String,
    pub description: String,
}

#[get("/events")]
async fn get_all_events() -> Result<HttpResponse> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&database_url).await;

    match db {
        Ok(db) => {
            let result = Event::find().all(&db).await;

            match result {
                Ok(result) => {
                    let events: Vec<EventResponse> = result
                        .into_iter()
                        .map(|event| EventResponse {
                            event_id: event.event_id,
                            name: event.name,
                            description: event.description,
                        })
                        .collect();

                    let json_response = create_response_data(events);
                    json_response
                }
                Err(_) => Ok(HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .finish()),
            }
        }
        Err(_) => Ok(HttpResponse::InternalServerError()
            .content_type("application/json")
            .finish()),
    }
}

pub fn create_response_data(data: Vec<EventResponse>) -> Result<HttpResponse> {
    let json_response = serde_json::json!({
        MODEL_NAME: data
    });

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response.to_string()))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_events);
}
