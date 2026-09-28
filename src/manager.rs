use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use std::fs::read_to_string;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::net::TcpListener;
use uuid::Uuid;
use askama::Template;

//template
struct recipe_format{
    ingredients: Vec<String>,
    seasoning: Vec<String>,
    cooking_tools: Vec<String>,
    time: String,
}

pub async fn new () {
    let router = Router::new().route("/", get(get_demo));

    let address: String = String::from("0.0.0.0:3000");

    let listener: TcpListener = TcpListener::bind(address)
    .await
    .expect("Couldn't bind to address");

    axum::serve(listener, router)
    .await
    .expect("Unable to start web server");
}

pub async fn get_demo(){}