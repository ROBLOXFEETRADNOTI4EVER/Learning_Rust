use axum::{
    body::Bytes, extract::{Json, Path, Query, Request, State}, http::HeaderMap, routing::{get, post}, Router
};
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;
#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}
#[tokio::main]

async fn main() {



    let mut  numb : i32 = 0;
    let app = Router::<()>::new().route("/", get(root))
    .route("/fun", get(fun).post(post_fun))
    .route("/users", post(post_fun));  // so you  can chain mmultiple things and can save time not defeinign them over and over again

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}

async  fn root() -> String{
    let bob : String = String::from("Root");
    bob
}

async  fn fun() -> String{
    let bob : String = String::from("fun");
    println!("fun");

    bob
}

async fn post_fun(Json(user): Json<CreateUser>)  -> String{
    let mut  numb: i32 = 0;

    let bob : String = String::from("Post_fun");
    numb += 1;
    println!("Post fun");
    print!("{numb}");
    println!("Email : {} Password : {}",user.email, user.password);
    bob
}




