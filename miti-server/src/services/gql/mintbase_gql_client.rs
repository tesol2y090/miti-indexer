use crate::utils::get_past_date;
use graphql_client::{GraphQLQuery, Response};
use reqwest;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

#[allow(non_camel_case_types)]
type numeric = f64;
#[allow(non_camel_case_types)]
type timestamp = String;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "src/services/gql/get_nft_activities.graphql",
    schema_path = "src/services/gql/schema.json",
    response_derives = "Debug"
)]
pub struct GetNFTActivities;

#[derive(Serialize)]
pub struct MarketActivities {
    pub start_timestamp: String,
    pub volume: f64,
    pub transaction_count: i64,
    pub activities_by_date: Vec<ActivitiesByDate>,
}

#[derive(Debug, Serialize)]
pub struct ActivitiesByDate {
    pub date: String,
    pub transaction: i64,
    pub price: f64,
}

pub async fn get_nft_activities(
    start_date: String,
    kind: String,
) -> Result<MarketActivities, Box<dyn Error>> {
    let start_timestamp = start_date.to_owned();
    let variables = get_nft_activities::Variables {
        date: start_date,
        kind: Some(kind),
    };

    let request_body = GetNFTActivities::build_query(variables);
    let mut activities_by_date_vec: Vec<ActivitiesByDate> = vec![];
    let mut transaction_count_map: HashMap<String, i64> = HashMap::new();
    let mut price_sum_map: HashMap<String, f64> = HashMap::new();
    let all_date = get_past_date(30);

    let client = reqwest::Client::new();
    let res = client
        .post("https://interop-mainnet.hasura.app/v1/graphql")
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<get_nft_activities::ResponseData> = res.json().await?;
    let response_activities = response_body.data.unwrap();
    let aggregate_data = response_activities
        .nft_activities_aggregate
        .aggregate
        .unwrap();

    let nodes_data = response_activities.nft_activities_aggregate.nodes;
    for node in nodes_data {
        let date = (&node.timestamp[..10]).to_owned();
        *transaction_count_map.entry(date.to_owned()).or_insert(0) += 1;
        *price_sum_map.entry(date.to_owned()).or_insert(0.0) += node.price.unwrap();
    }

    for date in all_date {
        transaction_count_map.entry(date.to_owned()).or_insert(0);
        price_sum_map.entry(date.to_owned()).or_insert(0.0);
    }

    for date in transaction_count_map.keys() {
        let transaction_count = *transaction_count_map.get(date).unwrap();
        let price_sum = *price_sum_map.get(date).unwrap();
        let activities_by_date = ActivitiesByDate {
            date: date.to_string(),
            transaction: transaction_count,
            price: price_sum,
        };
        activities_by_date_vec.push(activities_by_date);
    }

    // TODO: sort activities_by_date by date

    let aggregate_data_sum = aggregate_data.sum.unwrap();
    let aggregate_data_count = aggregate_data.count;
    let volume_price = aggregate_data_sum.price.unwrap();

    Ok(MarketActivities {
        volume: volume_price,
        start_timestamp,
        transaction_count: aggregate_data_count,
        activities_by_date: activities_by_date_vec,
    })
}
