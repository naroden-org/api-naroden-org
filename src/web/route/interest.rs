use std::collections::BTreeMap;
use std::str::FromStr;
use axum::extract::Path;
use axum::Json;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::context::NarodenContext;
use crate::data::database::NARODEN_DB;
use crate::data::model::interest::{DbHasInterest, DbInterest, DbInterestStatistics};
use crate::error::NarodenError;
use crate::web::server::NarodenResult;

pub async fn get_all_interests(context: NarodenContext) -> NarodenResult<Json<GetInterests>> {
    let interests: Option<Vec<DbInterest>> = NARODEN_DB.select("interest").await.ok().take();

    let user_id: Thing = Thing::from_str(format!("user:{}", context.user_id().to_owned()).as_str()).unwrap();
    let db_has_interest: Vec<DbHasInterest> = NARODEN_DB.query(GET_ALL_INTEREST_STATUSES)
        .bind(("user_id", user_id))
        .await.expect("error").take(0).expect("error");
    let sections = db_has_interest
        .into_iter()
        .map(|x| (x.out.clone(), status_to_section(x.status)))
        .collect::<BTreeMap<Thing, String>>();



    match interests {
        Some(db_interests) => Ok(Json(create_interests_response(db_interests, sections))),
        None => Err(NarodenError::GeneralError),
    }
}

fn status_to_section(status: i32) -> String {
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

fn create_interests_response(interests: Vec<DbInterest>, sections: BTreeMap<Thing, String>) -> GetInterests {
    let mut response_interests = vec![];
    for interest in interests.iter() {
        let section: String = if sections.contains_key(&interest.id) {
            sections.get(&interest.id).unwrap().to_string()
        } else {
            status_to_section(interest.default_status)
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

pub async fn retrieve_interest(context: NarodenContext, Path(id): Path<String>) -> NarodenResult<Json<GetInterest>> {
    let interest: Option<DbInterest> = NARODEN_DB.select(("interest", id)).await.expect("error");

    match interest {
        None => { Err(NarodenError::GeneralError) }
        Some(db_interest) => {
            let user_id: Thing = Thing::from_str(format!("user:{}", context.user_id()).as_str()).unwrap();

            let relation: Option<DbHasInterest> = NARODEN_DB.query(GET_INTEREST_STATUS_QUERY)
                .bind(("user", user_id))
                .bind(("interest", db_interest.id.clone()))
                .await.expect("error").take(0).expect("error");
            let interest_status: i32 = if relation.is_none() {
                db_interest.default_status
            } else {
                relation.unwrap().status
            };

            let interest_statistics: Vec<DbInterestStatistics> = NARODEN_DB.query(GET_INTEREST_STATISTICS_QUERY)
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

            Ok(Json(response))
        }
    }
}


pub async fn update_interest(context: NarodenContext, Path(id): Path<String>, request: Json<PatchInterestRequest>) {
    let user_id: Thing = Thing::from_str(format!("user:{}", context.user_id()).as_str()).unwrap();
    let interest_id: Thing = Thing::from_str(format!("interest:{}", id).as_str()).unwrap();

    NARODEN_DB.query(CREATE_OR_UPDATE_INTEREST_STATUS_QUERY)
        .bind(("user", user_id))
        .bind(("interest", interest_id))
        .bind(("status", request.status))
        .await.expect("error")
        .check().expect("TODO: panic message");


}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterests {
    pub interests: Vec<Interest>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Interest {
    pub id: String,
    pub name: String,
    pub section: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatchInterestRequest {
    pub status: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterest {
    pub stats: Vec<Statistics>,
    pub status: i32,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistics {
    pub section: String,
    pub favourite: i32,
    pub forbidden: i32,
    pub allowed: i32,
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