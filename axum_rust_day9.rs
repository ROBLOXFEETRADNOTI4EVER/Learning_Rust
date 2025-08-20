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
    .route("/user_delete", post(r0out::delete_user_main))
    .route("/decode_user",post(r0out::decode_user));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("http://0.0.0.0:3000");
   r0out::connect_database().await;

    axum::serve(listener, app).await.unwrap();

}


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
use std::time::{SystemTime, UNIX_EPOCH, Duration};
extern crate bcrypt;

use bcrypt::{DEFAULT_COST, hash, verify};
use anyhow::Result;
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
#[derive(Deserialize)]
pub struct CreateUser {
    email: String,
    password: String,
}

pub async  fn rr() -> String{
    let bob : String = String::from("Root");
    bob
}

pub async  fn fun() -> String{
    let bob : String = String::from("fun");
    println!("fun");

    bob
}

pub async fn post_fun(Json(user): Json<CreateUser>)  -> String{
    let mut  numb: i32 = 0;

    let bob : String = String::from("Post_fun");
    numb += 1;
    println!("Post fun");
    print!("{numb}");
    println!("Email : {} Password : {}",user.email, user.password);
    bob
}
#[derive(Deserialize)]
pub struct  MathThingy {
    first_num : isize,
    second_num : isize,
}

#[derive(Deserialize)]
pub struct  User{
    num1: isize,
    num2: isize,
}
// goal is to have  a route where you can add 2 numbers together 

pub async  fn math_thingy(Json(number): Json<MathThingy>) -> Json<isize>{

    
    let first_num = number.first_num;
    let secound_num = number.second_num;

    let result =     first_num + secound_num;
    Json(result)
}
pub async  fn simple_math(Path(User{num1,num2 }) : Path<User>) -> Json<i128>{
    if num1 == 139 || num2 == 139{
        return Json(3139207761732068657265);
    }

    println!("reacherd");
    let result = num1 - num2;
    println!("{result}");
    Json(result as i128)
}
#[derive(Deserialize)]
pub struct Usercheck {
    username: String,
    password: String,
    email_addres: String,
    age: u8
}

pub async  fn user_check(Json(user): Json<Usercheck>) -> Json<(String)>{
    
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // subject, e.g. username or user_id
    exp: usize,  // expiration timestamp
}


#[derive(Deserialize)]
pub struct UserRegister{
    username: String,
    password: String,
    email_address: String,

}
pub async  fn user_register(Json(user): Json<UserRegister>) -> Json<(String)>{


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



    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");



    match read_user_by_name(&pool, &username.trim()).await {
        Ok(existing_users) => {
            if !existing_users.is_empty() {
                return axum::Json("Username already exists".to_string());
            }
        },
        Err(e) => {
            eprintln!("DB read failed: {:?}", e);
        }
    }
    match read_user_by_email(&pool, &email_address.trim()).await {
        Ok(existing_users) => {
            if !existing_users.is_empty() {
                return axum::Json("Email already exists".to_string());
            }
        },
        Err(e) => {
            eprintln!("DB read failed: {:?}", e);
        }
    }
    
    match create_user(&pool, &username.trim(), &email_address.trim(),&hash_password.trim()).await {
        Ok((user_id,Token)) => {
            println!("✅ SUCCESS: User created with ID: {} and token {}", user_id,Token);
            Json(format!("User created successfully with ID: {} and token {}", user_id,Token));
        },
        Err(e) => {
            eprintln!("❌ DB insert failed: {:?}", e);
            Json("Failed to create user".to_string());
        }
    }
    Json({
        
        combined_msg
    })
}


pub async  fn decode_user(Json(user): Json<BUSER>) -> Json<(String)>{
    let name = user.name;
    let email_address = user.email;
    let token = user.token;
    let secret = env::var("JWT_SECREAT").expect("JWT_SECREAT must be set");

    // let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secreat.as_ref())).unwrap();
    // let tokenm = decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default()).unwrap();
   match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(data) => {
            let msg = format!("Token valid for user: {}, exp: {}", data.claims.sub, data.claims.exp);
            axum::Json(msg.to_string());
        }
        Err(e) => {
            eprintln!("Token decode failed: {:?}", e);
            axum::Json("Invalid token".to_string());
        }
    }
    Json({
        
        "combined_msg".to_string()
    })
}


// pub async  fn decode_tokken(token:String) -> String{
//     let secreat = env::var("JWT_SECREAT").expect("JWT_SECREAT must be set");

// //     let expiration = SystemTime::now()
// //     .checked_add(Duration::from_secs(24 * 3600)) // 24 hours
// //     .unwrap()
// //     .duration_since(UNIX_EPOCH)
// //     .unwrap()
// //     .as_secs() as usize;
// // // sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
// // // .bind(name)
// // // .bind(email)
// // // .execute(pool)
// // // .await?;
// // let my_claims = Claims {
// //     sub: name.to_owned(),
// //     exp: expiration, // unix timestamp
// // };
    

// }
pub async  fn email_update(Json(user): Json<BUSER>) ->  Json<String>{

    let username = user.name;
    let email_address = user.email;

    if email_address.len() <= 3 {
        return axum::Json(axum::Json("This email address is not valid brother ").to_string());
    }

    if username.len() <= 3 && username.len() >= 18 {
        return axum::Json(Json(("{username} is to short should be  betwen 3 and 18 caracters ")).to_string());
    };
  

   match update_mail(&username,&email_address).await{
    Ok((bool)) => {
        println!("✅ Succes users email udated: {}", bool);
        return axum::Json(Json(json!("Updated the email  ")).to_string());

    }
    Err(e) =>{
        println!("Error found couldnt upadte users email {e}");
        return axum::Json(Json(json!("Couldn't update the email {e}  ")).to_string());

    }
   }

   
  
}

async fn hash_password(password: String) -> Result<String>{


    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)

} 

pub async  fn find_user_data(Json(user): Json<BUSER>) ->  Json<String>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");

    let username = user.name;
    match find_user_fullpool(&pool,&username,).await{
        Ok((user)) => {
            match read_user(&pool, 42).await {
                Ok(Some(u)) =>{ 
                    println!("user found: {:?}", u)

            },
                Ok(None)    =>{
                    
                     println!(" no user with that id");
                     return axum::Json(Json(json!({"result": "no user with that id", "User":user})).to_string());
                },
                Err(e)      => {eprintln!("database error: {e}"
             

            )},
            }

            

            println!(" Succes user data is : {:?}",user );
            return axum::Json(Json(json!({"result": "your user data is succesfuly retrived", "User":user})).to_string());
    
        }
        Err(e) =>{
            println!("Error found couldnt  users  {e}");
            return axum::Json(Json(json!("Couldn't find user  {e}  ")).to_string());
    
        }
}
}



pub     async  fn connect_database() ->Result<()>{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");



    println!("Connected to database");

    // println!("{ress:?}");  
    Ok(()  )
}

#[derive(sqlx::FromRow, Deserialize, Debug,Serialize)]
pub struct BUSER {
    #[serde(default)]          // id will be 0 when absent
    pub id:    i32,
    pub name:  String,
    pub email: String,
    pub password: String,
    pub token : String,

}


pub async fn create_user(pool: &sqlx::PgPool, name: &str, email: &str,password: &str) -> Result<(i32,String), sqlx::Error> {


    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(24 * 3600)) // 24 hours
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    // sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
    // .bind(name)
    // .bind(email)
    // .execute(pool)
    // .await?;
    let my_claims = Claims {
        sub: name.to_owned(),
        exp: expiration, // unix timestamp
    };
    let secreat = env::var("JWT_SECREAT").expect("JWT_SECREAT must be set");

    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret(secreat.as_ref())).unwrap();

    let user_id = sqlx::query_scalar::<_, i32>("INSERT INTO users (name, email,password) VALUES ($1, $2, $3) RETURNING id")
    .bind(name)
    .bind(email)
    .bind(password)
    .fetch_one(pool)
    .await?;
println!("User created with ID: {}, name: {}, email: {}, hased password {}", user_id, name, email,password);

    println!("user of {name} {email} created ");
Ok((user_id,token))

}

pub async fn delete_user_main(Json(req): Json<DeleteReq>) -> Json<Value> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    match delete_user(&pool, req.id).await {
        Ok(true)  => Json(json!({ "status": "ok",    "message": "user deleted" })),
        Ok(false) => Json(json!({ "status": "error", "message": "user not found"})),
        Err(e)    => Json(json!({ "status": "error", "message": format!("db error: {e}") })),
    }
}
pub async  fn update_mail( name: &str, email: &str) -> Result<(bool), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Failed to create pool");

match read_user_by_email(&pool, email).await {
    Ok(users) => {
        if users.is_empty() {
            println!("No users found with email: {}", email);
        } else {
            println!("Found {} user(s) with email '{}':", users.len(), email);
            for user in users {
                println!("  - ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
            }
        }
    },
    Err(e) => {
        eprintln!("Error finding user by email: {:?}", e);
    }
}
  let result =   update_user_email(&pool,name,email).await?;


  if result == false{
    Ok((false))

  } else{
    Ok((true))

  }



}



pub async  fn read_user(pool:&sqlx::PgPool,user_id:i32) -> Result<Option<BUSER>, sqlx::Error>{

    let user = sqlx::query_as::<_,BUSER>("SELECT * FROM users WHERE id = $1")
    .bind(user_id)
    .fetch_optional(pool)
    .await?;


    Ok(user)

}

pub async  fn read_user_by_name(pool:&sqlx::PgPool,name:&str) -> Result<Vec<BUSER>, sqlx::Error>{

    let user = sqlx::query_as::<_,BUSER>("SELECT * FROM users WHERE name = $1")
    .bind(name)
    // .fetch_one(pool)
    .fetch_all(pool)    
    .await?;

 

    Ok(user)

}

pub async  fn read_user_by_email(pool:&sqlx::PgPool,email:&str) -> Result<Vec<BUSER>, sqlx::Error>{
    let user = sqlx::query_as::<_,BUSER>("SELECT * FROM users WHERE email = $1")
    .bind(email)
    // .fetch_one(pool)
    .fetch_all(pool)    
    .await?;

 

    Ok(user)

}
async fn find_user_by_email(pool: &sqlx::PgPool, email: &str) {
    match read_user_by_email(pool, email).await {
        Ok(users) => {
            if users.is_empty() {
                println!("No users found with email: {}", email);
            } else {
                println!("Found {} user(s) with email '{}':", users.len(), email);
                for user in users {
                    println!("  - ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
                }
            }
        },
        Err(e) => {
            eprintln!("Error finding user by email: {:?}", e);
        }
    }
}


async fn find_user_by_name(pool: &sqlx::PgPool, name: &str) {
    match read_user_by_name(pool, name).await {
        Ok(users) => {
            if users.is_empty() {
                println!("No users found with name: {}", name);
            } else {
                println!("Found {} user(s) with name '{}':", users.len(), name);
                for user in users {
                    println!("  - ID: {}, Name: {}, Email: {}", user.id, user.name, user.email);
                }
            }
        },
        Err(e) => {
            eprintln!("Error finding user by name: {:?}", e);
        }
    }
}


pub async fn delete_user(pool: &sqlx::PgPool, user_id: i32) -> Result<(bool), sqlx::Error> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
    .bind(user_id)
    .execute(pool)
    .await?;

Ok(result.rows_affected() > 0)
}
pub async fn update_user_email(pool: &sqlx::PgPool, name: &str, new_email: &str) -> Result<(bool), sqlx::Error> {

   let result =  sqlx::query("UPDATE users SET email = $1 WHERE name = $2")
        .bind(new_email)
        .bind(name)
        .execute(pool)
        .await?;
    
    Ok((result.rows_affected() > 0))
}

pub async  fn find_user_fullpool(pool : &sqlx::PgPool, name: &str)-> Result<Option<BUSER>, sqlx::Error>{ // Vec<Buser> was here before
    let users =  sqlx::query_as::<_, BUSER>("SELECT * FROM users WHERE name = $1")
    .bind(name)
    .fetch_optional(pool)
    .await?;

    Ok(users)
}


#[derive(Deserialize)]
pub struct DeleteReq {
    id: i32,
}

