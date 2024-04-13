
use line_bot_sdk_rust::client::LINE;
use line_bot_sdk_rust::line_messaging_api::apis::MessagingApiApi;
use line_bot_sdk_rust::line_messaging_api::models::Message;
use line_bot_sdk_rust::line_messaging_api::models::ReplyMessageRequest;
use line_bot_sdk_rust::line_messaging_api::models::TextMessage;
use line_bot_sdk_rust::line_webhook::models::CallbackRequest;
use line_bot_sdk_rust::line_webhook::models::Event;
use line_bot_sdk_rust::line_webhook::models::MessageContent;

pub async fn request_handler(reqest: CallbackRequest, line: LINE) {
    println!("req: {reqest:#?}");
    for e in reqest.events {
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
}
