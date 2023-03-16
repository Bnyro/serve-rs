pub mod directory;

use std::{path::{Path, PathBuf}, fs};

use clap::{Parser, ArgAction, ValueHint};
use actix_web::{App, HttpServer, web, HttpResponse, get, http::header::ContentType};

use crate::directory::directory_listing;
use lazy_static::lazy_static;

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
    #[arg(short, long)]
    index_path: Option<String>,
    #[arg(value_hint = ValueHint::DirPath)]
    directory: Option<String>,
}

lazy_static! {
    static ref ARGS: Cli = Cli::parse();
}

#[get("/{filename:.*}")]
async fn index(filename: web::Path<String>) -> HttpResponse {
    
    let base = PathBuf::from(ARGS.directory.clone().unwrap_or(String::from(".")));
    let target = if filename.to_string().is_empty() && ARGS.index_path.is_some() {
        base.join(ARGS.index_path.clone().unwrap())
    } else {
        let path: PathBuf = filename.parse().unwrap();
        base.join(path.clone())
    };

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
        match ARGS.list {
            true => {
                content_type = ContentType::html().to_string();
                content = String::from(directory_listing(target, base));
            },
            false => {
                return HttpResponse::NotFound().body("Page Not Found")
            }
        }
    }
    return HttpResponse::Ok().content_type(content_type).body(content);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ctrlc::set_handler(|| {
        println!("\nShutting down live server. See you later!");
        std::process::exit(exitcode::OK);
    }).expect("Error setting Crtl-C handler");

    let path = ARGS.directory.clone().unwrap_or(String::from("."));
    if !Path::new(path.clone().as_str()).exists() {
        println!("\nProvided path does not exits. Shutting down!");
        std::process::exit(exitcode::OSFILE);
    }

    println!("Started live server at {}:{}", "http://".to_string() + ARGS.address.replace("http://", "").as_str(), ARGS.port.clone());
    HttpServer::new(move || App::new().service(index))
        .bind((ARGS.address.clone(), ARGS.port))?
        .run()
        .await
}
