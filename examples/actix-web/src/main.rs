use std::{collections::HashMap, sync::RwLock};
use better_panic;

use actix_web;

mod user;

#[actix_web::main]
async fn main() {
    better_panic::Settings::new().verbosity(better_panic::Verbosity::Full).install();
    let db = std::sync::Arc::new(RwLock::new(HashMap::<String, String>::new()));
    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(
                tlns_google_oauth2::GoogleOAuth2Client::new(
                    std::env::var("GOOGLE_CLIENT_ID").unwrap(),
                    std::env::var("GOOGLE_CLIENT_SECRET").unwrap(),
                    "https://localhost:12700".to_string(),
                )
                .expect("Failed to build client"),
            ))
            .app_data(actix_web::web::Data::new(db.clone()))
            .service(user::user_auth)
            .service(user::user_cb)
            .service(user::user_get_name)
    })
    .bind("0.0.0.0:12700")
    .unwrap()
    .run();
    server.await.unwrap();
}
