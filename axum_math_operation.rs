
use axum::{
    extract::Path, routing::{get,post}, Json, Router
};
use serde::Deserialize;

#[derive(Deserialize)]
struct Numbers { 
    number_one: i128,
    number_two: i128,
}

#[tokio::main]
async  fn main() {

let app = Router::<()>::new().route("/math/{number_one}/{number_two}", get(crazy_math).post(crazy_math));
let listener = tokio::net::TcpListener::bind("0.0.0.0:1390").await.unwrap();
println!("http://0.0.0.0:1390");
axum::serve(listener,app).await.unwrap();
}

async  fn crazy_math(Path(Numbers{number_one,number_two}): Path<Numbers>) -> Json<i128>{
        let result = number_one - number_two;
        Json(result)
} 
