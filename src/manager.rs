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

pub async fn new () {
    let router = Router::new().route("/");
}