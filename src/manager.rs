use askama::Template;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{
    FromRow,
    sqlite::{SqliteConnectOptions, SqlitePool},
};
use std::fs::read_to_string;
use std::str::FromStr;
use std::sync::{Arc, Mutex, MutexGuard};
use tokio::net::TcpListener;
use uuid::Uuid;

#[derive(Debug, Template)]
#[template(path = "home.html")]
struct RecipeFormat {
    ingredients: Vec<String>,
    seasonings: Vec<String>,
    cooking_tools: Vec<String>,
    cooking_time: String,
}

#[derive(Debug, FromRow)]
struct Recipe {
    recipe_name: String,
    description: RecipeFormat,
}

//the frontend will recieve info through forms that uses the method "Post"
//Once post is invoked, the input will be incremented into the vectors
//THe values in the vectors will then to put into the database 1 by 1

pub async fn new() -> Result<(), sqlx::Error> {
    let connection = SqliteConnectOptions::from_str("sqlite://sqlite.db")?.create_if_missing(true);
    let pool = SqlitePool::connect_with(connection).await?;

    let router = Router::new()
        .route(
            "/",
            get(get_database)
                .post(post_database)
                .delete(delete_database)
                .put(put_database),
        )
        .with_state(pool.clone());
    let address: String = String::from("0.0.0.0:3000");
    let listener: TcpListener = TcpListener::bind(address)
        .await
        .expect("Couldn't bind to address");

    create_recipe_table(&pool);

    axum::serve(listener, router)
        .await
        .expect("Unable to start web server");

    Ok(())
}

pub async fn create_recipe_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS recipes(
            id PRIMARY KEY AUTOINCREMENT,
            Recipe Name TEXT NOT NULL,
            Ingredients TEXT NOT NULL,
            Seasonings TEXT NOT NULL,
            Cooking Tools TEXT NOT NULL,
            Cooking Time TEXT NOT NULL,
        );",
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_database(
    State(recipe_database): State<SqlitePool>,
) -> Result<Html<String>, sqlx::Error> {
    let mut ingredients: Vec<String> = Vec::new();
    let mut seasonings: Vec<String> = Vec::new();
    let mut cooking_tools: Vec<String> = Vec::new();
    let cooking_time: String = String::new();

    let page = RecipeFormat {
        ingredients: ingredients,
        seasonings: seasonings,
        cooking_tools: cooking_tools,
        cooking_time: cooking_time,
    };

    if let axum::response::Html(Ok(res)) = Html(page.render()) {
        return Ok(Html(res));
    }

    Err(sqlx::Error::Io)
}

pub async fn post_database() {}

pub async fn delete_database() {}

pub async fn put_database() {}

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
