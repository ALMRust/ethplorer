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

#[derive(Deserialize, Debug)]
pub struct TransactionDate {
    pub year: u64,
    pub month: u64,
    pub day: u64,
}

#[derive(Deserialize, Debug)]
pub struct CountTxs {
    #[serde(rename(deserialize = "_id"))]
    pub id: TransactionDate,
    pub ts: u64,
    pub cnt: u64,
}

#[derive(Deserialize, Debug)]
pub struct TokenDailyTransactionCounts {
    #[serde(rename(deserialize = "countTxs"))]
    pub count_txs: Vec<CountTxs>,
}

#[derive(Deserialize, Debug)]
pub struct Token {

}