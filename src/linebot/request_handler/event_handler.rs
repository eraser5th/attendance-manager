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
use crate::infra::User;
use crate::linebot::ChatContext;
use crate::linebot::ContextMode;

pub async fn message_event_handler(
    message_event: MessageEvent,
    line: &LINE,
    db: &mut Arc<DB>,
    chat_context: &mut Arc<ChatContext>,
) {
    let user_id = {
        let s = message_event.source.unwrap();
        let s = *s;
        if let Source::UserSource(s) = s {
            Ok(s.user_id)
        } else {
            Err(())
        }.ok().unwrap().unwrap()
    };

    if let MessageContent::TextMessageContent(text_message) = *message_event.message {
        let mode = chat_context.contexts.get(&user_id).unwrap_or(&ContextMode::Nomal);
        let reply_message_request = match mode {
            ContextMode::Nomal => {
                match text_message.text.as_str() {
                    "ical" => {
                        Arc::make_mut(chat_context).change_context(user_id, ContextMode::IcalRegist);

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
                }
            },
            ContextMode::IcalRegist => {
                println!("icalregist");
                let _ = Arc::make_mut(db).update_user(user_id.clone(), User {
                    id: user_id.clone(),
                    ical_url: Some(text_message.text.clone()),
                });
                let user = Arc::make_mut(db).get_user(user_id.clone());
                println!("{user:?}");
                ReplyMessageRequest {
                    reply_token: message_event.reply_token.unwrap(),
                    messages: vec![Message::Text(TextMessage::new("ical登録したよ".to_string()))],
                    notification_disabled: Some(false),
                }
            },
        };
        let result = line
            .messaging_api_client
            .reply_message(reply_message_request)
        .await;
    };
}
