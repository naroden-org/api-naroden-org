use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::{OpenApi};
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::error::data::{create_error, ApiError};
use crate::jwt::data::JwtClaims;
use crate::news::data::{DbNews, GetAllNews, GetAllNewsResponse, GetNewsDetailsResponse, NewsDetails, NewsItem};

pub struct Api;

pub fn get_default_news_count() -> i32 { 20 }

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("USER")]
    #[oai(path = "/private/v1/news", method = "get")]
    async fn getAll(&self,
                    db: Data<&Surreal<Client>>,
                    raw_request: &Request,
                    id: poem_openapi::param::Query<Option<String>>,
                    #[oai(default = "get_default_news_count")]
                    count: poem_openapi::param::Query<i32>) -> Result<GetAllNewsResponse> {
        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
        // TODO: implement filter by ID and filter by count from querry params!!!
        // TODO: set interests to news
        let news: Vec<DbNews> = db.query("SELECT * FROM news")
            // .bind(("user_id", claims.sub.to_owned()))
            .await.expect("error").take(0).expect("error");

        let mut data: Vec<NewsItem> = vec![];
        for dbNews in &news {
            data.push(NewsItem {
                id: dbNews.id.id.to_string(),
                title: dbNews.title.to_owned(),
                text: dbNews.text.chars().take(20).collect(),
                image: dbNews.image.to_owned(),
                buttons: dbNews.buttons.to_owned(),
            });
        }

        let response = GetAllNews {
            news: data,
        };

        Ok(GetAllNewsResponse::Ok(Json(response)))
    }

    #[protect("USER")]
    #[oai(path = "/private/v1/news/:id", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, raw_request: &Request, id: Path<String>) -> Result<GetNewsDetailsResponse>
    {
        let news: Option<DbNews> = db.select(("news", id.0)).await.expect("error");

        match news {
            None => Ok(GetNewsDetailsResponse::GeneralError(Json(create_error(ApiError::GeneralError)))),
            Some(news) => {
                let response = NewsDetails {
                    title: news.title.to_owned(),
                    text: news.text.to_owned(),
                    image: news.image.to_owned(),
                    buttons: news.buttons.to_owned(),
                };

                Ok(GetNewsDetailsResponse::Ok(Json(response)))
            }
        }
    }

    #[protect("USER")]
    #[oai(path = "/admin/v1/news/:id", method = "put")]
    async fn update(&self, db: Data<&Surreal<Client>>, raw_request: &Request, id: Path<String>) -> Result<GetNewsDetailsResponse>
    {
        let news: Option<DbNews> = db.select(("news", id.0)).await.expect("error");

        match news {
            None => Ok(GetNewsDetailsResponse::GeneralError(Json(create_error(ApiError::GeneralError)))),
            Some(news) => {
                let response = NewsDetails {
                    title: news.title.to_owned(),
                    text: news.text.to_owned(),
                    image: news.image.to_owned(),
                    buttons: news.buttons.to_owned(),
                };

                Ok(GetNewsDetailsResponse::Ok(Json(response)))
            }
        }
    }
}
