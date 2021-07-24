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

#[get("/callback")]
pub async fn callback(web::Query(info): web::Query<CallBackQuery>) -> HttpResponse {
    let req_code = info.code;

    let client = Client::builder()
        .max_http_version(http::Version::HTTP_11)
        .timeout(Duration::from_millis(1000))
        .finish();

    let params_arr = [
        ("client_id", CONF.key.client_id.as_ref().unwrap()),
        ("client_secret", CONF.key.client_secret.as_ref().unwrap()),
        ("code", &req_code),
        ("redirect_uri", &REDIRECT_URI.to_string()),
        ("grant_type", &GRANT_TYPE_AUTH.to_string()),
    ];
    let mut response = client
        .post("https://bbs.byr.cn/oauth2/token")
        .send_form(&params_arr)
        .await
        .unwrap();
    let response_body = response.body().await.unwrap();
    println!("{},{:?}", req_code, response_body);
    HttpResponse::Ok().body(response_body)
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
