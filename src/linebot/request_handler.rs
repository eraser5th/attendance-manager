use line_bot_sdk_rust::client::LINE;
use line_bot_sdk_rust::line_webhook::models::CallbackRequest;
use line_bot_sdk_rust::line_webhook::models::Event;

mod event_handler;

use event_handler::message_event_handler;

pub async fn request_handler(reqest: CallbackRequest, line: LINE) {
    println!("req: {reqest:#?}");
    for e in reqest.events {
        match e {
            Event::MessageEvent(message_event) => message_event_handler(message_event, &line).await,
            _ => (),
        };
    }
}
