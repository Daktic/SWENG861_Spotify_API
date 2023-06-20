use std::fmt::Debug;
use dotenv::dotenv;
use serde_json;
use actix_cors::Cors;
use serde::Serialize;
use std::time::Instant;

mod spotify;


use actix_web::{middleware, web, App, HttpRequest, HttpServer, Result, HttpResponse, get, Responder, http};
use actix_files::NamedFile;
use std::path::PathBuf;
use actix_web::http::StatusCode;
use crate::spotify::Artists;

#[derive(serde::Deserialize)]
struct ArtistQueryParams {
    artist_name: String,
}

#[derive(serde::Deserialize)]
struct SongQueryParams {
    song_name: String,
}

#[derive(serde::Serialize)]
struct ArtistResponse {
    name: String,
}

// The artist path. reads the query params and returns a data json with the artist.
#[get("/artist")]
async fn artist(query_params: web::Query<ArtistQueryParams>) -> impl Responder {
    let name = query_params.artist_name.as_str();
    log::info!("Searching {} in artists", &name);
    let start = Instant::now();
    let spotify_artist_query: spotify::QueryResult = spotify::query_builder(name, 1).await;

    // records how long each query took to the server.
    let elapsed = start.elapsed();
    log::info!("found {} results in {:?}", spotify_artist_query.get_artist().map_or(0, |artists| artists.artists.len()), elapsed);

    let json_response = match spotify_artist_query {
        spotify::QueryResult::QueryArtists(artists) => serde_json::to_string(&artists).unwrap(),
        spotify::QueryResult::SpotifyError(error) => serde_json::to_string(&error).unwrap(),
        _ => String::from(""),
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response)
}

// The song path. reads the query params and returns a data json with the songs.
#[get("/song")]
async fn song(query_params: web::Query<SongQueryParams>) -> impl Responder {
    let name = query_params.song_name.as_str();
    log::info!("Searching {} in songs",&name);
    let start = Instant::now();
    let spotify_song_query: spotify::QueryResult = spotify::query_builder(
        name,
        2,
    ).await;

    // records how long each query took to the server.
    let elapsed = start.elapsed();
    log::info!("found {} results in {:?}", spotify_song_query.get_song().map_or(0, |songs| songs.songs.len()), elapsed);

    let json_response = match spotify_song_query {
        spotify::QueryResult::Tracks(songs) => serde_json::to_string(&songs).unwrap(),
        spotify::QueryResult::SpotifyError(error) => serde_json::to_string(&error).unwrap(),
        _ => String::from(""),
    };

    HttpResponse::Ok()
        .content_type("application/json")
        .body(json_response)
}

// serves the index page.
async fn index(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = "./web/index.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}

// The main function that attaches the methods to the web server.
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
            .service(song)
            .service(web::resource("/").to(index))
            .service(actix_files::Files::new("/", "./web").show_files_listing())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}