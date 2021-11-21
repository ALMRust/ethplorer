use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LastBlock {
    #[serde(rename(deserialize = "lastBlock"))]
    last_block: u64,
}
