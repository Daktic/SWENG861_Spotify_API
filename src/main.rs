use std::fmt::Debug;
use dotenv::dotenv;
use serde_json;
use actix_cors::Cors;
use serde::Serialize;

mod spotify;


use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result, HttpResponse, get, Responder, http};
use actix_files::NamedFile;
use std::path::PathBuf;
use crate::spotify::Artists;

#[derive(serde::Deserialize)]
struct ArtistQueryParams {
    artist_name: String,
}

#[derive(serde::Serialize)]
struct ArtistResponse {
    name: String,
}

#[get("/artist")]
async fn artist(query_params: web::Query<QueryParams>) -> impl Responder {
    let name = query_params.artist_name.as_str();
    let spotify_artist_query: spotify::QueryResult = spotify::query_builder(
        name,
        1,
    ).await;
    let spotify_artist: &spotify::Artists = spotify_artist_query.get_artist().unwrap();
    

    let json_response = serde_json::to_string(spotify_artist).unwrap();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response)
}
// #[get("/artist")]
// async fn artist(query_params: web::Query<ArtistQueryParams>) -> impl Responder {
//     let name = query_params.artist_name.to_string(); // Replace "name" with the actual artist name
//     println!("The name from the fetch request is {}", &name);
//     HttpResponse::Ok().body(format!("Name: {}", name))
// }

async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./web/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
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
            .service(artist)
            .service(web::resource("/").to(index))
            .service(actix_files::Files::new("/", "./web").show_files_listing())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


#[derive(serde::Deserialize)]
struct QueryParams {
    artist_name: String,
}