use dotenv::dotenv;

mod spotify;


// // tokio let's us use "async" on our main function
// #[tokio::main]
// async fn main() {
//     let client = reqwest::Client::new();
//
//     //load in the dotenv
//     dotenv().ok();
//
//     let client_secret = std::env::var("CLIENT_SECRET").unwrap();
//     let client_id = std::env::var("CLIENT_ID").unwrap();
//
//     let access_credentials = spotify::get_auth_code(
//         &client,
//         &client_id,
//         &client_secret,
//     ).await;
// }

use actix_web::{middleware, web, App, HttpRequest, HttpServer};


async fn index(req: HttpRequest) -> &'static str {
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
    let artist = spotify::query_builder(&client, &access_credentials.unwrap(), 1);
    "Hello world?"
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

