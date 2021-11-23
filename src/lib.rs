use crate::types::{LastBlock, TopTokenHolders, TokenDailyTransactionCounts};
use reqwest::Error;

mod types;

// pub fn get_token_info(address: string, api_key: String) -> Result<LastBlock, Error> {
//     let client = reqwest::blocking::Client::new();
//     let url = "https://api.ethplorer.io/getTokenInfo/" + address;
//     let mut final_api_key = api_key;
//
//     if final_api_key == "" {
//         final_api_key = String::from("freekey");
//     }
//
//     let query = client.get(url)
//         .query(&[("apiKey", final_api_key)])
//         .send();
//
//     let res = match query {
//         Ok(res)  => res,
//         Err(e) => return Err(e),
//     };
//
//     let block = match res.json::<types::TokenInfo>() {
//         Ok(last_block)  => last_block,
//         Err(e) => return Err(e),
//     };
//
//     Ok(block)
// }

pub fn get_top_token_holders(
    address: &str,
    mut limit: u64,
    api_key: &str,
) -> Result<TopTokenHolders, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getTopTokenHolders/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 2] = [
        ("apiKey", final_api_key),
        ("", ""),
    ];

    let limit_string;
    if limit != 0 {
        if limit > 1000 {
            limit = 1000;
        }
        limit_string = limit.to_string();
        query_params[1] = ("limit", limit_string.as_str())
    }

    let query = client
        .get(url)
        .query(&query_params)
        .send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let holders = match res.json::<TopTokenHolders>() {
        Ok(top_holders) => top_holders,
        Err(e) => return Err(e),
    };

    Ok(holders)
}

pub fn get_last_block(api_key: &str) -> Result<LastBlock, Error> {
    let client = reqwest::blocking::Client::new();
    let url = "https://api.ethplorer.io/getLastBlock";

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let query = client.get(url).query(&[("apiKey", final_api_key)]).send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let block = match res.json::<LastBlock>() {
        Ok(last_block) => last_block,
        Err(e) => return Err(e),
    };

    Ok(block)
}

pub fn get_token_daily_transaction_count(
    address: &str,
    mut period: u64,
    api_key: &str,
) -> Result<TokenDailyTransactionCounts, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getTokenHistoryGrouped/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 2] = [
        ("apiKey", final_api_key),
        ("", ""),
    ];

    let period_string;
    if period != 0 {
        if period > 90 {
            period = 90;
        }
        period_string = period.to_string();
        query_params[1] = ("period", period_string.as_str())
    }

    let query = client
        .get(url)
        .query(&query_params)
        .send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let counts = match res.json::<TokenDailyTransactionCounts>() {
        Ok(txs_counts) => txs_counts,
        Err(e) => return Err(e),
    };

    Ok(counts)
}
