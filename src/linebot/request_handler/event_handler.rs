use std::sync::Arc;
use std::boxed::Box;

use line_bot_sdk_rust::client::LINE;
use line_bot_sdk_rust::line_messaging_api::apis::MessagingApiApi;
use line_bot_sdk_rust::line_messaging_api::models::Message;
use line_bot_sdk_rust::line_messaging_api::models::ReplyMessageRequest;
use line_bot_sdk_rust::line_messaging_api::models::TextMessage;
use line_bot_sdk_rust::line_webhook::models::message_event::MessageEvent;
use line_bot_sdk_rust::line_webhook::models::MessageContent;
use line_bot_sdk_rust::line_webhook::models::source::Source;

use crate::infra::DB;
use crate::linebot::ChatContext;
use crate::linebot::ContextMode;

pub async fn message_event_handler(
    message_event: MessageEvent,
    line: &LINE,
    db: &mut Arc<DB>,
    chat_context: &mut Arc<ChatContext>,
) {
    if let MessageContent::TextMessageContent(text_message) = *message_event.message {
        let reply_message_request = match text_message.text.as_str() {
            "ical" => {
                let user_id = {
                    let s = message_event.source.unwrap();
                    let s = *s;
                    if let Source::UserSource(s) = s {
                        Ok(s.user_id)
                    } else {
                        Err(())
                    }
                };
                Arc::make_mut(chat_context).change_context(user_id.ok().unwrap().unwrap(), ContextMode::IcalRegist);
                

                ReplyMessageRequest {
                    reply_token: message_event.reply_token.unwrap(),
                    messages: vec![Message::Text(TextMessage::new("URLおしえてちょ".to_string()))],
                    notification_disabled: Some(false),
                }
            },
            _ => ReplyMessageRequest {
                reply_token: message_event.reply_token.unwrap(),
                messages: vec![Message::Text(TextMessage::new(text_message.text))],
                notification_disabled: Some(false),
            },
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
}
