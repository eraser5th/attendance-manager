use actix_web::error::ErrorBadRequest;
use actix_web::web;
use actix_web::Error;
use actix_web::HttpResponse;

use line_bot_sdk_rust::client::LINE;
use line_bot_sdk_rust::line_webhook::models::CallbackRequest;
use line_bot_sdk_rust::parser::signature::validate_signature;
use line_bot_sdk_rust::support::actix::Signature;

mod get_class_id_to_class_name;
mod get_events;
mod request_handler;

#[derive(Clone)]
pub struct LineBotEnv {
    pub line_channel_secret: String,
    pub line_channel_access_token: String,
}

pub async fn linebot(
    linebot_env: LineBotEnv,
    signature: Signature,
    bytes: web::Bytes,
) -> Result<HttpResponse, Error> {
    let line = LINE::new(linebot_env.line_channel_access_token);

    let body: &str = &String::from_utf8(bytes.to_vec()).unwrap();

    if !validate_signature(
        linebot_env.line_channel_secret,
        signature.key,
        body.to_string(),
    ) {
        return Err(ErrorBadRequest("x-line-signature is invalid."));
    }

    let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_str(&body);
    match request {
        Err(err) => return Err(ErrorBadRequest(err.to_string())),
        Ok(req) => {
            request_handler::request_handler(req, line).await;
            Ok(HttpResponse::Ok().body("ok"))
        }
    }
}
