use clap::Parser;
use actix_files as fs;
use actix_web::{App, HttpServer};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    path: String,
    #[arg(short, long, default_value = "/")]
    base: String,
    #[arg(short, long, default_value = "true")]
    list: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    ctrlc::set_handler(|| {
        println!("\nShutting down live server. See you later!");
        std::process::exit(exitcode::OK);
    }).expect("Error setting Crtl-C handler");

    HttpServer::new(move || {
        let files = fs::Files::new(args.base.as_str(), args.path.as_str());
        let service = match args.list.as_str() {
            "true" => files.show_files_listing(),
            "false" => files,
            _ => unreachable!()
        };
        App::new().service(service)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
