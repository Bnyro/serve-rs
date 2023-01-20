use std::path::Path;

use clap::{Parser, ArgAction, ValueHint};
use actix_files as fs;
use actix_web::{App, HttpServer};

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
    HttpServer::new(move || {
        let files = fs::Files::new(args.base.as_str(), path.as_str());
        let service = match args.list {
            true => files.show_files_listing(),
            false => files,
        };
        App::new().service(service)
    })
        .bind((args.address, args.port))?
        .run()
        .await
}
