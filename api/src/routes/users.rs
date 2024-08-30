use actix_web::{web, get, HttpResponse, Result};

#[get("/users")]
async fn get_all_users() -> Result<HttpResponse> {
    
    Ok(HttpResponse::Ok().body("List of users"))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
}
