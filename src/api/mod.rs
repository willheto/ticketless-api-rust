use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod routes;

#[actix_web::main]
pub async fn start_server() {
    dotenv().ok();

    let server = HttpServer::new(|| {
        App::new()
            .configure(routes::events::configure)
            .configure(routes::users::configure)
    })
    .bind("0.0.0.0:8080")
    .expect("Failed to bind to address")
    .run();

    if let Err(err) = server.await {
        println!("Error: {:?}", err);
    }
}
