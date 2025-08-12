use axum::{
    routing::get,
    Router,
};

#[tokio::main]


async fn main() {



    let app = Router::<()>::new().route("/", get(root))
    .route("/fun", get(fun).post(post_fun));  // so you  can chain mmultiple things and can save time not defeinign them over and over again

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
async fn post_fun() -> String{
    let bob : String = String::from("Post_fun");
    println!("Post fun");
    bob
}
