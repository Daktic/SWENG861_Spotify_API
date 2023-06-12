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
use tokio::task;
use std::sync::{Arc, Mutex};


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
    QueryArtists(Artists),
    Tracks(SpotifyTrack),
    Error,
}

impl QueryResult {
    pub fn get_artist(&self) -> Option<&Artists> {
        match self {
            QueryResult::QueryArtists(artists) => Some(artists),
            _ => None,
        }
    }
}

pub async fn query_builder(
    query: &str,
    type_of_search: u8,
) -> QueryResult {
    match type_of_search {
        1 => QueryResult::QueryArtists(get_artists(query).await),
        //TODO make 2 like 1, remove the other params
        2 => QueryResult::QueryArtists(get_artists(query).await),//QueryResult::Tracks(get_song_details(query, client, access_credentials).await),
        _ => {
            println!("Not a proper search param.");
            // Return a default value or handle the invalid case accordingly
            // Here, I'm returning an empty enum variant as an example
            QueryResult::Error
        }
    }
}

async fn get_artists(query_string: &str) -> Artists {
    let artist_ids = get_artist_ids(query_string.as_ref()).await;
    let mut artists = get_artists_details(&artist_ids).await;
    // This line takes the artist:Artists and sorts the Artists.artists vec by followers in descending order.
    artists.artists.sort_by(|a, b| b.followers.total.cmp(&a.followers.total));
    artists
}

async fn get_songs(query_string: &str) -> Songs {
    let song_ids = get_song_ids(query_string.as_ref()).await;
    let mut songs = get_songs_details(&song_ids).await;
    dbg!(&songs);
    songs
}

async fn get_artist_ids(
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

    let artists: SpotifyArtist = serde_json::from_str(&response).expect("Failed to deserialize response");
    //dbg!(&artist_details);


    return artists;
}

async fn get_artists_details(spotify_artist: &SpotifyArtist) -> Artists {
    let artists = &spotify_artist.artists.items;

    let mut tasks = vec![];

    let artist_vec = Arc::new(Mutex::new(vec![]));

    for artist in artists {
        let artist_id = artist.id.clone(); // Clone the artist ID
        let artist_vec_clone = Arc::clone(&artist_vec);

        let task = task::spawn(async move {
            let artist_details = get_artist_details(&artist_id).await;

            let mut lock = artist_vec_clone.lock().unwrap();
            lock.push(artist_details);
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    let artists = Artists {
        artists: artist_vec.lock().unwrap().clone(),
    };

    artists
}

async fn get_songs_details(spotify_tracks: &SpotifyTrack) -> Songs {
    dbg!(&spotify_tracks.tracks.items);
    let songs = &spotify_tracks.tracks.items;

    let mut tasks = vec![];

    let song_vec = Arc::new(Mutex::new(vec![]));

    for song in songs {
        let song_id = song.id.clone(); // Clone the artist ID
        let song_vec_clone = Arc::clone(&song_vec);

        let task = task::spawn(async move {
            let song_details = get_song_details(&song_id).await;

            let mut lock = song_vec_clone.lock().unwrap();
            lock.push(song_details);
        });

        tasks.push(task);
    }

    for task in tasks {
        task.await.unwrap();
    }

    let songs = Songs {
        songs: song_vec.lock().unwrap().clone(),
    };

    songs
}

async fn get_artist_details(artist_id: &str) -> Artist {
    let client = create_client();
    let access_credentials = get_access_credentials(&client).await;
    let url = format!("https://api.spotify.com/v1/artists/{artist_id}");

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

    let artist: Artist = serde_json::from_str(&response).unwrap();

    return artist;
}

async fn get_song_details(song_id: &str) -> Song {
    let client = create_client();
    let access_credentials = get_access_credentials(&client).await;
    let url = format!("https://api.spotify.com/v1/tracks/{song_id}");

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

    let song: Song = serde_json::from_str(&response).unwrap();

    return song;
}

async fn get_song_ids(
    query_string: &str,
) -> SpotifyTrack {
    let client = create_client();
    let access_credentials = get_access_credentials(&client).await;

    let url = format!("https://api.spotify.com/v1/search?q={query}&type=track&offset=0&limit=20",
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


    let song_details: SpotifyTrack = serde_json::from_str(&response).expect("Failed to deserialize response");

    return song_details;
}


#[derive(Deserialize, Debug)]
pub struct AccessCode {
    pub access_token: String,
    token_type: String,
    expires_in: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SpotifyArtist {
    artists: QueryArtists,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryArtists {
    href: String,
    items: Vec<QueryArtist>,
}

#[derive(Debug, Deserialize, Serialize)]
struct QueryArtist {
    external_urls: ExternalUrls,
    followers: Followers,
    genres: Vec<String>,
    href: String,
    id: String,
    images: Vec<Image>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Artists {
    artists: Vec<Artist>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Artist {
    external_urls: ExternalUrls,
    followers: Followers,
    genres: Vec<String>,
    href: String,
    id: String,
    images: Vec<Image>,
    name: String,
    popularity: u32,
    r#type: String,
    uri: String,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ExternalUrls {
    spotify: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Followers {
    href: Option<String>,
    total: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Songs {
    songs: Vec<Song>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    artists: Vec<Song>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrackItems {
    album: AlbumItems,
    artists: Vec<TrackArtist>,
    id: String,
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

