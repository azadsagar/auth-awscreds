use actix_web::{Responder,get,web,post};
use serde::Serialize;
use libc;

use crate::appdata::appdata::AppData;
use crate::crypt::crypt::decrypt_data;
use crate::iniconfig::configcreds::{AWSCreds,configcreds};

#[derive(Serialize)]
struct HTTPResponseData {
    status: u32,
    msg: String,
    success: bool,
    data: String
}

#[get("/publickey")]
pub async fn get_public_key(data: web::Data<AppData>) -> impl Responder {
    let public_key = &data.public_key;
    
    web::Json(HTTPResponseData{
        status: 200,
        msg: String::from("Ok"),
        success: true,
        data: String::from(public_key)
    })
}

#[post("/setcreds")]
pub async fn set_aws_creds(data: web::Data<AppData>,payload: web::Json<serde_json::Value>) -> impl Responder {
    let private_key = &data.private_key;
    let profile_name = &data.profile_name;

    let encrypted_data = payload["data"].as_array().unwrap();
    let username = payload["username"].as_str().unwrap();

    let mut decypted_payload = vec![];

    for str in encrypted_data.iter() {
        //println!("{}",str.to_string());
        let s = str.as_str().unwrap();
        let decrypted = decrypt_data(&private_key, &s.to_string());
        decypted_payload.extend_from_slice(&decrypted);
    }

    let credentials : serde_json::Value = serde_json::from_str(&String::from_utf8(decypted_payload).unwrap()).unwrap();

    let aws_creds = AWSCreds{
        profile_name: String::from(profile_name),
        aws_access_key_id: String::from(credentials["AccessKeyId"].as_str().unwrap()),
        aws_secret_access_key: String::from(credentials["SecretAccessKey"].as_str().unwrap()),
        aws_session_token: String::from(credentials["SessionToken"].as_str().unwrap())
    };

    println!("Authenticated as {}",username);
    println!("Updating AWS Credentials File...!");

    configcreds(&aws_creds);

    unsafe {
        libc::raise(libc::SIGINT);
    }

    web::Json(HTTPResponseData{
        status: 200,
        msg: String::from("Ok"),
        success: true,
        data: String::from("NA")
    })
    
}