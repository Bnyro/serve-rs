use std::{path::{Path, PathBuf}, fs};

use clap::{Parser, ArgAction, ValueHint};
use actix_web::{App, HttpServer, web, HttpRequest, HttpResponse};

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

async fn index(req: HttpRequest) -> HttpResponse {
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    let base = PathBuf::from("/home/bnyro/Projects/bnyro.github.io/");
    let target = base.join(path);
    if target.is_file() {
        let content_type = mime_guess::from_path(target.clone()).first().expect("No Mime Type found");
        let content = fs::read_to_string(target).expect("Can't read file!");
        return HttpResponse::Ok().content_type(content_type).body(content);
    } else if target.is_dir() {
        return HttpResponse::Ok().body(String::from("Directory, todo"));
    }
    HttpResponse::NotFound().body("Not Found")
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
        App::new().route("/{filename:.*}", web::get().to(index))
    })
        .bind((args.address, args.port))?
        .run()
        .await
}
