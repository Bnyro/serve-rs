use clap::Parser;
use actix_files as fs;
use actix_web::{App, HttpServer};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value = ".")]
    dir: String,
    #[arg(short, long, default_value = "127.0.0.1")]
    address: String,
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
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

    println!("\nListening on {}:{}", "http://".to_string() + args.address.replace("http://", "").as_str(), args.port);
    HttpServer::new(move || {
        let files = fs::Files::new(args.base.as_str(), args.dir.as_str());
        let service = match args.list.as_str() {
            "true" => files.show_files_listing(),
            "false" => files,
            _ => unreachable!()
        };
        App::new().service(service)
    })
        .bind((args.address, args.port))?
        .run()
        .await
}
