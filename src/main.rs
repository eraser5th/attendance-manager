use actix_web::web::ServiceConfig;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use std::sync::Arc;

use actix_web::{post, web, Error, HttpResponse};
use dotenv::dotenv;
use line_bot_sdk_rust::support::actix::Signature;
pub mod linebot;

use linebot::LineBotEnv;

#[derive(Clone)]
struct AppState {
    line_channel_access_token: Arc<String>,
    line_channel_secret: Arc<String>,
}

#[post("/callback")]
async fn callback(
    data: web::Data<AppState>,
    signature: Signature,
    bytes: web::Bytes,
) -> Result<HttpResponse, Error> {
    let access_token = data.line_channel_access_token.get(0..).unwrap();
    let channel_secret = data.line_channel_secret.get(0..).unwrap();

    let linebot_env = LineBotEnv {
        line_channel_access_token: access_token.to_string(),
        line_channel_secret: channel_secret.to_string(),
    };

    linebot::linebot(linebot_env, signature, bytes).await
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let channel_secret = secrets
        .get("LINE_CHANNEL_SECRET")
        .expect("Failed to get LINE_CHANNEL_SECRET");
    let access_token = secrets
        .get("LINE_CHANNEL_ACCESS_TOKEN")
        .expect("Failed to get LINE_CHANNEL_ACCESS_TOKEN");
    let app_state = AppState {
        line_channel_secret: Arc::new(channel_secret),
        line_channel_access_token: Arc::new(access_token),
    };

    dotenv().ok();
    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(web::Data::new(app_state.clone()))
            .service(callback);
    };

    Ok(config.into())
}
