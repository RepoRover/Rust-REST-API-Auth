use actix_web::{web, App, HttpServer, Responder};

struct AppStateData {
    db_connection: String,
}

async fn signup(app_state_data: web::Data<AppStateData>) -> impl Responder {
    format!("signup, {}", app_state_data.db_connection)
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
    let db_url: &str = "";
    let pool = sqlx::PgPool::connect(db_url).await.unwrap();
    println!("POOL: {:?}", pool);

    let res = sqlx::migrate!("./migrations/main").run(&pool).await;

    println!("MIGRATIONS: {:?}", res);

    let server: actix_web::dev::Server = HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppStateData {
                db_connection: "This is the connection to DB".to_string(),
            }))
            .route("/signup", web::post().to(signup))
            .route("/login", web::put().to(login))
            .route("/logout", web::put().to(logout))
            .route("/user", web::delete().to(delete_user))
            .route("/session", web::put().to(renew_user_session))
    })
    .bind("127.0.0.1:3000")?
    .run();

    println!("Server is listening for incoming requests...");

    server.await
}
