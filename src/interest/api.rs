use std::collections::BTreeMap;
use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::OpenApi;
use poem_openapi::param::{Path};
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::{Thing};
use surrealdb::Surreal;
use crate::error::data::{create_error, ApiError};
use crate::interest::data::{DbInterest, DbInterestStatistics, DbHasInterest, GetInterest, GetInterestResponse, GetInterests, GetInterestsResponse, Interest, PatchInterestRequest, Statistics};
use crate::jwt::data::JwtClaims;

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("USER")]
    #[oai(path = "/private/v1/interests", method = "get")]
    async fn get_all(&self, db: Data<&Surreal<Client>>, raw_request: &Request) -> Result<GetInterestsResponse> {
        let interests: Option<Vec<DbInterest>> = db.select("interest").await.ok().take();

        let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
        let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();
        let db_has_interest: Vec<DbHasInterest> = db.query(GET_ALL_INTEREST_STATUSES)
            .bind(("user_id", user_id))
            .await.expect("error").take(0).expect("error");
        let sections = db_has_interest
                    .into_iter()
                    .map(|x| (x.out.clone(), self.status_to_section(x.status)))
                    .collect::<BTreeMap<Thing, String>>();



        match interests {
            Some(db_interests) => Ok(GetInterestsResponse::Ok(Json(self.create_interests_response(db_interests, sections)))),
            None => Ok(GetInterestsResponse::GeneralError(Json(create_error(ApiError::GeneralError)))),
        }
    }

    fn status_to_section(&self, status: i32) -> String {
        if status == 0 {
            "Забранени".to_string()
        } else if status == 1 {
            "Позволени".to_string()
        } else if status == 2 {
            "Любими".to_string()
        } else {
            "Други".to_string()
        }

    }

    fn create_interests_response(&self, interests: Vec<DbInterest>, sections: BTreeMap<Thing, String>) -> GetInterests {
        let mut response_interests = vec![];
        for interest in interests.iter() {
            let section: String = if sections.contains_key(&interest.id) {
                sections.get(&interest.id).unwrap().to_string()
            } else {
                self.status_to_section(interest.default_status)
            };

            response_interests.push(Interest {
                id: interest.id.id.to_string(),
                name: interest.name.to_owned(),
                section: section,
            });
        }

        GetInterests {
            interests: response_interests,
        }
    }

    #[protect("USER")]
    #[oai(path = "/private/v1/interests/:id", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, id: Path<String>, raw_request: &Request) -> Result<GetInterestResponse> {
        let interest: Option<DbInterest> = db.select(("interest", id.0)).await.expect("error");

        match interest {
            None => { Ok(GetInterestResponse::GeneralError(Json(create_error(ApiError::GeneralError)))) }
            Some(db_interest) => {
                let claims = raw_request.extensions().get::<JwtClaims>().unwrap();
                let user_id: Thing = Thing::from_str(format!("user:{}", claims.sub.to_owned()).as_str()).unwrap();

                let relation: Option<DbHasInterest> = db.query(GET_INTEREST_STATUS_QUERY)
                    .bind(("user", user_id))
                    .bind(("interest", db_interest.id.clone()))
                    .await.expect("error").take(0).expect("error");
                let interest_status: i32 = if relation.is_none() {
                    db_interest.default_status
                } else {
                    relation.unwrap().status
                };

                let interest_statistics: Vec<DbInterestStatistics> = db.query(GET_INTEREST_STATISTICS_QUERY)
                    .bind(("interest", db_interest.id))
                    .await.expect("error").take(0).expect("error");
                let mut all_users_total: Statistics = Statistics {
                    section: "Всички потребители".to_string(),
                    favourite: 0,
                    forbidden: 0,
                    allowed: 0,
                };
                for interest_statistic in interest_statistics {
                    if interest_statistic.status == 0 {
                        all_users_total.forbidden = interest_statistic.count;
                    } else if interest_statistic.status == 1 {
                        all_users_total.allowed = interest_statistic.count;
                    } else if interest_statistic.status == 2 {
                        all_users_total.favourite = interest_statistic.count;
                    }
                }

                let response: GetInterest = GetInterest {
                    stats: vec![all_users_total],
                    status: interest_status,
                    name: db_interest.name,
                    description: db_interest.description,
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
        LET $relation = SELECT * FROM has_interest WHERE in=$user and out=$interest LIMIT 1;
        IF $relation {
            UPDATE $relation SET status=$status;
        } ELSE {
            RELATE $user->has_interest->$interest SET status=$status;
        };
    ";

const GET_INTEREST_STATUS_QUERY: &str = "SELECT * FROM has_interest WHERE in=$user and out=$interest LIMIT 1;";

const GET_INTEREST_STATISTICS_QUERY: &str = "
        SELECT status, count()
        FROM has_interest
        WHERE out = $interest
        GROUP BY status
";

const GET_ALL_INTEREST_STATUSES: &str = "SELECT status, out FROM has_interest where in=$user_id;";