use std::time::Duration;
use axum::{Router};
use axum::body::{Body};
use axum::error_handling::HandleErrorLayer;
use axum::http::{HeaderName, Request, StatusCode};
use axum::routing::{get, patch, post, put};
use tokio::net::TcpListener;
use tower_http::BoxError;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::{DefaultOnFailure, DefaultOnResponse, TraceLayer};
use tracing::Level;
use crate::error::{handle_panic, NarodenError};
use crate::web::middleware::authorization;
use crate::web::route::contact::{create_contact, retrieve_contacts};
use crate::web::route::interest::{get_all_interests, retrieve_interest, update_interest};
use crate::web::route::jwt::issue_jwt;
use crate::web::route::news::{get_all_news, get_single_news, update_news};
use crate::web::route::notification::create_notification_token;
use crate::web::route::statistic::generate_statistics;
use crate::web::route::survey::{create_survey_answer, retrieve_all_surveys, retrieve_survey};
use crate::web::route::user::{create_user, retrieve_user_profile};
use tower::ServiceBuilder;
use tower::timeout::error::Elapsed;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::catch_panic::CatchPanicLayer;

pub type NarodenResult<T> = Result<T, NarodenError>;

pub async fn start() {
    let listener = TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, create_routes()).await.unwrap();
}

fn create_routes() -> Router {
    let x_request_id = HeaderName::from_static("x-request-id");

    let public_routes = Router::new()
        .route("/public/v1/jwt", post(issue_jwt))
        .route("/public/v1/users", post(create_user))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    let request_id = request.headers().get("x-request-id");
                    tracing::info_span!("request_id", "{}", request_id.unwrap().to_str().unwrap())
                })
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)))
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(x_request_id.clone(), MakeRequestUuid))
        .layer(CorsLayer::permissive());

    let private_routes = Router::new()
        .route("/private/v1/profile", get(retrieve_user_profile))
        .route("/private/v1/contacts", get(retrieve_contacts))
        .route("/private/v1/contacts", post(create_contact))
        .route("/private/v1/statistics/coverage", get(generate_statistics))
        .route("/private/v1/interests", get(get_all_interests))
        .route("/private/v1/interests/:id", get(retrieve_interest))
        .route("/private/v1/interests/:id", patch(update_interest))
        .route("/private/v1/news/:id", get(get_single_news))
        .route("/private/v1/surveys", get(retrieve_all_surveys))
        .route("/private/v1/surveys/:id", get(retrieve_survey))
        .route("/private/v1/questions/:id/answers", post(create_survey_answer))
        .route("/private/v1/news", get(get_all_news))
        .route("/private/v1/notification-tokens", post(create_notification_token))
        .layer(RequestBodyLimitLayer::new(1_048_576))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<Body>| {
                    let request_id = request.headers().get("x-request-id");
                    tracing::info_span!("request_id", "{}", request_id.unwrap().to_str().unwrap())
                })
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::ERROR)))
        .layer(PropagateRequestIdLayer::new(x_request_id.clone()))
        .layer(SetRequestIdLayer::new(x_request_id.clone(), MakeRequestUuid))
        .layer(axum::middleware::from_fn(authorization::authorize))
        .layer(CorsLayer::permissive())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(30))
        )
        .layer(ServiceBuilder::new()
            .layer(CatchPanicLayer::custom(handle_panic))
        );

    let admin_routes = Router::new()
        .route("/admin/v1/news/:id", put(update_news))
        .layer(CorsLayer::permissive());

    Router::new()
        .merge(public_routes)
        .merge(private_routes)
        .merge(admin_routes)
}

async fn handle_timeout_error(error: BoxError) -> (StatusCode, String) {
    if error.is::<Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {error}"),
        )
    }
}
