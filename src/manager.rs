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
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePool}, FromRow};
use std::str::FromStr;

//template
struct RecipeFormat{
    ingredients: Vec<String>,
    seasoning: Vec<String>,
    cooking_tools: Vec<String>,
    time: String,
}

#[derive(Debug, FromRow)]
struct Recipe{
    recipe_name: String,
    recipe_format: RecipeFormat,
}

//the table will be filled with entries provided by form from the frontend
pub async fn new () -> Result<(), sqlx::Error>{

    let connection = SqliteConnectOptions::from_str("sqlite://sqlite.db")?.create_if_missing(true);
    let pool = SqlitePool::connect_with(connection).await?;

    let router = Router::new().route("/", get(get_demo)).with_state(pool.clone());
    let address: String = String::from("0.0.0.0:3000");
    let listener: TcpListener = TcpListener::bind(address)
    .await
    .expect("Couldn't bind to address");


    axum::serve(listener, router)
    .await
    .expect("Unable to start web server");

    Ok(())
}

pub async fn create_recipe_table() -> Result<(), sqlx::Error>{
    sqlx::query(
        "CREATE TABLE PRIMARY KEY AUTOINCREMENT,
        Recipe Name TEXT NOT NULL,
        Recipe Format TEXT NOT NULL"
    )
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn get_demo(){}



// sqlx ::query(
//         "CREATE TABLE IF NOT EXISTS users (
//             id INTEGER PRIMARY KEY AUTOINCREMENT,
//             name TEXT NOT NULL
//         );"
//     )
//     .execute(&pool)
//     .await?;

//     sqlx::query("INSERT INTO users (name) VALUES (?)")
//     .bind("Bob")
//     .execute(&pool)
//     .await?;

// let user: test = sqlx::query_as::<_, test>("SELECT id, name FROM users WHERE name = ?")
//         .bind("Bob")
//         .fetch_one(&pool)
//         .await?;
//     dbg!(user);


