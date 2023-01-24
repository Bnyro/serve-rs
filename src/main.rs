pub mod directory;

use std::{path::{Path, PathBuf}, fs::{self}};

use clap::{Parser, ArgAction, ValueHint};
use actix_web::{App, HttpServer, web, HttpResponse, get, http::header::ContentType};

use crate::directory::directory_listing;

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    #[arg(short, long, default_value = "/")]
    base: String,
    #[arg(short, long, action = ArgAction::SetFalse)]
    list: bool,
    #[arg(value_hint = ValueHint::DirPath)]
    directory: Option<String>,
}

#[get("/{filename:.*}")]
async fn index(filename: web::Path<String>, args: web::Data<Cli>) -> HttpResponse {
    let path: PathBuf = filename.parse().unwrap();
    let base = PathBuf::from(args.directory.clone().unwrap_or(String::from(".")));
    let target = base.join(path.clone());

    let mut content_type = ContentType::plaintext().to_string();
    let mut content = String::from("Not Found");
    if target.is_file() {
        match mime_guess::from_path(target.clone()).first() {
            Some(ct) => {
                content_type = ct.to_string();
            },
            None => {}
        }
        content = fs::read_to_string(target).expect("Can't read file!");
    } else if target.is_dir() {
        content_type = ContentType::html().to_string();
        content = String::from(directory_listing(target));
    }
    return HttpResponse::Ok().content_type(content_type).body(content);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();
    let app_state = args.clone();

    ctrlc::set_handler(|| {
        println!("\nShutting down live server. See you later!");
        std::process::exit(exitcode::OK);
    }).expect("Error setting Crtl-C handler");

    let path = args.directory.clone().unwrap_or(String::from("."));
    if !Path::new(path.clone().as_str()).exists() {
        println!("\nProvided path does not exits. Shutting down!");
        std::process::exit(exitcode::OSFILE);
    }

    println!("\nStarted live server at {}:{}", "http://".to_string() + args.address.replace("http://", "").as_str(), args.port);
    HttpServer::new(move || {
        App::new().app_data(web::Data::new(app_state.clone())).service(index)
    })
        .bind((args.address, args.port))?
        .run()
        .await
}
