use actix_web::{get, post, web, HttpResponse, Result};
use crate::entity::event::{self, Entity as Event};
use sea_orm::{
    sqlx::types::chrono::{DateTime, Utc}, Database, DatabaseConnection, DbErr, EntityTrait, Set
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use validator::Validate;

const CRUD_RESPONSE_ARRAY: &str = "events";
const CRUD_RESPONSE_OBJECT: &str = "event";

#[derive(Serialize)]
pub struct EventResponse {
    pub event_id: i32,
    pub organization_id: Option<i32>,
    pub name: String,
    pub location: String,
    pub event_type: String,
    pub date: String,
    pub image: String,
    pub is_public: i8,
    pub status: String,
    pub ticket_sale_url: String,
    pub active_from: String,
    pub active_to: String,
    pub trending_score: i32,
    pub ticket_max_price: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

async fn get_database_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url).await
}

#[get("/events")]
async fn get_all_events() -> Result<HttpResponse> {
    match get_database_connection().await {
        Ok(db) => {
            let result = Event::find().all(&db).await;

            match result {
                Ok(result) => {
                    let events: Vec<EventResponse> = result
                        .into_iter()
                        .map(|event| EventResponse {
                            event_id: event.event_id,
                            organization_id: event.organization_id,
                            name: event.name,
                            location: event.location,
                            event_type: event.event_type,
                            date: event.date,
                            image: event.image,
                            is_public: event.is_public,
                            status: event.status,
                            ticket_sale_url: event.ticket_sale_url,
                            active_from: event.active_from,
                            active_to: event.active_to,
                            trending_score: event.trending_score,
                            ticket_max_price: event.ticket_max_price,
                            created_at: event.created_at,
                            updated_at: event.updated_at,
                        })
                        .collect();

                    let json_response =
                        create_response_data(ResponseDataType::EventResponseArray(events));
                    json_response
                }
                Err(err) => create_internal_error_response(err),
            }
        }
        Err(err) => create_internal_error_response(err),
    }
}

#[get("/events/{id}")]
async fn get_single_event(path: web::Path<(i32,)>) -> Result<HttpResponse> {
    match get_database_connection().await {
        Ok(db) => {
            let (id,) = path.into_inner();
            let result = Event::find_by_id(id).one(&db).await;

            match result {
                Ok(Some(event)) => {
                    let event_response = EventResponse {
                        event_id: event.event_id,
                        organization_id: event.organization_id,
                        name: event.name,
                        location: event.location,
                        event_type: event.event_type,
                        date: event.date,
                        image: event.image,
                        is_public: event.is_public,
                        status: event.status,
                        ticket_sale_url: event.ticket_sale_url,
                        active_from: event.active_from,
                        active_to: event.active_to,
                        trending_score: event.trending_score,
                        ticket_max_price: event.ticket_max_price,
                        created_at: event.created_at,
                        updated_at: event.updated_at,
                    };

                    let json_response =
                        create_response_data(ResponseDataType::EventResponseObject(event_response));
                    json_response
                }
                Ok(None) => Ok(HttpResponse::NotFound().finish()),
                Err(err) => create_internal_error_response(err),
            }
        }
        Err(err) => create_internal_error_response(err),
    }
}

#[derive(Debug, Validate, Deserialize, Serialize)]
struct EventData {
    organization_id: Option<i32>,

    #[validate(length(min = 1, max = 255), required)]
    name: Option<String>,

    #[validate(length(min = 1), required)]
    location: Option<String>,

    #[validate(length(min = 1), required)]
    event_type: Option<String>,

    #[validate(length(min = 1), required)]
    date: Option<String>,

    #[validate(url)]
    image: Option<String>,

    #[validate(range(min = 0, max = 1), required)]
    is_public: Option<i8>,

    #[validate(length(min = 1), required)]
    status: Option<String>,

    #[validate(url)]
    ticket_sale_url: Option<String>,

    active_from: Option<String>,
    active_to: Option<String>,
}

#[post("/events")]
async fn create_event(payload: web::Json<EventData>) -> Result<HttpResponse> {
    let event_data = payload.into_inner();

    if let Err(err) = event_data.validate() {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Validation error",
            "details": err
        })));
    }

    match get_database_connection().await {
        Ok(db) => {
            let event = event::ActiveModel {
                organization_id: Set(Some(event_data.organization_id.unwrap_or_default())),
                name: Set(event_data.name.unwrap_or_default()),
                location: Set(event_data.location.unwrap_or_default()),
                event_type: Set(event_data.event_type.unwrap_or_default()),
                date: Set(event_data.date.unwrap_or_default()),
                image: Set(event_data.image.unwrap_or_default()),
                is_public: Set(event_data.is_public.unwrap_or_default()),
                status: Set(event_data.status.unwrap_or_default()),
                ticket_sale_url: Set(event_data.ticket_sale_url.unwrap_or_default()),
                active_from: Set(event_data.active_from.unwrap_or_default()),
                active_to: Set(event_data.active_to.unwrap_or_default()),
                ..Default::default()
            };

            let response = event::Entity::insert(event).exec(&db).await;

            match response {
                Ok(_) => Ok(HttpResponse::Created()
                    .content_type("application/json")
                    .finish()),
                Err(err) => create_internal_error_response(err),
            }
        }
        Err(err) => create_internal_error_response(err),
    }
}

fn create_internal_error_response(error: DbErr) -> Result<HttpResponse> {
    let json_response = serde_json::json!({
        "error": error.to_string()
    });

    Ok(HttpResponse::InternalServerError()
        .content_type("application/json")
        .body(json_response.to_string()))
}

pub enum ResponseDataType {
    EventResponseArray(Vec<EventResponse>),
    EventResponseObject(EventResponse),
}

/**
 * Map response data according to data type.
 * If data is an object, return an object, else, return array of objects
 */
pub fn create_response_data(data: ResponseDataType) -> Result<HttpResponse, actix_web::Error> {
    let json_response = match data {
        ResponseDataType::EventResponseArray(events) => {
            json!({ CRUD_RESPONSE_ARRAY: events })
        }
        ResponseDataType::EventResponseObject(event) => {
            json!({ CRUD_RESPONSE_OBJECT: event })
        }
    };

    Ok(HttpResponse::Ok().json(json_response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_events);
    cfg.service(get_single_event);
    cfg.service(create_event);
}
