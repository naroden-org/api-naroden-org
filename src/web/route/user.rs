use std::collections::HashMap;
use std::str::FromStr;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use axum::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::{Error};
use crate::context::NarodenContext;
use crate::data::database::NARODEN_DB;
use crate::data::model::user::{DbUser, User};
use crate::error::NarodenError;
use crate::web::route::jwt::{create_jwt, Jwt};
use crate::web::server::NarodenResult;

pub async fn create_user(request: Json<PostUserRequest>) -> NarodenResult<Json<Jwt>> {
        let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(request.password.as_ref(), &salt)
        .ok()
        .expect("error")
        .to_string();

    let email: String = match &request.email {
        None => { "".to_string() }
        Some(email) => { email.to_string() }
    };

    let phone_code: String = match &request.phone_code {
        None => { "".to_string() }
        Some(phone_code) => { phone_code.to_string() }
    };

    let phone: String = match &request.phone {
        None => { "".to_string() }
        Some(phone) => {
            normalize_phone_number(phone, &phone_code)
        }
    };

    let create_user_response = NARODEN_DB
        .query(CREATE_USER_QUERY)
        .bind(("first_name", request.first_name.to_owned()))
        .bind(("last_name", request.last_name.to_owned()))
        .bind(("password", password_hash))
        .bind(("password_salt", salt.to_string()))
        .bind(("email", email.to_string()))
        .bind(("phone", phone.to_string()))
        .bind(("phone_code", phone_code))
        .await;

    let mut create_user_response = match create_user_response {
        Ok(response) => {
            response
        }
        Err(error) => {
            dbg!(&error);
            return Err(NarodenError::GeneralError);

        }
    };

    let data:HashMap<usize, Error> = create_user_response.take_errors();
    if !data.is_empty() {
        match serde_json::to_string(&data) {
            Ok(error) => {
                if error.contains("Database index `unique_contact_value` already contains")
                {
                    if email.len() > 0  && error.contains(&email) {
                        return Err(NarodenError::AlreadyUsedEmail)
                    }
                    if phone.len() > 0 && error.contains(&phone) {
                        return Err(NarodenError::AlreadyUsedPhone)
                    }
                }
            }
            Err(e) => {
                dbg!(e);
                return Err(NarodenError::GeneralError)
            }
        }
    }

    let user: Option<User> = create_user_response.take(0).expect("reason");

    create_jwt(user.unwrap().id.unwrap().id.to_string())

    // TODO: send email on registration
    // TODO: accept terms and conditions
    // TODO: validate phone code
}

pub fn normalize_phone_number(phone: &String, phone_code: &String) -> String {
    let mut normalized_phone = phone.replace("+", "")
        .replace(" ", "")
        .replace("-", "")
        .replace("(", "")
        .replace(")", "");

    if normalized_phone.starts_with("00") {
        normalized_phone = normalized_phone.strip_prefix("00").unwrap().to_string()
    } else if normalized_phone.starts_with("0") {
        normalized_phone = normalized_phone.replacen("0", phone_code, 1);
    }

    normalized_phone
}

pub async fn retrieve_user_profile(context: NarodenContext) -> NarodenResult<Json<UserResponse>> {
    let user_id: Thing = Thing::from_str(format!("user:{}", context.user_id()).as_str()).unwrap();

    let user: Option<DbUser> = NARODEN_DB.query(GET_USER_INFO)
        .bind(("user", user_id))
        .await.expect("error").take(0).expect("error");

    match user {
        None => Err(NarodenError::GeneralError),
        Some(user) => {
            let response = UserResponse {
                first_name: user.first_name,
                last_name: user.last_name,
                email: user.email,
                phone: user.phone,
                phone_code: user.phone_code,
            };

            Ok(Json(response))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_code: Option<String>,
    pub password: String,
    pub referral: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub referral: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub first_name: String,
    pub last_name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub phone_code: Option<String>
}

pub const CREATE_USER_QUERY: &str = "
        BEGIN TRANSACTION;

        IF string::len($email) = 0 && string::len($phone) = 0 {
            RETURN 'email or phone required';
        } ELSE {
            LET $user = CREATE user CONTENT {
            	first_name: $first_name,
        	    last_name: $last_name,
                password: $password,
                password_salt: $password_salt,
                phone_code: $phone_code,
                created_on: time::now()
            };

            IF string::len($email) > 0 {
                LET $email_contact = CREATE contact CONTENT {
                    type: 'EMAIL',
                    value: $email
                };
                RELATE $user->owns_contact->$email_contact
                    SET is_email = true;
            };

            IF string::len($phone) > 0 {
                LET $phone_contact = CREATE contact CONTENT {
                    type: 'PHONE',
                    value: $phone
                };
                RELATE $user->owns_contact->$phone_contact
                    SET is_phone = true;
            };

            RETURN $user;
        };

        COMMIT TRANSACTION;
    ";

const GET_USER_INFO: &str = "
    SELECT
        ->owns_contact[WHERE is_phone]->contact.value[0][0] as phone,
        ->owns_contact[WHERE is_email]->contact.value[0][0] as email,
        first_name,
        last_name,
        phone_code
    FROM ONLY $user;
";