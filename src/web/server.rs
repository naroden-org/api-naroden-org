use std::net::SocketAddr;
use axum::{middleware, Router};
use axum::routing::{get, patch, post, put};
use crate::error::NarodenError;
use crate::web::{mw};
use crate::web::route::contact::{create_contact, retrieve_contacts};
use crate::web::route::interest::{get_all_interests, retrieve_interest, update_interest};
use crate::web::route::jwt::issue_jwt;
use crate::web::route::news::{get_all_news, get_single_news, update_news};
use crate::web::route::statistic::generate_statistics;
use crate::web::route::survey::{create_survey_answer, retrieve_all_surveys, retrieve_survey};
use crate::web::route::user::{create_user, retrieve_user_profile};

pub type NarodenResult<T> = Result<T, NarodenError>;

pub async fn start() {
    let address = SocketAddr::from(([127, 0, 0, 1], 3001));
    axum::Server::bind(&address)
        .serve(create_routes().into_make_service())
        .await
        .unwrap();
}

fn create_routes() -> Router {
    let public_routes = Router::new()
        .route("/public/v1/jwt", post(issue_jwt))
        .route("/public/v1/users", post(create_user));

    let private_routes = Router::new()
        .route("/private/v1/profile", get(retrieve_user_profile))
        .route("/private/v1/contacts", get(retrieve_contacts))
        .route("/private/v1/contacts", post(create_contact))
        .route("/private/v1/statistics/coverage", get(generate_statistics))
        .route("/private/v1/interests", get(get_all_interests))
        .route("/private/v1/interests/:id", get(retrieve_interest))
        .route("/private/v1/interests/:id", patch(update_interest))
        .route("/private/v1/news", get(get_all_news))
        .route("/private/v1/news/:id", get(get_single_news))
        .route("/private/v1/surveys", get(retrieve_all_surveys))
        .route("/private/v1/surveys/:id", get(retrieve_survey))
        .route("/private/v1/questions/:id/answers", post(create_survey_answer))
        .layer(middleware::from_fn(mw::authorization::authorize));

    let admin_routes = Router::new()
        .route("/admin/v1/news/:id", put(update_news));

    Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .merge(admin_routes)
}

