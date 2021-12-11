use serde::{Deserialize, de, Deserializer};
use std::str::FromStr;
use serde::de::{Visitor, MapAccess};
use std::marker::PhantomData;
use std::fmt;
use void::Void;
use std::fmt::Display;

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

#[derive(Deserialize, Debug, Default)]
pub struct TokenPrice {
    pub rate: f64,
    pub currency: String,
    pub diff: f64,
    pub diff7d: f64,
    pub diff30d: f64,
    #[serde(rename(deserialize = "volume24h"))]
    pub volume_24h: f64,
    #[serde(rename(deserialize = "volDiff1"))]
    pub volume_diff1: f64,
    #[serde(rename(deserialize = "volDiff7"))]
    pub volume_diff7: f64,
    #[serde(rename(deserialize = "volDiff30"))]
    pub volume_diff30: f64,
    #[serde(rename(deserialize = "marketCapUsd"))]
    pub market_cap_usd: f64,
    #[serde(rename(deserialize = "availableSupply"))]
    pub available_supply: f64,
    pub ts: i64,
}

impl FromStr for TokenPrice {
    // This implementation of `from_str` can never fail, so use the impossible
    // `Void` type as the error type.
    type Err = Void;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        Ok(TokenPrice {
            ..Default::default()
        })
    }
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
        D: Deserializer<'de>,
{
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
        where
            T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
            fmtr.write_str("bool or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
            where
                E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
            where
                M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

fn num_from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where T: FromStr,
          T::Err: Display,
          D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

#[derive(Deserialize, Debug, Default)]
pub struct TokenInfo {
    pub address: String,
    pub name: String,
    #[serde(deserialize_with = "num_from_str")]
    pub decimals: u64,
    pub symbol: String,
    #[serde(rename(deserialize = "totalSupply"))]
    pub total_supply: String,
    pub owner: String,
    #[serde(default)]
    pub txs_count: u64,
    #[serde(rename(deserialize = "transfersCount"))]
    pub transfers_count: u64,
    #[serde(rename(deserialize = "lastUpdated"))]
    pub last_updated: i64,
    #[serde(default)]
    pub slot: u64,
    #[serde(rename(deserialize = "StorageTotalSupply"), default)]
    pub storage_total_supply: u64,
    #[serde(rename(deserialize = "issuancesCount"))]
    pub issuances_count: u64,
    #[serde(rename(deserialize = "holdersCount"))]
    pub holders_count: u64,
    pub image: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub website: String,
    #[serde(default)]
    pub telegram: String,
    #[serde(default)]
    pub twitter: String,
    #[serde(default)]
    pub reddit: String,
    #[serde(default)]
    pub facebook: String,
    #[serde(default)]
    pub coingecko: String,
    #[serde(rename(deserialize = "ethTransfersCount"))]
    pub eth_transfer_count: u64,
    #[serde(deserialize_with = "string_or_struct")]
    pub price: TokenPrice,
    #[serde(rename(deserialize = "countOps"))]
    pub count_ops: u64,
    #[serde(rename(deserialize = "publicTags"), default)]
    pub public_tags: Vec<String>,
    #[serde(rename(deserialize = "opCount"), default)]
    pub op_count: u64,
    #[serde(default)]
    pub added: u64,
}

#[derive(Deserialize, Debug)]
pub struct Token {
    #[serde(rename(deserialize = "tokenInfo"))]
    pub token_info: TokenInfo,
    // TokenFinancials
    pub balance: f64,
    #[serde(rename(deserialize = "rawBalance"))]
    pub raw_balance: String,
    #[serde(rename(deserialize = "totalIn"))]
    pub total_in: f64,
    #[serde(rename(deserialize = "totalOut"))]
    pub total_out: f64,
}
