use std::fmt::Debug;
use dotenv::dotenv;
use serde_json;
use actix_cors::Cors;

mod spotify;


use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result, HttpResponse, get, Responder, http};
use actix_files::NamedFile;
use std::path::PathBuf;


#[get("/artist")]
async fn artist() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./web/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)

    //let client = reqwest::Client::new();

    // //load in the dotenv
    // dotenv().ok();
    // let client_secret = std::env::var("CLIENT_SECRET").unwrap();
    // let client_id = std::env::var("CLIENT_ID").unwrap();
    //
    // let access_credentials = spotify::get_auth_code(
    //     &client,
    //     &client_id,
    //     &client_secret,
    // ).await;
    //
    //
    // let artist = spotify::query_builder("test", &client, &access_credentials.access_token, 1);
    // let mut response: String = String::from("Hello, World!");
    // match artist.await {
    //     spotify::QueryResult::Artists(spotify_artist) => {
    //         // Serialize spotify_artist into a JSON string
    //         if let Ok(json_str) = serde_json::to_string(&spotify_artist) {
    //             // Update the response value with the JSON string
    //             response = json_str;
    //         }
    //     }
    //     spotify::QueryResult::Tracks(_) => {
    //         // Handle the case when the type_of_search is 2 (tracks)
    //         println!("Expected artist details, but received track details.");
    //     }
    //     spotify::QueryResult::Error => {
    //         // Handle the error case or empty variant
    //         println!("Not a proper search param.");
    //     }
    // }
    //
    // response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");


    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
            .service(actix_files::Files::new("/", "./web").show_files_listing())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(artist)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


