#![feature(in_band_lifetimes)]

use crate::types::*;
use reqwest::Error;
use serde::de::DeserializeOwned;
use crate::consts::{FREE_KEY, GET_ADDRESS_INFO_ROUTE, GET_LAST_BLOCK_ROUTE, NETWORK};

pub mod types;
mod consts;

// TODO: Add status code error handling
// TODO: use macro for repeat values

pub fn api_key_param(api_key: &str) -> (&str, String) {
    let out;
    if api_key == "" {
        out = String::from(FREE_KEY);
    } else {
        out = String::from(api_key);
    }
    ("apiKey", out)
}

pub fn handle_request<T: DeserializeOwned>(config: RequestConfig) -> Result<T, Error> {
    let url = config.to_string();
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).query(&config.params).send()?;
    res.json::<T>()
}

pub fn get_address_info_config<'a>(
    api_key: &'a str,
    address: &'a str,
    in_params: &'a GetAddressInfoParams,
) -> RequestConfig<'a> {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    if in_params.token != "" {
        params.push(("token", in_params.token.clone()));
    }

    let eth_totals = in_params.show_eth_totals.to_string();
    params.push(("showETHTotals", eth_totals));

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_ADDRESS_INFO_ROUTE.to_string(), address.to_string()],
        params,
    }
}

pub fn get_address_info(
    api_key: &str,
    address: &str,
    params: &GetAddressInfoParams,
) -> Result<AddressInfo, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getAddressInfo/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 3] = [("apiKey", final_api_key), ("", ""), ("", "")];

    if params.token != "" {
        query_params[1] = ("token", params.token.as_str())
    }

    let eth_totals = params.show_eth_totals.to_string();
    query_params[2] = ("showETHTotals", eth_totals.as_str());

    let res = client.get(url).query(&query_params).send()?;
    res.json::<AddressInfo>()
}

pub fn get_token_info(api_key: &str, address: &str) -> Result<TokenInfo, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getTokenInfo/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let res = client.get(url).query(&[("apiKey", final_api_key)]).send()?;
    res.json::<types::TokenInfo>()
}

pub fn get_top_token_holders(
    api_key: &str,
    address: &str,
    mut limit: u64,
) -> Result<TopTokenHolders, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getTopTokenHolders/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 2] = [("apiKey", final_api_key), ("", "")];

    let limit_string;
    if limit != 0 {
        if limit > 1000 {
            limit = 1000;
        }
        limit_string = limit.to_string();
        query_params[1] = ("limit", limit_string.as_str())
    }

    let res = client.get(url).query(&query_params).send()?;
    res.json::<TopTokenHolders>()
}

// Get Last Block

pub fn get_last_block_config(
    api_key: &str,
) -> RequestConfig {
    let key = api_key_param(api_key);
    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_LAST_BLOCK_ROUTE.to_string()],
        params: vec![key],
    }
}

pub fn get_last_block(api_key: &str) -> Result<LastBlock, Error> {
    let config = get_last_block_config(api_key);
    handle_request(config)
}

pub fn get_token_new(api_key: &str) -> Result<Vec<TokenInfo>, Error> {
    let client = reqwest::blocking::Client::new();
    let url = "https://api.ethplorer.io/getTokensNew";

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let res = client.get(url).query(&[("apiKey", final_api_key)]).send()?;
    res.json::<Vec<TokenInfo>>()
}

pub fn get_token_daily_transaction_count(
    api_key: &str,
    address: &str,
    mut period: u64,
) -> Result<TokenDailyTransactionCounts, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getTokenHistoryGrouped/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 2] = [("apiKey", final_api_key), ("", "")];

    let period_string;
    if period != 0 {
        if period > 90 {
            period = 90;
        }
        period_string = period.to_string();
        query_params[1] = ("period", period_string.as_str())
    }

    let res = client.get(url).query(&query_params).send()?;
    res.json::<TokenDailyTransactionCounts>()
}

pub fn get_token_history(
    api_key: &str,
    address: &str,
    params: &GetTokenHistoryParams,
) -> Result<TokenHistory, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getTokenHistory/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 4] =
        [("apiKey", final_api_key), ("", ""), ("", ""), ("", "")];

    let limit_string;
    let mut limit = params.limit;
    if limit != 0 {
        if limit > 1000 {
            limit = 1000;
        }
        limit_string = limit.to_string();
        query_params[1] = ("limit", limit_string.as_str())
    }

    if params.history_type != "" {
        query_params[2] = ("type", params.history_type.as_str());
    }

    let timestamp_string;
    if params.timestamp.timestamp() != 0 {
        timestamp_string = params.timestamp.timestamp().to_string();
        query_params[3] = ("timestamp", timestamp_string.as_str());
    }

    let res = client.get(url).query(&query_params).send()?;
    res.json::<TokenHistory>()
}

pub fn get_address_history(
    api_key: &str,
    address: &str,
    params: &GetAddressHistoryParams,
) -> Result<TokenHistory, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getAddressHistory/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 5] = [
        ("apiKey", final_api_key),
        ("", ""),
        ("", ""),
        ("", ""),
        ("", ""),
    ];

    let limit_string;
    let mut limit = params.limit;
    if limit != 0 {
        if limit > 1000 {
            limit = 1000;
        }
        limit_string = limit.to_string();
        query_params[1] = ("limit", limit_string.as_str())
    }

    if params.history_type != "" {
        query_params[2] = ("type", params.history_type.as_str());
    }

    let timestamp_string;
    if params.timestamp.timestamp() != 0 {
        timestamp_string = params.timestamp.timestamp().to_string();
        query_params[3] = ("timestamp", timestamp_string.as_str());
    }

    if params.token != "" {
        query_params[4] = ("token", params.token.as_str());
    }

    let res = client.get(url).query(&query_params).send()?;

    res.json::<TokenHistory>()
}

pub fn get_address_transactions(
    api_key: &str,
    address: &str,
    params: &GetAddressTransactionsParams,
) -> Result<Vec<AddressTransaction>, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getAddressTransactions/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 4] =
        [("apiKey", final_api_key), ("", ""), ("", ""), ("", "")];

    let limit_string;
    let mut limit = params.limit;
    if limit != 0 {
        if limit > 1000 {
            limit = 1000;
        }
        limit_string = limit.to_string();
        query_params[1] = ("limit", limit_string.as_str())
    }

    let timestamp_string;
    if params.timestamp.timestamp() != 0 {
        timestamp_string = params.timestamp.timestamp().to_string();
        query_params[2] = ("timestamp", timestamp_string.as_str());
    }

    let show_zero_values = params.show_zero_values.to_string();
    query_params[3] = ("showZeroValues", show_zero_values.as_str());

    let res = client.get(url).query(&query_params).send()?;
    res.json::<Vec<AddressTransaction>>()
}

pub fn get_top_tokens(api_key: &str) -> Result<TopTokens, Error> {
    let client = reqwest::blocking::Client::new();
    let url = "https://api.ethplorer.io/getTopTokens";

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let res = client.get(url).query(&[("apiKey", final_api_key)]).send()?;

    res.json::<TopTokens>()
}

pub fn get_top(api_key: &str, params: &GetTopParams) -> Result<TopTokens, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getTop/");

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 4] =
        [("apiKey", final_api_key), ("", ""), ("", ""), ("", "")];

    let mut limit = params.limit;
    let limit_string;
    if limit != 0 {
        if limit > 1000 {
            limit = 1000;
        }
        limit_string = limit.to_string();
        query_params[1] = ("limit", limit_string.as_str())
    }

    if params.criteria != "" {
        query_params[2] = ("criteria", params.criteria.as_str());
    }

    let res = client.get(url).query(&query_params).send()?;
    res.json::<TopTokens>()
}

pub fn get_token_daily_price_history(
    api_key: &str,
    address: &str,
    mut period: u64,
) -> Result<TokenDailyPriceHistory, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getTokenPriceHistoryGrouped/") + address;

    let final_api_key;
    if api_key == "" {
        final_api_key = "freekey";
    } else {
        final_api_key = api_key;
    }

    let mut query_params: [(&str, &str); 2] = [("apiKey", final_api_key), ("", "")];

    let period_string;
    if period != 0 {
        if period > 90 {
            period = 90;
        }
        period_string = period.to_string();
        query_params[1] = ("period", period_string.as_str())
    }

    let res = client.get(url).query(&query_params).send()?;
    res.json::<TokenDailyPriceHistory>()
}
