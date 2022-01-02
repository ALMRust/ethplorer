#![feature(in_band_lifetimes)]

use num::clamp;
pub use crate::types::*;
use reqwest::Error;
use serde::de::DeserializeOwned;
pub use crate::consts::*;

pub mod types;
pub mod consts;

// TODO: Add status code error handling
// TODO: use macro for repeat values

pub fn api_key_param(api_key: &str) -> (String, String) {
    let out;
    if api_key == "" {
        out = String::from("freekey");
    } else {
        out = api_key.to_string();
    }
    (String::from("apiKey"), out)
}

// Get Address Info

pub fn get_address_info_config(
    api_key: &str,
    address: &str,
    in_params: &GetAddressInfoParams,
) -> RequestConfig {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    if in_params.token != "" {
        params.push(("token".to_string(), in_params.token.clone()));
    }

    let eth_totals = in_params.show_eth_totals.to_string();
    params.push(("showETHTotals".to_string(), eth_totals));

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_ADDRESS_INFO_ROUTE.to_string(), address.to_string()],
        params,
    }
}

// Get Token Info

pub fn get_token_info_config(api_key: &str, address: &str) -> RequestConfig {
    let key = api_key_param(api_key);
    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOKEN_INFO_ROUTE.to_string(), address.to_string()],
        params: vec![key],
    }
}

// Get Top Token Holders

pub fn get_top_token_holders_config(api_key: &str, address: &str, mut limit: u64) -> RequestConfig {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit".to_string(), limit.to_string()))
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOP_TOKEN_HOLDERS_ROUTE.to_string(), address.to_string()],
        params,
    }
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

// Get Token New

pub fn get_tokens_new_config(
    api_key: &str,
) -> RequestConfig {
    let key = api_key_param(api_key);
    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOKENS_NEW_ROUTE.to_string()],
        params: vec![key],
    }
}

// Get Token Daily Transaction Count

pub fn get_token_daily_transaction_count_config(api_key: &str, address: &str, mut period: u64) -> RequestConfig {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    if period != 0 {
        period = clamp(period, 0, 90);
        params.push(("period".to_string(), period.to_string()))
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOKEN_DAILY_TRANSACTION_COUNT_ROUTE.to_string(), address.to_string()],
        params,
    }
}

// Get Token History

pub fn get_token_history_config(api_key: &str, address: &str, in_params: &GetTokenHistoryParams) -> RequestConfig {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    let mut limit = in_params.limit;
    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit".to_string(), limit.to_string()))
    }

    if in_params.history_type != "" {
        params.push(("type".to_string(), in_params.history_type.clone()));
    }

    let timestamp = in_params.timestamp.timestamp();
    if timestamp != 0 {
        params.push(("timestamp".to_string(), timestamp.to_string()));
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOKEN_HISTORY_ROUTE.to_string(), address.to_string()],
        params,
    }
}

// Get Address History

pub fn get_address_history_config(api_key: &str, address: &str, in_params: &GetAddressHistoryParams) -> RequestConfig {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    let mut limit = in_params.limit;
    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit".to_string(), limit.to_string()))
    }

    if in_params.history_type != "" {
        params.push(("type".to_string(), in_params.history_type.clone()));
    }

    let timestamp = in_params.timestamp.timestamp();
    if timestamp != 0 {
        params.push(("timestamp".to_string(), timestamp.to_string()));
    }

    if in_params.token != "" {
        params.push(("token".to_string(), in_params.token.clone()));
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_ADDRESS_HISTORY.to_string(), address.to_string()],
        params,
    }
}

// Get Address Transactions

pub fn get_address_transactions_config(api_key: &str, address: &str, in_params: &GetAddressTransactionsParams) -> RequestConfig {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    let mut limit = in_params.limit;
    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit".to_string(), limit.to_string()))
    }

    let timestamp = in_params.timestamp.timestamp();
    if timestamp != 0 {
        params.push(("timestamp".to_string(), timestamp.to_string()));
    }

    let show_zero_values = in_params.show_zero_values.to_string();
    params.push(("showZeroValues".to_string(), show_zero_values));

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_ADDRESS_TRANSACTIONS_ROUTE.to_string(), address.to_string()],
        params,
    }
}

// Get Top Tokens

pub fn get_top_tokens_config(
    api_key: &str,
) -> RequestConfig {
    let key = api_key_param(api_key);
    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOP_TOKENS_ROUTE.to_string()],
        params: vec![key],
    }
}

// Get Top

pub fn get_top_config(api_key: &str, in_params: &GetTopParams) -> RequestConfig {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    let mut limit = in_params.limit;
    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit".to_string(), limit.to_string()))
    }

    if in_params.criteria != "" {
        params.push(("criteria".to_string(), in_params.criteria.clone()));
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOP_ROUTE.to_string()],
        params,
    }
}

// Get Token Daily Price History

pub fn get_token_daily_price_history_config(
    api_key: & str,
    address: &str,
    mut period: u64
) -> RequestConfig {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    if period != 0 {
        period = clamp(period, 0, 90);
        params.push(("period".to_string(), period.to_string()))
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOKEN_PRICE_HISTORY_GROUPED_ROUTE.to_string(), address.to_string()],
        params,
    }
}
