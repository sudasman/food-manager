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
struct DisplayRecipes {
    recipes: Vec<RecipeFormat>,
}

#[derive(Debug)]
struct RecipeFormat {
    recipe_name: String,
    ingredients: Vec<String>,
    seasonings: Vec<String>,
    cooking_tools: Vec<String>,
    cooking_time: i32, 
}

//FromRow allows the sqlx to deserialize the table into a rust struct
#[derive(Debug, FromRow)]
struct Recipe {
    id: String,
    recipe_name: String,
    cooking_time: i32,
}

//Use new types to avoid rust's orphan rule (Can't implment exterior trait for exterior type)
struct AppError(sqlx::Error);

//Makes sure my error implements intoresponse so the router doesn't throw an error
impl IntoResponse for AppError{
    fn into_response(self) -> Response{
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database Error",
        ).into_response()
    }
}
//The backend will recieve information from the frontend using forms
//The backend will store such information inside a database
//When the program loads up, the saved data from the database should be visualized in the frontend

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

    create_recipe_table(&pool).await;

    axum::serve(listener, router)
        .await
        .expect("Unable to start web server");

    Ok(())
}

pub async fn create_recipe_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    //Creating the tables that will later be filled with data
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS recipes (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE,
        --in minutes
        cooking_time INTEGER NOT NULL
        );
        
        CREATE TABLE IF NOT EXISTS ingredients (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER NOT NULL,
            name TEXT NOT NULL,

            --The foreign key maps to a primary key in the recipes table 
            FOREIGN KEY (recipe_id)
                REFERENCES recipes(id)
                --deletes all children if parent is deleted
                ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS seasonings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER NOT NULL,
            name TEXT NOT NULL,

            FOREIGN KEY (recipe_id)
                REFERENCES recipes(id)
                ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS cooking_tools (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            recipe_id INTEGER NOT NULL,
            name TEXT NOT NULL,

            FOREIGN KEY (recipe_id)
                REFERENCES recipes(id)
                ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_database(
    State(recipe_database): State<SqlitePool>,
) -> Result<Html<String>, AppError> {
    //Note: Recipe {id, recipe_name}
    let recipes: Vec<Recipe> = sqlx::query_as::<_, Recipe>(
        r#"
        SELECT id, name, cooking_time
        FROM recipes
        "#,
    )
    .fetch_all(&recipe_database)
    .await
    //IMPORTANT! The error is mapped to an error that implements intoresponse 
    .map_err(AppError)?;

    let mut recipe_list: Vec<RecipeFormat> = Vec::new();

    //Dear future Eason, notice the type annotations of each
    for recipe in recipes {
        let ingredients: Vec<String> = sqlx::query_scalar::<_, String>(
            r#"
            SELECT name
            FROM ingredients
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe.id)
        .fetch_all(&recipe_database)
        .await
        .map_err(AppError)?;

        let seasonings: Vec<String> = sqlx::query_scalar::<_, String>(
            r#"
            SELECT name 
            FROM seasonings
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe.id)
        .fetch_all(&recipe_database)
        .await
        .map_err(AppError)?;

        let cooking_tools: Vec<String> = sqlx::query_scalar::<_, String>(
            r#"
            SELECT name
            FROM cooking_tools
            WHERE recipe_id = ?
            "#,
        )
        .bind(&recipe.id)
        .fetch_all(&recipe_database)
        .await
        .map_err(AppError)?;

        recipe_list.push(RecipeFormat {
            recipe_name: recipe.recipe_name,
            ingredients: ingredients,
            seasonings: seasonings,
            cooking_tools: cooking_tools,
            cooking_time: recipe.cooking_time,
        });
    }

    let page = DisplayRecipes {
        recipes: recipe_list,
    };

    if let axum::response::Html(Ok(res)) = Html(page.render()) {
        return Ok(Html(res));
    }
    //dummy return value
    Err(AppError(sqlx::Error::RowNotFound))
}

pub async fn post_database() {}

pub async fn delete_database() {}

pub async fn put_database() {}
