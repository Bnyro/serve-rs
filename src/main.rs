use clap::Parser;
use actix_files as fs;
use actix_web::{App, HttpServer};

#[derive(Parser)]
struct Cli {
    path: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    ctrlc::set_handler(|| {
        println!("\nShutting down live server. See you later!");
        std::process::exit(0x0100);
    }).expect("Error setting Crtl-C handler");

    HttpServer::new(move || App::new().service(fs::Files::new("/", args.path.as_str()).show_files_listing()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
