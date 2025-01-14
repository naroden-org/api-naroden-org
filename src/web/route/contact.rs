use axum::Json;
use crate::context::NarodenContext;
use crate::contract::contact::{GetUserContactsResponse, PostContactsRequest};
use crate::service::contact::{create_contacts, get_contacts};
use crate::web::server::NarodenResult;

pub async fn retrieve_contacts(context: NarodenContext) -> NarodenResult<Json<GetUserContactsResponse>> {
    let response : GetUserContactsResponse = get_contacts(context).await;

    Ok(Json(response))
}

pub async fn create_contact(context: NarodenContext, request: Json<PostContactsRequest>) {
    create_contacts(context, request.0).await;
}