use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
pub struct AdModel {
    pub id: u64,
    pub title: String,
    pub creatives: Vec<u64>,
    pub bid: u64,
    pub cpa_bid: u64,
    pub landing_url: String,
    pub bid_type: BidType,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BidType {
    Unknown,
    CPM,
    CPC,
    GD,
    OCPC,
    OCPM,
    CPA,
    CPT,
}
