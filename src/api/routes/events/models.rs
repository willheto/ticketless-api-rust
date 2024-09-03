use sea_orm::sqlx::types::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

pub enum EventResponseDataType {
    EventResponseArray(Vec<EventResponseData>),
    EventResponseObject(EventResponseData),
}

#[derive(Serialize)]
pub struct EventResponseData {
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

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct EventPostData {
    pub organization_id: Option<i32>,

    #[validate(length(min = 1, max = 255), required)]
    pub name: Option<String>,

    #[validate(length(min = 1), required)]
    pub location: Option<String>,

    #[validate(length(min = 1), required)]
    pub event_type: Option<String>,

    #[validate(length(min = 1), required)]
    pub date: Option<String>,

    #[validate(url)]
    pub image: Option<String>,

    #[validate(range(min = 0, max = 1), required)]
    pub is_public: Option<i8>,

    #[validate(length(min = 1), required)]
    pub status: Option<String>,

    #[validate(url)]
    pub ticket_sale_url: Option<String>,

    pub active_from: Option<String>,
    pub active_to: Option<String>,
}
