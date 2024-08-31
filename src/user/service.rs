use crate::user::data::{GetUserRequest, PostUserRequest, User};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::user::query::CREATE_USER_QUERY;

pub async fn create(db: &Surreal<Client>, user: PostUserRequest) -> User {

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user.password.as_ref(), &salt)
        .ok()
        .expect("error")
        .to_string();

    let query = db
        .query(CREATE_USER_QUERY)
        .bind(("first_name", user.first_name))
        .bind(("last_name", user.last_name))
        .bind(("password", password_hash))
        .bind(("password_salt", salt.to_string()))
        .bind(("email", user.email.map_or(String::from(""), |n|  n)))
        .bind(("phone", user.phone.map_or(String::from(""), |n|  n)));

    let created: Option<User> = query.await.expect("error").take(0).expect("no users returned from db");

    created.expect("failed to create a user")
}

pub fn find_all(_criteria: GetUserRequest) -> Vec<User> {
    todo!()
}
