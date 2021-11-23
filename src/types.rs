use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LastBlock {
    #[serde(rename(deserialize = "lastBlock"))]
    pub last_block: u64,
}

#[derive(Deserialize, Debug)]
pub struct Holder {
    pub address: String,
    pub balance: f64,
    pub share: f64,
}

#[derive(Deserialize, Debug)]
pub struct TopTokenHolders {
    pub holders: Vec<Holder>,
}
