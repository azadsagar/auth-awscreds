//use std::{fs,path::Path,env,sync::mpsc,thread};
use std::{fs,path::Path,env};
use actix_web::{HttpServer,App,web};
use actix_cors::Cors;
use std::io;
use std::io::Write;
//use futures::executor;

mod crypt;
mod appdata;
mod api;
mod iniconfig;

use crypt::crypt::{genkeypairs};
use api::api::{get_public_key,set_aws_creds};
use appdata::appdata::{AppData};
use iniconfig::configcreds::{read_profile,config_profile};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    /* std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init(); */

    let user_home_dir = format!("{}",env::var("HOME").unwrap());

    let aws_creds_dir: String = format!("{}/.aws",&user_home_dir);
    let aws_creds_dir_exist : bool = Path::new(&aws_creds_dir).is_dir();

    println!("Checking for the existince of aws dir...");

    if ! aws_creds_dir_exist {
        match fs::create_dir(&aws_creds_dir) {
            Ok(_) => println!("{} directory created !",aws_creds_dir),
            Err(error) => panic!("Unable to create directory : {}",error)
        };
    }

    println!("Reading configuration...");

    let app_profile_file = format!("{}/.auth-awscreds",&user_home_dir);

    let config_exist : bool = Path::new(&app_profile_file).exists();

    let mut profile_name = String::new();
    let mut app_domain = String::new();

    if !config_exist {
        //ask the series of questions
        print!("Which profile to write AWS Credentials [default] : ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut profile_name)
            .expect("Failed to read line");

        print!("App Domain : ");
        io::stdout().flush().unwrap();
        
        io::stdin()
            .read_line(&mut app_domain)
            .expect("Failed to read line");
        
        profile_name=String::from(profile_name.trim());
        app_domain=String::from(app_domain.trim());
        
        config_profile(&profile_name,&app_domain);
        
    }
    else {
        (profile_name,app_domain) = read_profile();
    }

    println!("Generating crypto keypairs...");

    let  (private_key , public_key) = genkeypairs();

    let crypt_keys = AppData{
        public_key: public_key.clone(),
        private_key: private_key.clone(),
        profile_name: profile_name
    };

    let crypto_data = web::Data::new(
        crypt_keys
    );

    //let (tx, rx) = mpsc::channel::<()>();
    println!("Opening web ui for authentication...!");
    open::that(&app_domain).unwrap();

    HttpServer::new(move || {
        //let stopper = tx.clone();
        let cors = Cors::permissive();
        App::new()
        .wrap(cors)
        //.app_data(stopper)
        .app_data(crypto_data.clone())
        .service(get_public_key)
        .service(set_aws_creds)
    })
    .bind(("127.0.0.1",63442))?
    .run()
    .await

    //let srv=server.clone();
    /* let handle = server.handle();


    thread::spawn(move || {
        rx.recv().unwrap();
        executor::block_on(handle.stop(true))
    });  */


}