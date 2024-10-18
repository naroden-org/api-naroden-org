use std::str::FromStr;
use poem::{Request, Result};
use poem::web::Data;
use poem_openapi::{OpenApi};
use poem_openapi::payload::Json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;
use crate::jwt::data::JwtClaims;
use crate::statistic::data::{DbStatistic, GetStatistics, GetStatisticsResponse, Statistic};

pub struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    #[protect("USER")]
    #[oai(path = "/private/v1/statistics/coverage", method = "get")]
    async fn get(&self, db: Data<&Surreal<Client>>, raw_request: &Request) -> Result<GetStatisticsResponse> {
        let new_user_statistic: Vec<DbStatistic> = db.query(NEW_USER_STATISTICS)
            .await.expect("error").take(0).expect("error");

        let mut new_user_response: Vec<Statistic> = vec![];
        let mut total_users: i32 = 0;
        for statistic in new_user_statistic.iter() {
            total_users += statistic.value;
            new_user_response.push(Statistic {
                description: statistic.description.to_string(),
                value: total_users.to_string(),
            });
        }

        let mut total_stats: Vec<Statistic> = vec![];
        total_stats.push(Statistic {
            description: "Нови за днес".to_string(),
            value: new_user_statistic.last().unwrap().value.to_string(),
        });
        // TODO: novi za grada
        // TODO: novi za balgaria
        // TODO: novi za chuzhbina

        let response = GetStatistics {
            stats: total_stats,
            new_users: new_user_response,
        };

        Ok(GetStatisticsResponse::Ok(Json(response)))
    }
}

pub const NEW_USER_STATISTICS: &str = "
    SELECT count() as value, time::format(created_on, '%d.%m.%Y') as description FROM
    (SELECT time::floor(created_on, 1d) as created_on FROM user WHERE time::floor(created_on, 1d) > time::now() - 7d)
    GROUP BY description
    ";