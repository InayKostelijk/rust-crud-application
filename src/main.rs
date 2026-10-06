#![allow(unused_imports)]
use std::env::{self, var};
use tokio::net::TcpListener;
use axum::{
    Json, Router, extract::{Path,State}, http::StatusCode, routing::{get, patch, post, put}, serve::Listener
};
use dotenvy::dotenv;

use serde::{Serialize, Deserialize};
use serde_json::json;

use sqlx::{mysql:: MySqlPoolOptions, MySqlPool};
#[warn(unused_variables)]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Env not found");
    let server_address = std::env::var("SERVER_ADDRESS").expect("server adres not found");
    let database_url = std::env::var("DATABASE_URL").expect("database url not found");

    
    let db_pool = MySqlPoolOptions::new().max_connections(16).connect(&database_url).await.expect("Can't connect to database");
    let api = Router::new().route("/", get(|| async { "Hello, World! this is a test" }));

    let listener = TcpListener::bind(server_address).await.expect("Could not create listener");

    print!("listening on {}", listener.local_addr().unwrap());
    print!("hello");

    axum::serve(listener, api).await.expect("Could not start the application because of an error")



}