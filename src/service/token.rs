use actix_web::client::Client;
use actix_web::{get, http, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::CONF;

static REDIRECT_URI: &str = "https://bbs.byrio.work/api/callback";
static GRANT_TYPE_AUTH: &str = "authorization_code";
static GRANT_TYPE_REFRESH: &str = "refresh_token";

#[derive(Serialize, Deserialize, Debug)]
pub struct CallBackQuery {
    code: String,
}

async fn get_access_token(code: String) -> (actix_web::http::StatusCode, actix_web::web::Bytes) {
    let client = Client::builder()
        .max_http_version(http::Version::HTTP_11)
        .timeout(Duration::from_millis(1000))
        .finish();

    let params_arr = [
        ("client_id", CONF.key.client_id.as_ref().unwrap()),
        ("client_secret", CONF.key.client_secret.as_ref().unwrap()),
        ("code", &code),
        ("redirect_uri", &REDIRECT_URI.to_string()),
        ("grant_type", &GRANT_TYPE_AUTH.to_string()),
    ];
    let mut response = client
        .post("https://bbs.byr.cn/oauth2/token")
        .send_form(&params_arr)
        .await
        .unwrap();
    match response.body().await {
        Ok(body) => (response.status(), body),
        Err(e) => {
            println!("{:?}", e);
            (
                http::StatusCode::INTERNAL_SERVER_ERROR,
                actix_web::web::Bytes::from(e.to_string()),
            )
        }
    }
}

#[get("/callback")]
pub async fn callback_get(web::Query(info): web::Query<CallBackQuery>) -> HttpResponse {
    let req_code = info.code;
    let (status, body) = get_access_token(req_code).await;
    HttpResponse::with_body(status, actix_web::dev::Body::Bytes(body))
}

#[post("/callback")]
pub async fn callback_post(web::Query(info): web::Query<CallBackQuery>) -> HttpResponse {
    let req_code = info.code;
    let (status, body) = get_access_token(req_code).await;
    HttpResponse::with_body(status, actix_web::dev::Body::Bytes(body))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshQuery {
    refresh_token: String,
}

#[post("/refresh")]
pub async fn refresh(web::Form(info): web::Form<RefreshQuery>) -> HttpResponse {
    let client = Client::builder()
        .max_http_version(http::Version::HTTP_11)
        .timeout(Duration::from_millis(1000))
        .finish();
    let params_arr = [
        ("client_id", CONF.key.client_id.as_ref().unwrap()),
        ("client_secret", CONF.key.client_secret.as_ref().unwrap()),
        ("refresh_token", &info.refresh_token),
        ("grant_type", &GRANT_TYPE_REFRESH.to_string()),
    ];
    let mut response = client
        .post("https://bbs.byr.cn/oauth2/token")
        .send_form(&params_arr)
        .await
        .unwrap();
    let response_body = response.body().await.unwrap();
    println!("{},{:?}", info.refresh_token, response_body);
    HttpResponse::Ok().body(response_body)
}
