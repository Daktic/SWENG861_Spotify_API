use std::fmt::Debug;
use dotenv::dotenv;
use serde_json;

mod spotify;


use actix_web::{middleware, web, App, HttpRequest, HttpServer};


async fn index(req: HttpRequest) -> String {
    println!("REQ: {req:?}");
    let client = reqwest::Client::new();

    //load in the dotenv
    dotenv().ok();
    let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    let client_id = std::env::var("CLIENT_ID").unwrap();

    let access_credentials = spotify::get_auth_code(
        &client,
        &client_id,
        &client_secret,
    ).await;


    let artist = spotify::query_builder("test", &client, &access_credentials.access_token, 1);
    let mut response: String = String::from("Hello, World!");
    match artist.await {
        spotify::QueryResult::Artists(spotify_artist) => {
            // Serialize spotify_artist into a JSON string
            if let Ok(json_str) = serde_json::to_string(&spotify_artist) {
                // Update the response value with the JSON string
                response = json_str;
            }
        }
        spotify::QueryResult::Tracks(_) => {
            // Handle the case when the type_of_search is 2 (tracks)
            println!("Expected artist details, but received track details.");
        }
        spotify::QueryResult::Error => {
            // Handle the error case or empty variant
            println!("Not a proper search param.");
        }
    }

    response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

