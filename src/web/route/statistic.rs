use axum::Json;
use serde::{Deserialize, Serialize};
use crate::data::database::NARODEN_DB;
use crate::data::statistic::DbStatistic;
use crate::web::server::NarodenResult;

pub async fn generate_statistics() -> NarodenResult<Json<GetStatistics>> {
    let new_user_statistic: Vec<DbStatistic> = NARODEN_DB.query(NEW_USER_STATISTICS)
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

    Ok(Json(response))
}

pub const NEW_USER_STATISTICS: &str = "
    SELECT count() as value, time::format(created_on, '%d.%m.%Y') as description FROM
    (SELECT time::floor(created_on, 1d) as created_on FROM user WHERE time::floor(created_on, 1d) > time::now() - 7d)
    GROUP BY description
    ";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetStatistics {
    pub stats: Vec<Statistic>,
    pub new_users: Vec<Statistic>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Statistic {
    pub description: String,
    pub value: String,
}