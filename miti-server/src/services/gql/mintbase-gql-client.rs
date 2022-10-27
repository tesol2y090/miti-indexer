use graphql_client::{GraphQLQuery, Response};
use reqwest;
use std::error::Error;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/services/gql/get_sale_volume.graphql",
    response_derives = "Debug"
)]
pub struct GetSaleVolume;

pub async fn get_sale_volume(client: &Client, config: &Config, date: String, kind: String) -> Result<GetSaleVolume> {
	let variable = get_sale_volume_query::Variable {
		date,
		kind
	};
}