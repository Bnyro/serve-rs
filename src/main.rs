pub mod directory;

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use actix_web::{get, http::header::ContentType, web, App, HttpResponse, HttpServer};
use clap::{ArgAction, Parser, ValueHint};

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
    let path = if filename.to_string().is_empty() && ARGS.index_path.is_some() {
        ARGS.index_path.clone().unwrap()
    } else {
        filename.parse().unwrap()
    };
    let target = base.join(path);

    let mut content_type = ContentType::plaintext().to_string();
    let mut response_bytes: Vec<u8> = String::from("Not Found").into_bytes();
    if target.is_file() {
        if let Some(ct) = mime_guess::from_path(target.clone()).first() {
            content_type = ct.to_string();
        }
        let mut file_content = Vec::new();
        File::open(target)
            .expect("Unable to open file")
            .read_to_end(&mut file_content)
            .expect("Can't read file!");
        response_bytes = file_content;
    } else if target.is_dir() {
        match ARGS.list {
            true => {
                content_type = ContentType::html().to_string();
                response_bytes = directory_listing(target, base).into_bytes();
            }
            false => return HttpResponse::NotFound().body("Page Not Found"),
        }
    }
    return HttpResponse::Ok()
        .content_type(content_type)
        .body(response_bytes);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    ctrlc::set_handler(|| {
        println!("\nShutting down live server. See you later!");
        std::process::exit(exitcode::OK);
    })
    .expect("Error setting Crtl-C handler");

    let path = ARGS.directory.clone().unwrap_or(String::from("."));
    if !Path::new(path.clone().as_str()).exists() {
        println!("\nProvided path does not exits. Shutting down!");
        std::process::exit(exitcode::OSFILE);
    }

    println!(
        "Started live server at {}:{}",
        "http://".to_string() + ARGS.address.replace("http://", "").as_str(),
        ARGS.port.clone()
    );
    HttpServer::new(move || App::new().service(index))
        .bind((ARGS.address.clone(), ARGS.port))?
        .run()
        .await
}
