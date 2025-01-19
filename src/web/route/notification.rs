use axum::Json;
use crate::context::NarodenContext;
use crate::contract::notification::NotificationTokenRequest;
use crate::service::notification::refresh_notification_token;

pub async fn create_notification_token(context: NarodenContext, request: Json<NotificationTokenRequest>){
    refresh_notification_token(context, request.0).await;
}