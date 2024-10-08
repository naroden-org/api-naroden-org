use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::{Path};
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;
use crate::error::data::{create_error, Error};
use crate::jwt::data::JwtClaims;
use crate::tag::data::{DbOwnsTag, DbTag, DbTagStatistics, GetTag, GetTagResponse, GetTags, GetTagsResponse, PatchTagRequest, Statistics, Tag};

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("USER")]
    #[oai(path = "/v1/tags", method = "get")]
    async fn get_all(&self, db: Data<&Surreal<Client>>) -> Result<GetTagsResponse> {
        let tags: Option<Vec<DbTag>> = db.select("tag").await.ok().take();

        match tags {
            Some(tags) => Ok(GetTagsResponse::Ok(Json(self.create_tags_response(tags)))),
            None => Ok(GetTagsResponse::GeneralError(Json(create_error(Error::GeneralError)))),
        }
    }


    fn create_tags_response(&self, tags: Vec<DbTag>) -> GetTags {
        let mut response_tags = vec![];
        for tag in tags.iter() {
            response_tags.push(self.create_tag_response(tag));
        }

        return GetTags {
            tags: response_tags,
        }
    }

    fn create_tag_response(&self, from: &DbTag) -> Tag {
        Tag {
            id: from.id.id.to_string(),
            name: from.name.to_owned(),
            section: from.section.to_owned(),
        }
    }

    #[protect("USER")]
    #[oai(path = "/v1/tags/:id", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, id: Path<String>, raw_request: &Request) -> Result<GetTagResponse> {
        let tag: Option<DbTag> = db.select(("tag", id.0)).await.expect("error");

        match tag {
            None => { Ok(GetTagResponse::GeneralError(Json(create_error(Error::GeneralError)))) }
            Some(tag) => {
                let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
                let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();

                let relation: Option<DbOwnsTag> = db.query(GET_TAG_STATUS_QUERY)
                    .bind(("user", user_id))
                    .bind(("tag", tag.id.clone()))
                    .await.expect("error").take(0).expect("error");
                let tag_status: i32 = if relation.is_none() {
                    tag.default_status
                } else {
                    relation.unwrap().status
                };

                let tag_statistics: Vec<DbTagStatistics> = db.query(GET_TAG_STATISTICS_QUERY)
                    .bind(("tag", tag.id))
                    .await.expect("error").take(0).expect("error");
                let mut all_users_total: Statistics = Statistics {
                    section: "Всички потребители".to_string(),
                    allowed: 0,
                    forbidden: 0,
                    neutral: 0,
                };
                for tag_statistic in tag_statistics {
                    if tag_statistic.status == 0 {
                        all_users_total.neutral = tag_statistic.count;
                    } else if tag_statistic.status == 1 {
                        all_users_total.allowed = tag_statistic.count;
                    } else if tag_statistic.status == 2 {
                        all_users_total.forbidden = tag_statistic.count;
                    }
                }

                let response: GetTag = GetTag {
                    stats: vec![all_users_total],
                    status: tag_status,
                    name: tag.name,
                    description: tag.text,
                };

                return Ok(GetTagResponse::Ok(Json(response)))
            }
        }
    }


    #[protect("USER")]
    #[oai(path = "/v1/tags/:id", method = "patch")]
    async fn patch(&self, db: Data<&Surreal<Client>>, id: Path<String>, raw_request: &Request, body: Json<PatchTagRequest>) {
        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();

        let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();
        let tag_id: Thing = Thing::from_str(format!("tag:{}", id.0).as_str()).unwrap();

        db.query(CREATE_OR_UPDATE_TAG_STATUS_QUERY)
            .bind(("user", user_id))
            .bind(("tag", tag_id))
            .bind(("status", body.status))
            .await.expect("error")
            .check().expect("TODO: panic message");


    }

}


const CREATE_OR_UPDATE_TAG_STATUS_QUERY: &str = "
        LET $relation = SELECT * FROM owns_tag WHERE in=$user and out=$tag LIMIT 1;
        IF $relation {
            UPDATE $relation SET status=$status;
        } ELSE {
            RELATE $user->owns_tag->$tag SET status=$status;
        };
    ";

const GET_TAG_STATUS_QUERY: &str = "SELECT * FROM owns_tag WHERE in=$user and out=$tag LIMIT 1;";

const GET_TAG_STATISTICS_QUERY: &str = "
        SELECT status, count()
        FROM owns_tag
        WHERE out = $tag
        GROUP BY status
";