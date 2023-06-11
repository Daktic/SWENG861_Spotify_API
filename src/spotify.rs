use reqwest;
use std::collections::HashMap;
use serde_urlencoded;
use serde_json;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use reqwest::Client;
use dotenv::dotenv;
use std::env;

#[derive(Deserialize, Debug)]
pub struct AccessCode {
    pub access_token: String,
    token_type: String,
    expires_in: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotifyArtist {
    artists: Artists,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artists {
    href: String,
    items: Vec<Artist>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Artist {
    external_urls: ExternalUrls,
    followers: Followers,
    genres: Vec<String>,
    href: String,
    id: String,
    images: Vec<Image>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrackArtist {
    external_urls: ExternalUrls,
    followers: Option<Followers>,
    genres: Option<Vec<String>>,
    href: String,
    id: String,
    images: Option<Vec<Image>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Followers {
    href: Option<String>,
    total: i32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Image {
    height: Option<i32>,
    url: String,
    width: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotifyTrack {
    tracks: Tracks,
}

#[derive(Debug, Deserialize, Serialize)]
struct Tracks {
    href: String,
    limit: u16,
    next: Option<String>,
    offset: u16,
    previous: Option<String>,
    total: u32,
    items: Vec<TrackItems>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrackItems {
    album: AlbumItems,
    artists: Vec<TrackArtist>,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlbumItemRestrictions {
    reason: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlbumItemCopyrights {
    text: String,
    r#type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlbumItemExternalIds {
    isrc: String,
    ean: String,
    upc: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct AlbumItems {
    album_type: String,
    total_tracks: u16,
    available_markets: Vec<String>,
    external_urls: ExternalUrls,
    href: String,
    id: String,
    images: Vec<Image>,
    name: String,
    release_date: String,
    release_date_precision: String,
    restrictions: Option<Vec<AlbumItemRestrictions>>,
    r#type: String,
    uri: String,
    copyrights: Option<Vec<AlbumItemCopyrights>>,
    external_ids: Option<Vec<AlbumItemExternalIds>>,
    genres: Option<Vec<String>>,
    label: Option<String>,
    popularity: Option<u32>,
    album_group: Option<String>,

}


fn create_client() -> Client {
    reqwest::Client::new()
}

async fn get_access_credentials(client: &reqwest::Client) -> AccessCode {
    dotenv::dotenv();
    get_auth_code(
        client,
        &env::var("CLIENT_ID")
            .expect("Did not find Client ID"),
        &env::var("CLIENT_SECRET")
            .expect("Did not find Client SECRET"),
    ).await
}

async fn get_auth_code(
    client: &reqwest::Client,
    client_id: &str,
    client_secret: &str,
) -> AccessCode {
    //println!("In the async function");
    let mut params = HashMap::new();
    params.insert("grant_type", "client_credentials");
    params.insert("client_id", client_id);
    params.insert("client_secret", client_secret);
    let query_string = serde_urlencoded::to_string(params).unwrap();

    // let url = format!("https://accounts.spotify.com/api/token?{}", query_string);
    // dbg!(&url);
    let response = client.post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(query_string)
        .send().await;
    //println!("In the async function");
    let auth_code = response.expect("REASON").text().await.unwrap();
    let access_credentials: AccessCode = serde_json::from_str(&auth_code).unwrap();
    access_credentials
}

pub enum QueryResult {
    Artists(SpotifyArtist),
    Tracks(SpotifyTrack),
    Error,
}

pub async fn query_builder(
    query: &str,
    type_of_search: u8,
) -> QueryResult {
    match type_of_search {
        1 => QueryResult::Artists(get_artist_details(query).await),
        //TODO make 2 like 1, remove the other params
        2 => QueryResult::Artists(get_artist_details(query).await),//QueryResult::Tracks(get_song_details(query, client, access_credentials).await),
        _ => {
            println!("Not a proper search param.");
            // Return a default value or handle the invalid case accordingly
            // Here, I'm returning an empty enum variant as an example
            QueryResult::Error
        }
    }
}

async fn get_artist_details(
    query_string: &str,
) -> SpotifyArtist {
    let client = create_client();
    let access_credentials = get_access_credentials(&client).await;

    let url = format!("https://api.spotify.com/v1/search?q={query}&type=artist&offset=0&limit=20",
                      query = query_string);

    let response = client
        .get(url)
        .header("AUTHORIZATION", "Bearer ".to_owned() + &access_credentials.access_token)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .expect("Failed to execute get request")
        .text()
        .await.
        unwrap();

    // dbg!(&response);

    let artist_details: SpotifyArtist = serde_json::from_str(&response).expect("Failed to deserialize response");
    //dbg!(&artist_details);

    return artist_details;
}

async fn get_song_details(
    query_string: &str,
    client: &reqwest::Client,
    access_credentials: &str,
) -> SpotifyTrack {
    // let mut query_string = String::new();
    // println!("Please enter the song you wish to query.");
    // std::io::stdin().read_line(&mut query_string).unwrap();

    let url = format!("https://api.spotify.com/v1/search?q={query}&type=track&offset=0&limit=20",
                      query = query_string);
    let response = client
        .get(url)
        .header("AUTHORIZATION", "Bearer ".to_owned() + access_credentials)
        .header("CONTENT_TYPE", "application/json")
        .header("ACCEPT", "application/json")
        .send()
        .await
        .expect("Failed to execute get request")
        .text()
        .await.
        unwrap();


    let song_details: SpotifyTrack = serde_json::from_str(&response).expect("Failed to deserialize response");

    return song_details;
}

impl QueryResult {
    pub fn get_artist(&self) -> Option<&SpotifyArtist> {
        match self {
            QueryResult::Artists(artist) => Some(artist),
            _ => None,
        }
    }
}