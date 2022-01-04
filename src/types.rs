use chrono::serde::ts_seconds;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::{value, MapAccess, SeqAccess, Visitor};
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;
use void::Void;

pub struct RequestConfig {
    pub network: String,
    pub routes: Vec<String>,
    pub params: Vec<(String, String)>,
}

impl Display for RequestConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.network)?;
        for route in &self.routes {
            write!(f, "/{}", route)?;
        }
        write!(f, "")
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct LastBlock {
    #[serde(rename(deserialize = "lastBlock"))]
    pub last_block: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct Holder {
    pub address: String,
    pub balance: f64,
    pub share: f64,
}

#[derive(Deserialize, Debug, Default)]
pub struct TopTokenHolders {
    pub holders: Vec<Holder>,
}

#[derive(Deserialize, Debug, Default)]
pub struct TransactionDate {
    pub year: u64,
    pub month: u64,
    pub day: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct CountTxs {
    #[serde(rename(deserialize = "_id"))]
    pub id: TransactionDate,
    pub ts: u64,
    pub cnt: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct TokenDailyTransactionCounts {
    #[serde(rename(deserialize = "countTxs"))]
    pub count_txs: Vec<CountTxs>,
}

#[derive(Deserialize, Debug, Default)]
pub struct TokenPrice {
    #[serde(default)]
    pub rate: f64,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub diff: f64,
    #[serde(default)]
    pub diff7d: f64,
    #[serde(default)]
    pub diff30d: f64,
    #[serde(rename(deserialize = "volume24h"), default)]
    pub volume_24h: f64,
    #[serde(rename(deserialize = "volDiff1"), default)]
    pub volume_diff1: f64,
    #[serde(rename(deserialize = "volDiff7"), default)]
    pub volume_diff7: f64,
    #[serde(rename(deserialize = "volDiff30"), default)]
    pub volume_diff30: f64,
    #[serde(rename(deserialize = "marketCapUsd"), default)]
    pub market_cap_usd: f64,
    #[serde(rename(deserialize = "availableSupply"), default)]
    pub available_supply: f64,
    #[serde(default)]
    pub ts: u64,
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

        fn visit_bool<E>(self, _: bool) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str("").unwrap())
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

#[derive(Deserialize, Debug, Default)]
#[serde(transparent)]
pub struct Prices(Vec<Price>);

impl Deref for Prices {
    type Target = Vec<Price>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn num_from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

fn str_or_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrU64<'a> {
        Str(&'a str),
        U64(u64),
    }

    Ok(match StrOrU64::deserialize(deserializer)? {
        StrOrU64::Str(v) => v.parse().unwrap_or(0), // Ignoring parsing errors
        StrOrU64::U64(v) => v,
    })
}

fn str_or_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StrOrU128<'a> {
        Str(&'a str),
        U128(u128),
    }

    Ok(match StrOrU128::deserialize(deserializer)? {
        StrOrU128::Str(v) => v.parse().unwrap_or(0), // Ignoring parsing errors
        StrOrU128::U128(v) => v,
    })
}

#[derive(Deserialize, Debug, Default)]
pub struct TokenInfo {
    pub address: String,
    pub name: String,
    #[serde(deserialize_with = "str_or_u64", default)]
    pub decimals: u64,
    pub symbol: String,
    #[serde(rename(deserialize = "totalSupply"), default)]
    pub total_supply: String,
    #[serde(default)]
    pub owner: String,
    #[serde(default)]
    pub txs_count: u64,
    #[serde(rename(deserialize = "transfersCount"), default)]
    pub transfers_count: u64,
    #[serde(rename(deserialize = "lastUpdated"), default)]
    pub last_updated: u64,
    #[serde(default)]
    pub slot: u64,
    #[serde(rename(deserialize = "StorageTotalSupply"), default)]
    pub storage_total_supply: u64,
    #[serde(rename(deserialize = "issuancesCount"), default)]
    pub issuances_count: u64,
    #[serde(rename(deserialize = "holdersCount"), default)]
    pub holders_count: u64,
    #[serde(default)]
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
    #[serde(rename(deserialize = "ethTransfersCount"), default)]
    pub eth_transfer_count: u64,
    #[serde(deserialize_with = "string_or_struct", default)]
    pub price: TokenPrice,
    #[serde(rename(deserialize = "countOps"), default)]
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
    #[serde(rename(deserialize = "rawBalance"), default)]
    pub raw_balance: String,
    #[serde(rename(deserialize = "totalIn"))]
    pub total_in: f64,
    #[serde(rename(deserialize = "totalOut"))]
    pub total_out: f64,
}

#[derive(Deserialize, Debug, Default)]
pub struct ETH {
    #[serde(deserialize_with = "string_or_struct")]
    pub price: TokenPrice,
    // TokenFinancials
    pub balance: f64,
    #[serde(rename(deserialize = "rawBalance"))]
    pub raw_balance: String,
    #[serde(rename(deserialize = "totalIn"))]
    pub total_in: f64,
    #[serde(rename(deserialize = "totalOut"))]
    pub total_out: f64,
}

#[derive(Deserialize, Debug, Default)]
pub struct ContractInfo {
    pub creator_hash: String,
    pub transaction_hash: String,
    #[serde(deserialize_with = "date_or_timestamp", default)]
    pub timestamp: Timestamp,
}
#[derive(Deserialize, Debug)]
pub struct Timestamp(#[serde(with = "ts_seconds")] DateTime<Utc>);

impl Deref for Timestamp {
    type Target = DateTime<Utc>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        let naive = NaiveDateTime::from_timestamp(0, 0);
        Timestamp(DateTime::from_utc(naive, Utc))
    }
}

fn date_or_timestamp<'de, D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: Deserializer<'de>,
{
    #[derive(Debug)]
    struct DateOrTimestamp(PhantomData<fn() -> Timestamp>);

    impl<'de> Visitor<'de> for DateOrTimestamp {
        type Value = Timestamp;

        fn expecting(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
           println!("{:?}", self);
            fmtr.write_str("i64, i32 or str")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Timestamp, E>
            where
                E: de::Error,
        {
            let naive = NaiveDateTime::from_timestamp(v, 0);
            Ok(Timestamp(DateTime::from_utc(naive, Utc)))
        }

        fn visit_i32<E>(self, v: i32) -> Result<Timestamp, E>
            where
                E: de::Error,
        {
            let naive = NaiveDateTime::from_timestamp(v as i64, 0);
            Ok(Timestamp(DateTime::from_utc(naive, Utc)))
        }

        fn visit_i16<E>(self, v: i16) -> Result<Timestamp, E>
            where
                E: de::Error,
        {
            let naive = NaiveDateTime::from_timestamp(v as i64, 0);
            Ok(Timestamp(DateTime::from_utc(naive, Utc)))
        }

        fn visit_i8<E>(self, v: i8) -> Result<Timestamp, E>
            where
                E: de::Error,
        {
            let naive = NaiveDateTime::from_timestamp(v as i64, 0);
            Ok(Timestamp(DateTime::from_utc(naive, Utc)))
        }

        fn visit_str<E>(self, s: &str) -> Result<Timestamp, E>
            where
                E: de::Error,
        {
            let date = DateTime::<Utc>::from_str(s);
            match date {
                Ok(v) => Ok(Timestamp(v)),
                Err(_) => Ok(Timestamp::default()),
            }
        }
    }
    deserializer.deserialize_any(DateOrTimestamp(PhantomData))
}

#[derive(Deserialize, Debug, Default)]
pub struct AddressInfo {
    pub address: String,
    #[serde(default)]
    pub eth: ETH,
    #[serde(default)]
    pub contract_info: ContractInfo,
    #[serde(default)]
    pub token_info: TokenInfo,
    pub tokens: Vec<Token>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Operations {
    #[serde(deserialize_with = "date_or_timestamp")]
    pub timestamp: Timestamp,
    #[serde(rename(deserialize = "transaction_hash"), default)]
    pub transaction_hash: String,
    #[serde(rename(deserialize = "tokenInfo"))]
    pub token_info: TokenInfo,
    #[serde(rename(deserialize = "type"))]
    pub op_type: String,
    #[serde(default)]
    pub address: String,
    pub from: String,
    pub to: String,
    #[serde(deserialize_with = "str_or_u128", default)]
    pub value: u128,
}

#[derive(Deserialize, Debug, Default)]
pub struct TokenHistory {
    pub operations: Vec<Operations>,
}

#[derive(Deserialize, Debug, Default)]
pub struct AddressTransaction {
    #[serde(deserialize_with = "date_or_timestamp")]
    pub timestamp: Timestamp,
    pub from: String,
    pub to: String,
    pub hash: String,
    pub value: f64,
    pub input: String,
    pub success: bool,
}

#[derive(Deserialize, Debug, Default)]
pub struct TopTokens {
    pub tokens: Vec<TokenInfo>,
    #[serde(rename(deserialize = "opCount"), default)]
    pub op_count: u64,
}

#[derive(Deserialize, Debug, Default)]
pub struct Price {
    pub ts: u64,
    #[serde(deserialize_with = "date_or_timestamp")]
    pub date: Timestamp,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    #[serde(rename(deserialize = "volumeConverted"))]
    pub volume_usd: f64,
    pub average: f64,
}

#[derive(Deserialize, Debug, Default)]
pub struct History {
    #[serde(rename(deserialize = "countTxs"))]
    pub count_txs: Vec<CountTxs>,
    pub prices: Prices,
}

#[derive(Deserialize, Debug, Default)]
pub struct TokenDailyPriceHistory {
    pub history: History,
}

// Struct Params
#[derive(Debug, Default)]
pub struct GetAddressInfoParams {
    pub token: String,
    pub show_eth_totals: bool,
}

#[derive(Debug, Default)]
pub struct GetTokenHistoryParams {
    pub history_type: String,
    pub limit: u64,
    pub timestamp: Timestamp,
}

#[derive(Debug, Default)]
pub struct GetAddressHistoryParams {
    pub history_type: String,
    pub limit: u64,
    pub timestamp: Timestamp,
    pub token: String,
}

#[derive(Debug, Default)]
pub struct GetAddressTransactionsParams {
    pub limit: u64,
    pub timestamp: Timestamp,
    pub show_zero_values: bool,
}

#[derive(Debug, Default)]
pub struct GetTopParams {
    pub limit: u64,
    pub criteria: String,
}
