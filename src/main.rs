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


mod v1;
mod state;
mod test;
mod util;
mod error;

#[warn(unused_variables)]
#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Env not found");
    let server_address = std::env::var("SERVER_ADDRESS").expect("server adres not found");
    let database_url = std::env::var("DATABASE_URL").expect("database url not found");

    
    let pool = MySqlPoolOptions::new().max_connections(16).connect(&database_url).await.expect("Can't connect to database");
    sqlx::migrate!().run(&pool).await.expect("Migrations failed");



    let state = state::State::new(pool);
    let api = Router::new().nest("api/v1", v1::routes::router()).with_state(state);


    let listener = TcpListener::bind(server_address).await.expect("Could not create listener");





    axum::serve(listener, api).await.expect("Could not start the application because of an error")



}