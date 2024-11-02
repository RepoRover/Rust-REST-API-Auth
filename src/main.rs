use actix_web::{web, App, HttpServer, Responder};

async fn say_hello() -> impl Responder {
    "Hello World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server: actix_web::dev::Server =
        HttpServer::new(|| App::new().route("/signup", web::get().to(say_hello)))
            .bind("127.0.0.1:3000")?
            .run();

    println!("Server is listening to incoming requests...");

    server.await
}
