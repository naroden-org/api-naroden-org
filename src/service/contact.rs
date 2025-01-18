use crate::context::NarodenContext;
use crate::contract::contact::{Contact, ContactPhoneRequest, GetUserContactsResponse, PostContactsRequest};
use crate::data::model::contact::{get_user_contacts, store_user_contacts, DbContact, DbContactPhone};
use crate::data::model::user::{get_user_info, DbUser};
use crate::web::route::user::normalize_phone_number;

pub async fn get_contacts(context: NarodenContext) -> GetUserContactsResponse {
    let db_contacts: Vec<DbContact> = get_user_contacts(context.user_id()).await;

    GetUserContactsResponse {
        contacts: to_contacts(&db_contacts)
    }
}

fn to_contacts(db_contacts: &Vec<DbContact>) -> Vec<Contact> {
    let mut contacts:  Vec<Contact> = Vec::new();

    for db_contact in db_contacts {
        contacts.push(Contact {
            phone: db_contact.phone.clone(),
            section: db_contact.section.clone(),
            nickname: db_contact.nickname.clone(),
        });
    }

    contacts
}

pub async fn create_contacts(context: NarodenContext, request: PostContactsRequest) {
    let db_contact_phones: Vec<DbContactPhone> = to_db_contact_phone(context.user_id(), request.phones).await;
    store_user_contacts(context.user_id(), db_contact_phones).await;
}



async fn to_db_contact_phone(user_id: &str, phones: Vec<ContactPhoneRequest>) -> Vec<DbContactPhone> {
    let phone_code: String = get_phone_code(user_id).await;

    let mut data: Vec<DbContactPhone> = vec![];
    for contact in phones {
        data.push(DbContactPhone {
            user_id: user_id.to_string(),
            normalized_phone: normalize_phone_number(&contact.phone, &phone_code),
            phone: contact.phone.clone(),
            nickname: contact.nickname.clone(),
        });
    }

    data
}

async fn get_phone_code(user_id: &str) -> String {
    let user: Option<DbUser> = get_user_info(user_id).await;

    match user.unwrap().phone_code {
        None => {
            "".to_string()
        }
        Some(p) => {
            p.to_string()
        }
    }
}

