use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod routes;

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    dotenv().ok();

    HttpServer::new(|| {
        App::new()
            .configure(routes::events::configure)
            .configure(routes::users::configure)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

pub fn main() {
    let result = start_server();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
