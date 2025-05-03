use actix_web::{web, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use oauth2::TokenResponse;

#[actix_web::get("/auth/{user_id}")]
async fn user_auth(
    oauth_client: web::Data<tlns_google_oauth2::GoogleOAuth2Client>,
    user_id: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let auth = oauth_client
        .authorize_url(
            None,
            vec![&tlns_google_oauth2::grouped_scopes::GoogleOAuth2APIv2::AuthUserinfoProfile],
        )
        .map_err(|_| "Failed to build authorization URL")
        .unwrap();
    let mut c = actix_web::cookie::Cookie::new("user_id", user_id.clone());
    c.make_permanent();
    c.set_path("/callback");
    c.set_secure(Some(false));
    Ok(HttpResponse::Found()
        .append_header(("Location", auth.0))
        .cookie(c)
        .finish())
}

#[actix_web::get("/callback")]
async fn user_cb(
    oauth_client: web::Data<tlns_google_oauth2::GoogleOAuth2Client>,
    db: web::Data<Arc<RwLock<HashMap<String, String>>>>,
    queries: web::Query<HashMap<String, String>>,
    req: actix_web::HttpRequest,
) -> impl Responder {
    let code = queries.get("code").expect("No code authentication?");
    let token = oauth_client
        .get_token(code.to_string(), None)
        .await
        .expect("Google OAuth2 server sent a not successful response");
    let user_id = req
        .cookie("user_id")
        .expect("No cookie stored")
        .value()
        .to_string();
    req.cookie("user_id").unwrap().make_removal();
    let name = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v1/userinfo")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .expect("Failed to send the HTTP request")
        .error_for_status()
        .expect("Sent a non successful response for requesting a name");
    let jsonized: serde_json::Value = name.json().await.unwrap();
    let rname = jsonized["name"].as_str().or(None);
    match rname {
        Some(n) => {
            db.write().unwrap().entry(user_id.clone()).or_insert(n.to_string());
            HttpResponse::Found()
                .append_header(("Location", "http://localhost:5500"))
                .cookie({
                    let mut c = req.cookie("user_id").unwrap();
                    c.make_removal();
                    c
                })
                .body("Successfully authenticated! Please enjoy!")
        }
        None => HttpResponse::FailedDependency().body("Sent an invalid json"),
    }
}

#[actix_web::get("/{user_id}")]
async fn user_get_name(
    user_id: web::Path<String>,
    db: web::Data<Arc<RwLock<HashMap<String, String>>>>,
) -> String {
    // life time stuffs (painful)
    let x = match db.read().unwrap().get(&user_id.to_string()) {
        Some(v) => v.to_string(),
        None => "a".to_string(),
    };
    x
}
