use axum::extract::{Path, Query};
use axum::Json;
use serde::{Deserialize, Serialize};
use crate::data::database::NARODEN_DB;
use crate::data::news::{DbNews, DbNewsButton};
use crate::error::NarodenError;
use crate::web::server::NarodenResult;

pub async fn get_all_news() -> NarodenResult<Json<GetAllNews>> {
    // TODO: add Query(_id): Query<Option<String>>, Query(_count): Query<Option<i32>>
    // let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
    // TODO: implement filter by ID and filter by count from querry params!!!
    // TODO: set interests to news
    let news: Vec<DbNews> = NARODEN_DB.select("news").await?;

    let mut data: Vec<NewsItem> = vec![];
    for db_news in &news {
        data.push(NewsItem {
            id: db_news.id.id.to_string(),
            title: db_news.title.to_owned(),
            text: db_news.text.chars().take(20).collect(),
            image: db_news.image.to_owned(),
            buttons: to_news_buttons(&db_news.buttons),
        });
    }

    let response = GetAllNews {
        news: data,
    };

    Ok(Json(response))
}

fn to_news_buttons(buttons: &Vec<DbNewsButton>) -> Vec<NewsButton> {
    let mut data: Vec<NewsButton> = vec![];
    for db_button in buttons {
        data.push(NewsButton {
            r#type: db_button.r#type.clone(),
            url: db_button.url.clone(),
        });
    }

    data
}

pub async fn get_single_news(Path(id): Path<String>) -> NarodenResult<Json<NewsDetails>>
{
    let news: Option<DbNews> = NARODEN_DB.select(("news", id)).await.expect("error");

    match news {
        None => Err(NarodenError::GeneralError),
        Some(news) => {
            let response = NewsDetails {
                title: news.title.to_owned(),
                text: news.text.to_owned(),
                image: news.image.to_owned(),
                buttons: to_news_buttons(&news.buttons),
            };

            Ok(Json(response))
        }
    }
}

pub async fn update_news(Path(id): Path<String>) -> NarodenResult<Json<NewsDetails>>
{
    let news: Option<DbNews> = NARODEN_DB.select(("news", id)).await.expect("error");

    match news {
        None => Err(NarodenError::GeneralError),
        Some(news) => {
            let response = NewsDetails {
                title: news.title.to_owned(),
                text: news.text.to_owned(),
                image: news.image.to_owned(),
                buttons: to_news_buttons(&news.buttons),
            };

            Ok(Json(response))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllNews {
    pub news: Vec<NewsItem>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsItem {
    pub id: String,
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<NewsButton>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsDetails {
    pub title: String,
    pub text: String,
    pub image: String,
    pub buttons: Vec<NewsButton>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NewsButton {
    pub r#type: String,
    pub url: String,
}