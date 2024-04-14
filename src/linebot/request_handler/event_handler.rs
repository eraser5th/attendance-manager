use line_bot_sdk_rust::client::LINE;
use line_bot_sdk_rust::line_messaging_api::apis::MessagingApiApi;
use line_bot_sdk_rust::line_messaging_api::models::Message;
use line_bot_sdk_rust::line_messaging_api::models::ReplyMessageRequest;
use line_bot_sdk_rust::line_messaging_api::models::TextMessage;
use line_bot_sdk_rust::line_webhook::models::message_event::MessageEvent;
use line_bot_sdk_rust::line_webhook::models::MessageContent;
use line_bot_sdk_rust::line_webhook::models::source::Source;

use crate::linebot::create_text::create_text;
use crate::linebot::get_events::get_events;

fn get_user_id(message_event: &MessageEvent) -> Result<String, ()> {
    let s = message_event.source.clone().unwrap();
    let s = *s;
    if let Source::UserSource(s) = s {
        Ok(s.user_id.unwrap_or("".to_string()))
    } else {
        Err(())
    }
}

pub async fn message_event_handler(message_event: MessageEvent, line: &LINE) {
    // let user_id = get_user_id(&message_event);
    let MessageContent::TextMessageContent(text_message) = *message_event.message else {
        return
    };
    let Some(reply_token) = message_event.reply_token else {
        return
    };

    let reply_message_request = match text_message.text.as_str() {
        "ical" => ReplyMessageRequest {
            reply_token: reply_token.clone(),
            messages: vec![Message::Text(TextMessage::new("URLちょーだい".to_string()))],
            notification_disabled: Some(false),
        },
        url => {
            let events = get_events(url.to_string()).await.ok().unwrap();
            let text = create_text(events[0].clone());
            ReplyMessageRequest {
                reply_token: reply_token.clone(),
                messages: vec![Message::Text(TextMessage::new(text))],
                notification_disabled: Some(false),
            }
        },
    };

    let _ = line
        .messaging_api_client
        .reply_message(reply_message_request)
        .await;
}
