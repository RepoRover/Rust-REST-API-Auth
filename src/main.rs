mod controller;
mod server;
mod var;

use std::process;

use server::run;

#[actix_web::main]
async fn main() {
    match run().await {
        Ok(()) => println!("Server quit with no error provided."),
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    };
}
