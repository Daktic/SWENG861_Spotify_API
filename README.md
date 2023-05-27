# SWENG861_Spotify_API

## Purpose

The purpose of this API is to request either Artist, or
Song names and receive a spotify response object that can be parsed or used in a frontend.

## What are the Inputs and Outputs?

### Input

The input will be a GET request to a localhost web address.
It will expect a body with the query paramater in it.
The exact path will determine if it is for an artist, or if for a song.

### Output

The output will be a JSON of elements from the Spotify API.
The response will facilitate the ability for a frontend to do more if desired.

## What Programming Language?

The API will be written in rust.
The Rust crate Reqwest will handle querying the Spotify API.
The actix_web crate will handle responding to inbound API requests.
using the actix_web crate will allow to build a simple frontend in react if time permits.

## Which API?

We will be querying the Spotify public API using a client ID and client Secret.

