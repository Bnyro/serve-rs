use std::{path::{Path, PathBuf}, fs::{self}};

use clap::{Parser, ArgAction, ValueHint};
use actix_web::{App, HttpServer, web, HttpResponse, get, http::header::ContentType};

#[derive(Parser, Debug)]
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
async fn index(filename: web::Path<String>) -> HttpResponse {
    let path: PathBuf = filename.parse().unwrap();
    let base = PathBuf::from("/home/bnyro/Projects/bnyro.github.io/");
    let target = base.join(path.clone());

    let mut content_type = ContentType::plaintext().to_string();
    let mut content = String::from("Not Found");
    if target.is_file() {
        content_type = mime_guess::from_path(target.clone()).first().expect("No Mime Type found").to_string();
        content = fs::read_to_string(target).expect("Can't read file!");
    } else if target.is_dir() {
        content_type = ContentType::html().to_string();
        content = String::from(directory_listing(target));
    }
    return HttpResponse::Ok().content_type(content_type).body(content);
}

fn directory_listing(base: PathBuf) -> String {
    let children = base.read_dir().unwrap();

    let mut body = String::new();

    for entry in children {
            let entry = entry.unwrap();
            let path = entry.path();
            let p = match path.strip_prefix(&base) {
                Ok(p) if cfg!(windows) => base.join(p).to_string_lossy().replace('\\', "/"),
                Ok(p) => base.join(p).to_string_lossy().into_owned(),
                Err(_) => continue,
            };

            if path.is_dir() {
                body += &format!(
                    "<li><a href=\"{}\">{}/</a></li>",
                    p,
                    entry.file_name().to_string_lossy(),
                ).to_string();
            } else {
                body += &format!(
                    "<li><a href=\"{}\">{}</a></li>",
                    p,
                    entry.file_name().to_string_lossy(),
                ).to_string();
            }
        }

    format!(
        "<html>\
         <head><title>{}</title></head>\
         <body><h1>{}</h1>\
         <ul>\
         {}\
         </ul></body>\n</html>",
        "", "", body
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();

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
    HttpServer::new(|| {
        App::new().service(index)
    })
        .bind((args.address, args.port))?
        .run()
        .await
}
