use sea_orm::sqlx::types::chrono::{DateTime, Utc};
use serde::Serialize;

pub enum UserResponseDataType {
    UserResponseArray(Vec<UserResponseData>),
    UserResponseObject(UserResponseData),
}

#[derive(Serialize)]
pub struct UserResponseData {
    pub user_id: i32,
    pub organization_id: Option<i32>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_number: String,
    pub city: String,
    pub user_type: String,
    pub profile_picture: String,
    pub language: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
