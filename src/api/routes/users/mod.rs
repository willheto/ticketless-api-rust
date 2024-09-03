pub mod models;

use crate::entity::user::Entity as User;
use actix_web::{get, web, HttpResponse, Result};
use models::{UserResponseData, UserResponseDataType};
use sea_orm::{Database, DatabaseConnection, DbErr, EntityTrait};
use serde_json::json;
use std::env;

const CRUD_RESPONSE_ARRAY: &str = "users";
const CRUD_RESPONSE_OBJECT: &str = "user";

async fn get_database_connection() -> Result<DatabaseConnection, DbErr> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Database::connect(&database_url).await
}

#[get("/users")]
async fn get_all_users() -> Result<HttpResponse> {
    match get_database_connection().await {
        Ok(db) => {
            let result = User::find().all(&db).await;
            match result {
                Ok(result) => {
                    let users: Vec<UserResponseData> = result
                        .into_iter()
                        .map(|user| UserResponseData {
                            user_id: user.user_id,
                            organization_id: user.organization_id,
                            first_name: user.first_name,
                            last_name: user.last_name,
                            email: user.email,
                            phone_number: user.phone_number,
                            city: user.city,
                            user_type: user.user_type,
                            profile_picture: user.profile_picture,
                            language: user.language,
                            created_at: user.created_at,
                            updated_at: user.updated_at,
                        })
                        .collect();

                    let json_response =
                        create_response_data(UserResponseDataType::UserResponseArray(users));
                    json_response
                }
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

/**
 * Map response data according to data type.
 * If data is an object, return an object, else, return array of objects
 */
pub fn create_response_data(data: UserResponseDataType) -> Result<HttpResponse, actix_web::Error> {
    let json_response = match data {
        UserResponseDataType::UserResponseArray(events) => {
            json!({ CRUD_RESPONSE_ARRAY: events })
        }
        UserResponseDataType::UserResponseObject(event) => {
            json!({ CRUD_RESPONSE_OBJECT: event })
        }
    };

    Ok(HttpResponse::Ok().json(json_response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
}
