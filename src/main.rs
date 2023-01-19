use clap::Parser;
use actix_files as fs;
use actix_web::{App, HttpServer};

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    path: String,
    #[arg(short, long, default_value = "/")]
    base: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    ctrlc::set_handler(|| {
        println!("\nShutting down live server. See you later!");
        std::process::exit(exitcode::OK);
    }).expect("Error setting Crtl-C handler");

    HttpServer::new(move || App::new().service(fs::Files::new(args.base.as_str(), args.path.as_str()).show_files_listing()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
