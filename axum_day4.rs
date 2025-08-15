use axum::{
    body::Bytes, extract::{Json, Path, Query, Request, State}, http::HeaderMap, routing::{get, post}, Router
};
use serde::Deserialize;
use serde_json::{json, Value};

extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use anyhow::Result;

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}
#[tokio::main]

async fn main() {



    let app = Router::<()>::new().route("/", get(root))
    .route("/fun", get(fun).post(post_fun))
    .route("/users", post(post_fun))
    .route("/math_add", post(math_thingy))
    .route("/math/{num1}/-{num2}",get(simple_math).post(simple_math))
    .route("/user_register", post(user_check))
    .route("/register", post(user_register));  // so you  can chain mmultiple things and can save time not defeinign them over and over again

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
#[derive(Deserialize)]
struct  MathThingy {
    first_num : isize,
    second_num : isize,
}

#[derive(Deserialize)]
struct  User{
    num1: isize,
    num2: isize,
}
// goal is to have  a route where you can add 2 numbers together 

async  fn math_thingy(Json(number): Json<MathThingy>) -> Json<isize>{

    
    let first_num = number.first_num;
    let secound_num = number.second_num;

    let result =     first_num + secound_num;
    Json(result)
}
async  fn simple_math(Path(User{num1,num2 }) : Path<User>) -> Json<i128>{
    if num1 == 139 || num2 == 139{
        return Json(3139207761732068657265);
    }

    println!("reacherd");
    let result = num1 - num2;
    println!("{result}");
    Json(result as i128)
}
#[derive(Deserialize)]
struct Usercheck {
    username: String,
    password: String,
    email_addres: String,
    age: u8
}

async  fn user_check(Json(user): Json<Usercheck>) -> Json<(String)>{
    
    if user.username.is_empty() || user.password.is_empty() || user.email_addres.is_empty()  {
       return axum::Json(Json(json!("didn't suply with a username or a password or an email address ")).to_string());
    }

    let username = user.username;
    let password = user.password;
    let email_address = user.email_addres;
    let age = user.age;

    if age <= 18 {
        return axum::Json(Json(json!("You must be old enough to use this produkt 18+ ")).to_string());

    }
    println!("{username},{password}, {email_address} , {age}");

    Json({
        String::from("Done")
    })
}

// todays task is to have a simple register function and give user a jsonwebtoken and bycrypt their password

#[derive(Deserialize)]
struct UserRegister{
    username: String,
    password: String,
    email_address: String,

}
async  fn user_register(Json(user): Json<UserRegister>) -> Json<(String)>{


    let username = user.username;
    let password = user.password;
    let email_address = user.email_address;

    if email_address.len() <= 3 {
        return axum::Json(Json(json!("This email address is not valid brother ")).to_string());

    }

    if username.len() <= 3 && username.len() >= 18 {
        return axum::Json(Json(json!("{username} is to short should be  betwen 3 and 18 caracters ")).to_string());
    }
    if password.len() <= 3 && password.len() >= 18 {
        return axum::Json(Json(json!("{password} is to short should be  betwen 3 and 18 caracters ")).to_string());
    }

    let hash_password: String = hash_password(password).await.expect("idk");
    
    // let hash_password = hash_password.to_string();
    let finalmsg = "Congrats you have made it ";
    let finalmsg2 = "Your password encrypted safely";
    let combined_msg = [finalmsg,finalmsg2,hash_password.as_str()].join(" ");

    Json({
        
        combined_msg
    })
}


async fn hash_password(password: String) -> Result<String>{


    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)

} 
