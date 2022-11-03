use actix_web::{
    get,
    web::{self},
    Responder, Result,
};
use chrono::Duration;
use serde::Deserialize;

use crate::services::gql::mintbase_gql_client::get_nft_activities;

#[derive(Deserialize)]
pub enum Marketplace {
    Paras,
    MintBase,
}

#[derive(Deserialize)]
pub struct GetVolumeRequest {
    market: Marketplace,
    kind: String,
}

#[get("/get_market_data")]
async fn get_market_data(
    get_volume_request: web::Json<GetVolumeRequest>,
) -> Result<impl Responder> {
    let market_place = &get_volume_request.market;
    let start_date = (chrono::offset::Local::now() - Duration::days(30)).to_string();
    let kind = get_volume_request.kind.to_owned();

    let market_activities = match market_place {
        Marketplace::MintBase => get_nft_activities(start_date, kind).await.unwrap(),
        Marketplace::Paras => get_nft_activities(start_date, kind).await.unwrap(),
    };

    Ok(web::Json(market_activities))
}
