use crate::{chat::RoomSocket, http::db::Database};

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie, http::header, middleware::Logger, web, App, HttpResponse, HttpServer};
use oauth2::basic::BasicClient;
use tokio::sync::mpsc;
use std::time::Duration;

mod auth;
mod user;
mod room;
mod error;

pub async fn start_actix_server(
    db: Database,
    auth_client: BasicClient,
    room_update_sender: mpsc::Sender<RoomSocket>,
) {
    // get cookie key from enviroment
    let env_key = std::env::var("SESSION_KEY").expect("SESSION_KEY must be set");
    let secret_key = cookie::Key::from(env_key.as_bytes());

    // Start the Actix Web server
    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(auth_client.clone()))
            .app_data(web::Data::new(room_update_sender.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(IdentityMiddleware::builder()
					.login_deadline(Some(Duration::from_micros(120000)))
					.visit_deadline(Some(Duration::from_micros(120000)))
					.build(),
			)
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                    .cookie_secure(false)
                    .build(),
            )
            .service(
                web::resource("/health")
                    .route(web::get().to(|| async { HttpResponse::Ok().json("I am alive!") })),
            )
            .service(auth::login)
            .service(auth::logout)
            .service(auth::callback)
            .service(user::home)
            .service(user::all)
            .service(user::get)
			.service(user::post)
			.service(user::rooms_get)
			.service(room::all)
			.service(room::get)
			.service(room::post)
            .default_service(web::to(|| HttpResponse::NotFound()))
			// .service(auth::login_test)
				// .service(user::user_get_test)
				// .service(user::user_post)
    })
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to port 8080")
    .run()
    .await;
}
