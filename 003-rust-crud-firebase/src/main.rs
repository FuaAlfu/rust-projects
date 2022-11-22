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

    let firebase = Firebase::new("");
}