use crate::user::data::{GetUserRequest, PostUserRequest, User};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn create(db: &Surreal<Client>, user: PostUserRequest) -> User {
    let create_user_query = "
        BEGIN TRANSACTION;

        LET $user = CREATE user CONTENT {
	        first_name: $first_name,
	        last_name: $last_name,
            password: $password,
            password_salt: $password_salt,
            created_on: time::now()
        };

        LET $email = CREATE contact CONTENT {
            type: 'EMAIL',
            value: $email
        };

        RELATE $user->owns->$email;

        RETURN $user;

        COMMIT TRANSACTION;
    ";

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(user.password.as_ref(), &salt)
        .ok()
        .expect("error")
        .to_string();

    let mut result = db
        .query(create_user_query)
        .bind(("first_name", user.first_name))
        .bind(("last_name", user.last_name))
        .bind(("password", password_hash))
        .bind(("password_salt", salt.to_string()))
        .bind(("email", user.email))
        .await
        .expect("failed to execute create user query");

    let created: Option<User> = result.take(0).expect("no users returned from db");

    created.expect("failed to create a user")
}

pub fn find_all(_criteria: GetUserRequest) -> Vec<User> {
    todo!()
}
