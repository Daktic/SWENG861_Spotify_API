use dotenv::dotenv;

mod spotify;


// tokio let's us use "async" on our main function
#[tokio::main]
async fn main() {
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

    let mut query_string = String::new();
    println!("What would you like to query?");
    println!("Type \"1\" for artist, \"2\" for song.");
    std::io::stdin().read_line(&mut query_string).unwrap();

    match query_string.trim().parse::<u8>().expect("Not a number") {
        1 => spotify::query_builder(&client, &*access_credentials.access_token, 1).await,
        2 => spotify::query_builder(&client, &*access_credentials.access_token, 2).await,
        _ => println!("Invalid selection.")
    }

    //spotify::get_artist_details(&client, "flume", &*access_credentials.access_token).await;
    spotify::query_builder(&client, &*access_credentials.access_token, 1).await;
}



