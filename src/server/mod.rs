use crate::controller::{AppError, ErrorKind, Result};
use crate::var::environment::{constants::*, get_var_name, load_app_env};

use actix_web::{dev::Server, web, App, HttpServer};
use std::env;

pub async fn run() -> Result<()> {
    load_app_env()?;

    let address = format!(
        "{}:{}",
        env::var(get_var_name(APP_ADDRESS)).unwrap(),
        env::var(get_var_name(APP_PORT)).unwrap()
    );

    let server: Server = match HttpServer::new(|| {
        App::new().app_data(web::Data::new(AppStateData {
            db_connection: "This is the connection to DB".to_string(),
        }))
    })
    .bind(&address)
    {
        Ok(server) => {
            println!(
                "Server is listening for incoming requests on {}...",
                &address
            );
            server.run()
        }
        Err(err) => {
            return Err(AppError::new(
                ErrorKind::AddressBindingFail(format!(
                    "Failed to bind server to address: {}",
                    &address
                )),
                Some(Box::new(err)),
            ))
        }
    };

    if let Err(err) = server.await {
        return Err(AppError::new(
            ErrorKind::HttpServerFail,
            Some(Box::new(err)),
        ));
    }

    Ok(())
}

// ///////////////////////////
struct AppStateData {
    db_connection: String,
}

// async fn signup(app_state_data: web::Data<AppStateData>) -> impl Responder {
//     format!(
//         "signup, {}, and this is from strings: {}",
//         app_state_data.db_connection,
//         strings::HELLO_MESSAGE
//     )
// }

// async fn login() -> impl Responder {
//     "login"
// }

// async fn logout() -> impl Responder {
//     "logout"
// }

// async fn delete_user() -> impl Responder {
//     "delete_user"
// }

// async fn renew_user_session() -> impl Responder {
//     "renew_user_session"
// }

// let db_url: &str = "";
// let pool = sqlx::PgPool::connect(db_url).await.unwrap();
// println!("POOL: {:?}", pool);

// let res = sqlx::migrate!("./migrations/main").run(&pool).await;

// println!("MIGRATIONS: {:?}", res);

// .route("/signup", web::post().to(signup))
// .route("/login", web::put().to(login))
// .route("/logout", web::put().to(logout))
// .route("/user", web::delete().to(delete_user))
// .route("/session", web::put().to(renew_user_session))
