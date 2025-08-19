use axum::{
    body::Bytes, extract::{Json, Path, Query, Request, State}, http::HeaderMap, routing::{get, post}, Router
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// use std::collections::BTreeMap;
use sqlx::{pool, postgres::PgPoolOptions};
use tracing_subscriber::field::debug;
use std::{env, io::BufRead};
use dotenv::dotenv;

extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use anyhow::Result;
use rust_basics as lib;
use rust_basics::route as r0out;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}
#[tokio::main]

async fn main() {



    let app = Router::<()>::new().route("/", get(rust_basics::route::rr))
    .route("/fun", get(r0out::fun).post(r0out::post_fun))
    .route("/users", post(r0out::post_fun))
    .route("/math_add", post(r0out::math_thingy))
    .route("/math/{num1}/-{num2}",get(r0out::simple_math).post(r0out::simple_math))
    .route("/user_register", post(r0out::user_check))
    .route("/register", post(r0out::user_register))// so you  can chain mmultiple things and can save time not defeinign them over and over again
    .route("/email_update",post(r0out::email_update))
    .route("/user_find", post(r0out::find_user_data))
    .route("/user_delete", post(r0out::delete_user_main));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("http://0.0.0.0:3000");
   r0out::connect_database().await;

    axum::serve(listener, app).await.unwrap();

}
