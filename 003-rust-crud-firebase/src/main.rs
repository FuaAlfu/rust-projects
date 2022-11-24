use std::collections::HashMap;
use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User{
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response{
    name: String,
}

#[tokio::main]
async fn main(){
    let user = User{
        name: "fua alfu".to_string(),
        age: 29,
        email: "alfuGun@foo.com".to_string(),
    };

    //add the link of app, from your firebase account
    let firebase = Firebase::new("");

    let response = set_user(&firebase, &user.await);
    let mut user = get_user(&firebase, &response.name.await);
    println!("{:?}",user);

    let users = get_users(&firebase).await;
    println!("{:?}",users);

    user.email = "updated.mail@gmail.com".to_string();
    let updated_user = updated_user(&firebase, &response.name, &user).await;
    println!("{:?}",updated_user);

    delete_user(&firebase, &response.name).await;
    println!("User deleted..");
}

async fn set_user() -> Response{}

async fn get_users() -> HashMap<String, User>{}

async fn set_user() -> User{}

async fn update_user() -> User{}

async fn delete_user(){}

//convert a string to a response
fn string_to_response(s: &str) -> Response{
    serde_json::from_str(s).unwarp()
}

//convert a string to a user
fn string_to_user(s: &str) -> User{
    serde_json::from_str(s).unwarp()
}