use graphql_client::{GraphQLQuery, Response};
use reqwest;
use serde::Serialize;
use std::error::Error;

#[allow(non_camel_case_types)]
type numeric = f64;
#[allow(non_camel_case_types)]
type timestamp = String;

#[derive(GraphQLQuery)]
#[graphql(
    query_path = "src/services/gql/get_sale_volume.graphql",
    schema_path = "src/services/gql/schema.json",
    response_derives = "Debug"
)]
pub struct GetSaleVolume;

#[derive(Serialize)]
pub struct Volume {
    volume: f64,
    start_timestamp: String,
}

pub async fn get_sale_volume(date: String, kind: String) -> Result<Volume, Box<dyn Error>> {
    let start_timestamp = date.to_owned();
    let variables = get_sale_volume::Variables {
        date,
        kind: Some(kind),
    };

    let request_body = GetSaleVolume::build_query(variables);

    let client = reqwest::Client::new();
    let res = client
        .post("https://interop-mainnet.hasura.app/v1/graphql")
        .json(&request_body)
        .send()
        .await?;
    let response_body: Response<get_sale_volume::ResponseData> = res.json().await?;
    let response_volume = response_body.data.unwrap();
    let aggregate_data = response_volume.nft_activities_aggregate.aggregate.unwrap();
    let aggregate_data_sum = aggregate_data.sum.unwrap();
    let volume_price = aggregate_data_sum.price.unwrap();
    Ok(Volume {
        volume: volume_price,
        start_timestamp,
    })
}
