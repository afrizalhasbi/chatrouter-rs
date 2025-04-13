use crate::utils::{Body, send};
use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use std::env;

// Use web::Json extractor for the body and return HttpResponse
async fn handle_request(body: web::Json<Body>) -> impl Responder {
    // Extract the inner Body struct
    let body_data = body.into_inner();

    match send(&body_data).await {
        Ok(response) => {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(response) // Send the raw JSON string back
        }
        Err(e) => {
            eprintln!("Error forwarding request to OpenRouter: {}", e);
            HttpResponse::InternalServerError()
                .body("Failed to communicate with downstream service.")
        }
    }
}

#[actix_web::main]
pub async fn server() -> std::io::Result<()> {
    env::var("OPENROUTER_API_KEY")
        .expect("Please set the OPENROUTER_API_KEY environment variable.");

    println!("Starting server on localhost:8080...");
    HttpServer::new(|| {
        App::new()
            // Ensure the route expects JSON
            .route("/", web::post().to(handle_request))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
