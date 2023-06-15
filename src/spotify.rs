use reqwest;
use std::collections::HashMap;
use serde_urlencoded;
use serde_json;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use reqwest::Client;
use dotenv::{dotenv};
use std::env;
use tokio::task;
use std::sync::{Arc, Mutex};
use futures::future::err;
use log::log;
use crate::ArtistResponse;


fn create_client() -> Client {
    reqwest::Client::new()
}

async fn get_access_credentials(client: &reqwest::Client) -> AccessCode {
    dotenv::dotenv().expect("Failed to load ENV");
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


    let response = client.post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(query_string)
        .send().await;
    let auth_code = response.expect("REASON").text().await.unwrap();
    let access_credentials: AccessCode = serde_json::from_str(&auth_code).unwrap();
    access_credentials
}

pub enum QueryResult {
    QueryArtists(Artists),
    Tracks(Songs),
    SpotifyError(SpotifyError),
}

impl QueryResult {
    pub fn get_artist(&self) -> Option<&Artists> {
        match self {
            QueryResult::QueryArtists(artists) => Some(artists),
            _ => None,
        }
    }
    pub fn get_song(&self) -> Option<&Songs> {
        match self {
            QueryResult::Tracks(songs) => Some(songs),
            _ => None,
        }
    }
    pub fn get_error(&self) -> Option<&SpotifyError> {
        match self {
            QueryResult::SpotifyError(error) => Some(error),
            _ => None,
        }
    }
}

pub async fn query_builder(
    query: &str,
    type_of_search: u8,
) -> QueryResult {
    match type_of_search {
        1 => {
            match get_artists(query).await {
                Ok(artists) => QueryResult::QueryArtists(artists),
                Err(error) => QueryResult::SpotifyError(error),
            }
        }
        2 => {
            match get_songs(query).await {
                Ok(songs) => QueryResult::Tracks(songs),
                Err(error) => QueryResult::SpotifyError(error),
            }
        }
        _ => {
            println!("Not a proper search param.");
            QueryResult::SpotifyError(SpotifyError {
                error: SpotifyErrorMessage {
                    status: 400,
                    message: "Invalid search parameter".to_string(),
                },
            })
        }
    }
}

async fn get_artists(query_string: &str) -> Result<Artists, SpotifyError> {
    let artist_ids = get_artist_ids(query_string.as_ref()).await;

    match artist_ids {
        Ok(ids) => {
            let mut artists = get_artists_details(&ids).await;
            artists.artists.sort_by(|a, b| b.followers.total.cmp(&a.followers.total));
            Ok(artists)
        }
        Err(err) => Err(err),
    }
}

async fn get_songs(query_string: &str) -> Result<Songs, SpotifyError> {
    let song_ids = get_song_ids(query_string.as_ref()).await;

    match song_ids {
        Ok(ids) => {
            let mut songs = get_songs_details(&ids).await;
            songs.songs.sort_by(|a, b| b.album.popularity.cmp(&a.album.popularity));
            Ok(songs)
        }
        Err(err) => Err(err),
    }
}

async fn get_artist_ids(
    query_string: &str,
) -> Result<SpotifyArtist, SpotifyError> {
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

    //dbg!(&response);

    let result: Result<SpotifyArtist, SpotifyError> = serde_json::from_str(&response)
        .map_err(|error| {
            // Handle the deserialization error here
            log::info!("Error occurred during JSON deserialization: {}", error);

            // Try to deserialize the response into the error struct
            if let Ok(error_response) = serde_json::from_str::<SpotifyError>(&response) {
                // dbg!(&error_response);
                SpotifyError {
                    error: SpotifyErrorMessage {
                        status: error_response.error.status,
                        message: error_response.error.message,
                    },
                }
            } else {
                // dbg!(&error);
                // Fallback to a generic error if the response doesn't match the expected error structure
                SpotifyError {
                    error: SpotifyErrorMessage {
                        status: 500,
                        message: "Unknown error occurred".to_string(),
                    },
                }
            }
        });
    result
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

async fn get_song_ids(
    query_string: &str,
) -> Result<SpotifyTrack, SpotifyError> {
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


    let result: Result<SpotifyTrack, SpotifyError> = serde_json::from_str(&response)
        .map_err(|error| {
            // Handle the deserialization error here
            log::info!("Error occurred during JSON deserialization: {}", error);
            // Try to deserialize the response into the error struct
            if let Ok(error_response) = serde_json::from_str::<SpotifyError>(&response) {
                SpotifyError {
                    error: SpotifyErrorMessage {
                        status: error_response.error.status,
                        message: error_response.error.message,
                    },
                }
            } else {
                // Fallback to a generic error if the response doesn't match the expected error structure
                SpotifyError {
                    error: SpotifyErrorMessage {
                        status: 500,
                        message: "Unknown error occurred".to_string(),
                    },
                }
            }
        });
    result
}

async fn get_songs_details(spotify_tracks: &SpotifyTrack) -> Songs {
    //dbg!(&spotify_tracks.tracks.items);
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

    //dbg!(&response);
    let song: Song = serde_json::from_str(&response).unwrap();

    return song;
}


#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyError {
    pub error: SpotifyErrorMessage,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SpotifyErrorMessage {
    pub status: u16,
    pub message: String,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Artists {
    pub artists: Vec<Artist>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Artist {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
struct SongArtist {
    external_urls: ExternalUrls,
    href: String,
    id: String,
    name: String,
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
    pub songs: Vec<Song>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Song {
    album: AlbumItems,
    artists: Vec<SongArtist>,
    available_markets: Vec<String>,
    disc_number: u8,
    duration_ms: u32,
    explicit: bool,
    external_ids: AlbumItemExternalIds,
    external_urls: ExternalUrls,
    href: String,
    id: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TrackItems {
    album: AlbumItems,
    artists: Vec<TrackArtist>,
    id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AlbumItemRestrictions {
    reason: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AlbumItemCopyrights {
    text: String,
    r#type: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct AlbumItemExternalIds {
    isrc: String,
    ean: Option<String>,
    upc: Option<String>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
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
    followers: Option<Followers>,
    copyrights: Option<Vec<AlbumItemCopyrights>>,
    external_ids: Option<Vec<AlbumItemExternalIds>>,
    genres: Option<Vec<String>>,
    label: Option<String>,
    popularity: Option<u32>,
    album_group: Option<String>,

}

