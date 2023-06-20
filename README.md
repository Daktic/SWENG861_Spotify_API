# SWENG861_Spotify_API

## Purpose & Overview

The purpose of this API is to request either Artist, or
Song names and receive a spotify response object that can be parsed or used in a frontend.
Users will be able to select what they wish to search for and a text box entry will allow
them to type in a query that will return a list of items matching that query from the API.
The response will be a JSON and that will manifest in the frontend.

## How to run this project

1. Visit the official Rust website at https://www.rust-lang.org/.
    - Follow the installation instructions provided on the website for the chosen operating system.
2. Clone or Download Program: https://github.com/Daktic/SWENG861_Spotify_API.git
    - example : `git clone https://github.com/Daktic/SWENG861_Spotify_API.git`
3. run the program from the terminal
    - Navigate to the root directory of the program
    - Run the following command to compile the Rust program:
      `cargo build`
    - Run the following command to run the Rust program:
      `cargo run`

## Use Cases

### Artist Search

When the Artist search is toggled the text entry will query the artists from the spotify API.
The response will contain, names, follower counts, and perhaps either genres, a picture or both.

### Song Search

When a song is selected the text box will query the Spotify API for a song name matching the text input.
The response will then contain more information about the song such as artist, genre and perhaps album artwork.

## What are the Inputs and Outputs?

### Input

The input will be a GET request to a localhost web address.
It will expect a url with a ?query="**querytext**" parameter in it.
There will also be a queryType param to indicate if it is a song or a artist request.

### Output

The output will be a JSON of elements from the Spotify API.
The response will facilitate the ability for a frontend to do more if desired.

## What Programming Language?

The API will be written in rust.
The Rust crate Reqwest will handle querying the Spotify API.
The actix_web crate will handle responding to inbound API requests.
using the actix_web crate will allow to build a simple frontend if time permits.

## Which API?

We will be querying the Spotify public API using a client ID and client Secret.

## Project flowcharts

<img width="516" alt="Screen Shot 2023-06-20 at 4 03 01 PM" src="https://github.com/Daktic/SWENG861_Spotify_API/assets/58493066/2075437c-2183-4be4-b3f4-d497300b8782">
<img width="548" alt="Screen Shot 2023-06-20 at 4 21 46 PM" src="https://github.com/Daktic/SWENG861_Spotify_API/assets/58493066/02775192-d6b1-4c57-aa77-ec90e8d3ebe7">
