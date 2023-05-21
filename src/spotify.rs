use reqwest;
use std::collections::HashMap;
use serde_urlencoded;
use serde_json;
use serde::{Deserialize};
use std::fs::File;
use std::io::Write;
use reqwest::Client;

#[derive(Deserialize, Debug)]
pub struct AccessCode {
    pub access_token: String,
    token_type: String,
    expires_in: u16,
}

#[derive(Debug, Deserialize)]
struct SpotifyArtist {
    artists: Artists,
}

#[derive(Debug, Deserialize)]
struct Artists {
    href: String,
    items: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
struct Artist {
    external_urls: ExternalUrls,
    followers: Followers,
    genres: Vec<String>,
    href: String,
    id: String,
    images: Vec<Image>,
}

#[derive(Debug, Deserialize)]
struct TrackArtist {
    external_urls: ExternalUrls,
    followers: Option<Followers>,
    genres: Option<Vec<String>>,
    href: String,
    id: String,
    images: Option<Vec<Image>>,
}

#[derive(Debug, Deserialize)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Debug, Deserialize)]
struct Followers {
    href: Option<String>,
    total: i32,
}

#[derive(Debug, Deserialize)]
struct Image {
    height: Option<i32>,
    url: String,
    width: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct SpotifyTrack {
    tracks: Tracks,
}

#[derive(Debug, Deserialize)]
struct Tracks {
    href: String,
    limit: u16,
    next: Option<String>,
    offset: u16,
    previous: Option<String>,
    total: u32,
    items: Vec<TrackItems>,
}

#[derive(Debug, Deserialize)]
struct TrackItems {
    album: AlbumItems,
    artists: Vec<TrackArtist>,
}

#[derive(Debug, Deserialize)]
struct AlbumItemRestrictions {
    reason: String,
}

#[derive(Debug, Deserialize)]
struct AlbumItemCopyrights {
    text: String,
    r#type: String,
}

#[derive(Debug, Deserialize)]
struct AlbumItemExternalIds {
    isrc: String,
    ean: String,
    upc: String,
}

#[derive(Debug, Deserialize)]
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

pub(crate) async fn get_auth_code(
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

pub async fn query_builder(
    client: &reqwest::Client,
    access_credentials: &str,
    type_of_search: u8,
) {
    match type_of_search {
        1 => get_artist_details(client, access_credentials).await,
        2 => get_song_details(client, access_credentials).await,
        _ => println!("Not a proper search param.")
    }
}

async fn get_artist_details(
    client: &reqwest::Client,
    access_credentials: &str,
) {
    let mut query_string = String::new();
    println!("Please enter the artist you wish to query.");
    std::io::stdin().read_line(&mut query_string).unwrap();

    let url = format!("https://api.spotify.com/v1/search?q={query}&type=artist&offset=0&limit=20",
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

    dbg!(&response);

    let artist_details: SpotifyArtist = serde_json::from_str(&response).expect("Failed to deserialize response");

    dbg!(&artist_details);

    // let album_type = &artist_details.items.get("album").unwrap().album.get("album_type").unwrap().album_type;
    //
    // println!("{}", album_type);
}

async fn get_song_details(
    client: &reqwest::Client,
    access_credentials: &str,
) {
    let mut query_string = String::new();
    println!("Please enter the song you wish to query.");
    std::io::stdin().read_line(&mut query_string).unwrap();

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

    //dbg!(&response);
    let song_details: SpotifyTrack = serde_json::from_str(&response).expect("Failed to deserialize response");
    for i in 0..song_details.tracks.items.len() {
        dbg!(&song_details.tracks.items.get(i).unwrap().artists);
    }
}