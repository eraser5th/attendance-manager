use std::sync::Arc;

use line_bot_sdk_rust::client::LINE;
use line_bot_sdk_rust::line_webhook::models::CallbackRequest;
use line_bot_sdk_rust::line_webhook::models::Event;

mod event_handler;

use crate::infra::DB;
use crate::linebot::ChatContext;
use event_handler::message_event_handler;

pub async fn request_handler(
    reqest: CallbackRequest,
    line: LINE,
    db: &mut Arc<DB>,
    chat_context: &mut Arc<ChatContext>,
) {
    for e in reqest.events {
        match e {
            Event::MessageEvent(message_event) => {
                message_event_handler(message_event, &line, &mut db.clone(), &mut chat_context.clone()).await
            }
            _ => (),
        };
    }
}
