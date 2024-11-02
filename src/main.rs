use actix_web::{web, App, HttpServer, Responder};

async fn signup() -> impl Responder {
    "signup"
}

async fn login() -> impl Responder {
    "login"
}

async fn logout() -> impl Responder {
    "logout"
}

async fn delete_user() -> impl Responder {
    "delete_user"
}

async fn renew_user_session() -> impl Responder {
    "renew_user_session"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server: actix_web::dev::Server = HttpServer::new(|| {
        App::new()
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login))
            .route("/logout", web::put().to(logout))
            .route("/user", web::delete().to(delete_user))
            .route("/session", web::put().to(renew_user_session))
    })
    .bind("127.0.0.1:3000")?
    .run();

    println!("Server is listening to incoming requests...");

    server.await
}
