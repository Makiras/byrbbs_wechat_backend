use actix_web::client::Client;
use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::{CONF};

static REDIRECT_URI : &str = "https://bbs.byrio.work/api/callback";
static GRANT_TYPE : &str = "authorization_code";

#[derive(Serialize, Deserialize, Debug)]
pub struct CallBackQuery {
    code: String,
}

#[get("/callback")]
pub async fn callback(web::Query(info): web::Query<CallBackQuery>) -> HttpResponse {
    let req_code = info.code;

    let client = Client::new();
    let params_arr = [
        ("client_id", CONF.key.client_id.as_ref().unwrap()),
        ("client_secret", CONF.key.client_secret.as_ref().unwrap()),
        ("code", &req_code),
        ("redirect_uri", &REDIRECT_URI.to_string()),
        ("grant_type", &GRANT_TYPE.to_string()),
    ];
    let mut response = client
        .post("https://bbs.byr.cn/oauth2/token")
        .send_form(&params_arr)
        .await
        .unwrap();
    let response_body = response.body().await.unwrap();
    println!("{:?},{:?}", &req_code, response_body);
    HttpResponse::Ok().body(response_body)
}
