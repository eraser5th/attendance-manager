use actix_web::web;
use actix_web::error::ErrorBadRequest;
use actix_web::Error;
use actix_web::HttpResponse;

use line_bot_sdk_rust::support::actix::Signature;
use line_bot_sdk_rust::client::LINE;
use line_bot_sdk_rust::line_messaging_api::apis::MessagingApiApi;
use line_bot_sdk_rust::line_messaging_api::models::Message;
use line_bot_sdk_rust::line_messaging_api::models::ReplyMessageRequest;
use line_bot_sdk_rust::line_messaging_api::models::TextMessage;
use line_bot_sdk_rust::line_webhook::models::CallbackRequest;
use line_bot_sdk_rust::line_webhook::models::Event;
use line_bot_sdk_rust::line_webhook::models::MessageContent;
use line_bot_sdk_rust::parser::signature::validate_signature;

#[derive(Clone)]
pub struct LineBotEnv {
    pub line_channel_secret: String,
    pub line_channel_access_token: String,
}

pub async fn linebot(linebot_env: LineBotEnv, signature: Signature, bytes: web::Bytes)  -> Result<HttpResponse, Error> {
    let line = LINE::new(linebot_env.line_channel_access_token);

    let body: &str = &String::from_utf8(bytes.to_vec()).unwrap();

    if !validate_signature(linebot_env.line_channel_secret, signature.key, body.to_string()) {
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
            };
            Ok(HttpResponse::Ok().body("ok"))
        }
    }
}
