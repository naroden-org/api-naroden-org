use crate::context::NarodenContext;
use crate::contract::notification::NotificationTokenRequest;
use crate::data::model::notification::{delete_notification_token, get_notification_token, store_notification_token, DbNotificationToken};

pub async fn refresh_notification_token(context: NarodenContext, request: NotificationTokenRequest) {
     let existing_token: Option<DbNotificationToken> = get_notification_token(&request.token).await;
     if existing_token.is_none() {
          let new_token = DbNotificationToken {
               id: None,
               device_type: context.device_type().to_string(),
               token: request.token.clone(),
               device_id: context.device_id().to_string().clone(),
               user_id: context.user_id().to_string()
          };

          store_notification_token(new_token).await;
          return;
     }

     let existing_token = existing_token.unwrap();

     if existing_token.device_id != context.device_id() {
          // TODO: make it transactional

          delete_notification_token(&existing_token.id.unwrap()).await;

          let new_token = DbNotificationToken {
               id: None,
               device_type: context.device_type().to_string(),
               token: request.token.clone(),
               device_id: context.device_id().to_string().clone(),
               user_id: context.user_id().to_string()
          };

          store_notification_token(new_token).await;
          return;
     }


}