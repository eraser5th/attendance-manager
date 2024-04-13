use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;

use std::sync::Arc;

use actix_web::{error::ErrorBadRequest, post, web, Error, HttpResponse};
use dotenv::dotenv;
use line_bot_sdk_rust::{
    client::LINE,
    line_messaging_api::{
        apis::MessagingApiApi,
        models::{Message, ReplyMessageRequest, TextMessage},
    },
    line_webhook::models::{CallbackRequest, Event, MessageContent},
    parser::signature::validate_signature,
    support::actix::Signature,
};

#[derive(Clone)]
struct AppState {
    line_channel_secret: Arc<String>,
    line_channel_access_token: Arc<String>,
}

#[post("/callback")]
async fn callback(data: web::Data<AppState>, signature: Signature, bytes: web::Bytes) -> Result<HttpResponse, Error> {
    // Get channel secret and access token by environment variable
    let channel_secret = data.line_channel_secret.get(0..).unwrap();
    let access_token = data.line_channel_access_token.get(0..).unwrap();

    let line = LINE::new(access_token.to_string());

    let body: &str = &String::from_utf8(bytes.to_vec()).unwrap();

    if !validate_signature(channel_secret.to_string(), signature.key, body.to_string()) {
        return Err(ErrorBadRequest("x-line-signature is invalid."));
    }

    let request: Result<CallbackRequest, serde_json::Error> = serde_json::from_str(&body);
    match request {
        Err(err) => return Err(ErrorBadRequest(err.to_string())),
        Ok(req) => {
            println!("req: {req:#?}");
            for e in req.events {
                if let Event::MessageEvent(message_event) = e {
                    if let MessageContent::TextMessageContent(text_message) = *message_event.message
                    {
                        let reply_message_request = ReplyMessageRequest {
                            reply_token: message_event.reply_token.unwrap(),
                            messages: vec![Message::Text(TextMessage::new(text_message.text))],
                            notification_disabled: Some(false),
                        };
                        let result = line
                            .messaging_api_client
                            .reply_message(reply_message_request)
                            .await;
                        match result {
                            Ok(r) => println!("{:#?}", r),
                            Err(e) => println!("{:#?}", e),
                        }
                    };
                };
            }
        }
    }

    Ok(HttpResponse::Ok().body("ok"))
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let channel_secret = secrets.get("LINE_CHANNEL_SECRET").expect("Failed to get LINE_CHANNEL_SECRET");
    let access_token = secrets.get("LINE_CHANNEL_ACCESS_TOKEN").expect("Failed to get LINE_CHANNEL_ACCESS_TOKEN");
    let app_state = AppState {
        line_channel_secret: Arc::new(channel_secret),
        line_channel_access_token: Arc::new(access_token),
    };

    dotenv().ok();
    let config = move |cfg: &mut ServiceConfig| {
        cfg
            .app_data(web::Data::new(app_state.clone()))
            .service(callback);
    };

    Ok(config.into())
}
