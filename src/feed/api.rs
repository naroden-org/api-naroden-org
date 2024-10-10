use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::{OpenApi};
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Query, Thing};
use surrealdb::Surreal;
use crate::error::data::{create_error, Error};
use crate::feed::data::{DbFeed, FeedDetails, FeedItem, GetAllFeed, GetAllFeedResponse, GetFeedDetailsResponse};
use crate::jwt::data::JwtClaims;
use crate::user::data::{GetUserResponse, UserResponse};

pub struct Api;

pub fn get_default_feed_count() -> i32 { 20 }

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("USER")]
    #[oai(path = "/v1/feed", method = "get")]
    async fn getAll(&self,
                    db: Data<&Surreal<Client>>,
                    raw_request: &Request,
                    id: poem_openapi::param::Query<Option<String>>,
                    #[oai(default = "get_default_feed_count")]
                    count: poem_openapi::param::Query<i32>) -> Result<GetAllFeedResponse> {
        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
        // TODO: implement filter by ID and filter by count!!!
        // TODO: set tags to feed
        let feeds: Vec<DbFeed> = db.query("SELECT * FROM feed")
            // .bind(("user_id", claims.sub.to_owned()))
            .await.expect("error").take(0).expect("error");

        let mut data: Vec<FeedItem> = vec![];
        for dbFeed in &feeds {
            data.push(FeedItem {
                id: dbFeed.id.id.to_string(),
                title: dbFeed.title.to_owned(),
                text: dbFeed.text.to_owned(),
                image: dbFeed.image.to_owned(),
                buttons: dbFeed.buttons.to_owned(),
            });
        }

        let response = GetAllFeed {
            feed: data,
        };

        Ok(GetAllFeedResponse::Ok(Json(response)))
    }

    #[protect("USER")]
    #[oai(path = "/v1/feed/:id", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, raw_request: &Request, id: Path<String>) -> Result<GetFeedDetailsResponse>
    {
        let feed: Option<DbFeed> = db.select(("feed", id.0)).await.expect("error");

        match feed {
            None => Ok(GetFeedDetailsResponse::GeneralError(Json(create_error(Error::GeneralError)))),
            Some(feed) => {
                let response = FeedDetails {
                    title: feed.title.to_owned(),
                    text: feed.text.to_owned(),
                    image: feed.image.to_owned(),
                    buttons: feed.buttons.to_owned(),
                };

                Ok(GetFeedDetailsResponse::Ok(Json(response)))
            }
        }
    }
}