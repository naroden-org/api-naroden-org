use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::{Path};
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Id, Thing};
use surrealdb::Surreal;
use crate::error::data::{create_error, ApiError};
use crate::interest::data::{DbInterest, DbInterestStatistics, DbOwnsInterest, GetInterest, GetInterestResponse, GetInterests, GetInterestsResponse, Interest, PatchInterestRequest, Statistics};
use crate::jwt::data::JwtClaims;

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("USER")]
    #[oai(path = "/private/v1/interests", method = "get")]
    async fn get_all(&self, db: Data<&Surreal<Client>>) -> Result<GetInterestsResponse> {
        let interests: Option<Vec<DbInterest>> = db.select("interest").await.ok().take();

        match interests {
            Some(dbInterests) => Ok(GetInterestsResponse::Ok(Json(self.create_interests_response(dbInterests)))),
            None => Ok(GetInterestsResponse::GeneralError(Json(create_error(ApiError::GeneralError)))),
        }
    }

    fn create_interests_response(&self, interests: Vec<DbInterest>) -> GetInterests {
        let mut response_interests = vec![];
        for interest in interests.iter() {
            response_interests.push(self.create_interest_response(interest));
        }

        GetInterests {
            interests: response_interests,
        }
    }

    fn create_interest_response(&self, from: &DbInterest) -> Interest {
        Interest {
            id: from.id.id.to_string(),
            name: from.name.to_owned(),
            section: from.section.to_owned(),
        }
    }

    #[protect("USER")]
    #[oai(path = "/private/v1/interests/:id", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, id: Path<String>, raw_request: &Request) -> Result<GetInterestResponse> {
        let interest: Option<DbInterest> = db.select(("interest", id.0)).await.expect("error");

        match interest {
            None => { Ok(GetInterestResponse::GeneralError(Json(create_error(ApiError::GeneralError)))) }
            Some(dbInterest) => {
                let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
                let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();

                let relation: Option<DbOwnsInterest> = db.query(GET_INTEREST_STATUS_QUERY)
                    .bind(("user", user_id))
                    .bind(("interest", dbInterest.id.clone()))
                    .await.expect("error").take(0).expect("error");
                let interest_status: i32 = if relation.is_none() {
                    dbInterest.default_status
                } else {
                    relation.unwrap().status
                };

                let interest_statistics: Vec<DbInterestStatistics> = db.query(GET_INTEREST_STATISTICS_QUERY)
                    .bind(("interest", dbInterest.id))
                    .await.expect("error").take(0).expect("error");
                let mut all_users_total: Statistics = Statistics {
                    section: "Всички потребители".to_string(),
                    allowed: 0,
                    forbidden: 0,
                    neutral: 0,
                };
                for interest_statistic in interest_statistics {
                    if interest_statistic.status == 0 {
                        all_users_total.forbidden = interest_statistic.count;
                    } else if interest_statistic.status == 1 {
                        all_users_total.neutral = interest_statistic.count;
                    } else if interest_statistic.status == 2 {
                        all_users_total.allowed = interest_statistic.count;
                    }
                }

                let response: GetInterest = GetInterest {
                    stats: vec![all_users_total],
                    status: interest_status,
                    name: dbInterest.name,
                    description: dbInterest.text,
                };

                return Ok(GetInterestResponse::Ok(Json(response)))
            }
        }
    }


    #[protect("USER")]
    #[oai(path = "/private/v1/interests/:id", method = "patch")]
    async fn patch(&self, db: Data<&Surreal<Client>>, id: Path<String>, raw_request: &Request, body: Json<PatchInterestRequest>) {
        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();

        let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();
        let interest_id: Thing = Thing::from_str(format!("interest:{}", id.0).as_str()).unwrap();

        db.query(CREATE_OR_UPDATE_INTEREST_STATUS_QUERY)
            .bind(("user", user_id))
            .bind(("interest", interest_id))
            .bind(("status", body.status))
            .await.expect("error")
            .check().expect("TODO: panic message");


    }

}


const CREATE_OR_UPDATE_INTEREST_STATUS_QUERY: &str = "
        LET $relation = SELECT * FROM owns_interest WHERE in=$user and out=$interest LIMIT 1;
        IF $relation {
            UPDATE $relation SET status=$status;
        } ELSE {
            RELATE $user->owns_interest->$interest SET status=$status;
        };
    ";

const GET_INTEREST_STATUS_QUERY: &str = "SELECT * FROM owns_interest WHERE in=$user and out=$interest LIMIT 1;";

const GET_INTEREST_STATISTICS_QUERY: &str = "
        SELECT status, count()
        FROM owns_interest
        WHERE out = $interest
        GROUP BY status
";