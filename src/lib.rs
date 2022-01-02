#![feature(in_band_lifetimes)]

use num::clamp;
use crate::types::*;
use reqwest::Error;
use serde::de::DeserializeOwned;
use crate::consts::*;

pub mod types;
mod consts;

// TODO: Add status code error handling
// TODO: use macro for repeat values

pub fn api_key_param(api_key: &str) -> (&str, String) {
    let out;
    if api_key == "" {
        out = String::from("freekey");
    } else {
        out = String::from(api_key);
    }
    ("apiKey", out)
}

#[cfg(test)]
mod tests {
    use crate::{api_key_param, GET_ADDRESS_INFO_ROUTE, get_token_info_config, GET_TOKEN_INFO_ROUTE, get_top_token_holders_config, GET_TOP_TOKEN_HOLDERS_ROUTE, NETWORK};

    #[test]
    fn api_key_param_works() {
        let key1 = api_key_param("");
        assert_eq!(("apiKey", String::from("freekey")), key1);

        let key2 = api_key_param("test");
        assert_eq!(("apiKey", String::from("test")), key2);
    }

    use crate::{get_address_info_config, GetAddressInfoParams};

    #[test]
    fn get_address_info_config_works() {
        let token = "token".to_string();
        let params = GetAddressInfoParams {
            token,
            show_eth_totals: false
        };
        let config = get_address_info_config("", "0x0", &params);
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_ADDRESS_INFO_ROUTE, "0x0"]);
        assert_eq!(config.params, vec![("apiKey", String::from("freekey")), ("token", String::from("token")), ("showETHTotals", String::from("false"))]);
        assert_eq!(config.to_string(), "https://api.ethplorer.io/getAddressInfo/0x0");
    }

    #[test]
    fn get_token_info_config_works() {
        let config = get_token_info_config("", "0x0");
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_TOKEN_INFO_ROUTE, "0x0"]);
        assert_eq!(config.params, vec![("apiKey", String::from("freekey"))]);
        assert_eq!(config.to_string(), "https://api.ethplorer.io/getTokenInfo/0x0");
    }

    #[test]
    fn get_top_token_holders_config_works() {
        let config = get_top_token_holders_config("", "0x0", 100);
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_TOP_TOKEN_HOLDERS_ROUTE, "0x0"]);
        assert_eq!(config.params, vec![("apiKey", String::from("freekey")), ("limit", String::from("100"))]);
        assert_eq!(config.to_string(), "https://api.ethplorer.io/getTopTokenHolders/0x0");
    }
}

pub fn handle_request<T: DeserializeOwned>(config: RequestConfig) -> Result<T, Error> {
    let url = config.to_string();
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).query(&config.params).send()?;
    res.json::<T>()
}

// Get Address Info

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
    let config = get_address_info_config(api_key, address, params);
    handle_request(config)
}

// Get Token Info

pub fn get_token_info_config(api_key: &'a str, address: &'a str) -> RequestConfig<'a> {
    let key = api_key_param(api_key);
    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOKEN_INFO_ROUTE.to_string(), address.to_string()],
        params: vec![key],
    }
}

pub fn get_token_info(api_key: &str, address: &str) -> Result<TokenInfo, Error> {
    let config = get_token_info_config(api_key, address);
    handle_request(config)
}

// Get Top Token Holders

pub fn get_top_token_holders_config(api_key: &'a str, address: &'a str, mut limit: u64) -> RequestConfig<'a> {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit", limit.to_string()))
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOP_TOKEN_HOLDERS_ROUTE.to_string(), address.to_string()],
        params,
    }
}

pub fn get_top_token_holders(
    api_key: &str,
    address: &str,
    limit: u64,
) -> Result<TopTokenHolders, Error> {
    let config = get_top_token_holders_config(api_key, address, limit);
    handle_request(config)
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

pub fn get_tokens_new(api_key: &str) -> Result<Vec<TokenInfo>, Error> {
    let config = get_tokens_new_config(api_key);
    handle_request(config)
}

// Get Token Daily Transaction Count

pub fn get_token_daily_transaction_count_config(api_key: &'a str, address: &'a str, mut period: u64) -> RequestConfig<'a> {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    if period != 0 {
        period = clamp(period, 0, 90);
        params.push(("period", period.to_string()))
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOKEN_DAILY_TRANSACTION_COUNT_ROUTE.to_string(), address.to_string()],
        params,
    }
}

pub fn get_token_daily_transaction_count(
    api_key: &str,
    address: &str,
    period: u64,
) -> Result<TokenDailyTransactionCounts, Error> {
    let config = get_token_daily_transaction_count_config(api_key, address, period);
    handle_request(config)
}

// Get Token History

pub fn get_token_history_config(api_key: &'a str, address: &'a str, in_params: &GetTokenHistoryParams) -> RequestConfig<'a> {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    let mut limit = in_params.limit;
    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit", limit.to_string()))
    }

    if in_params.history_type != "" {
        params.push(("type", in_params.history_type.clone()));
    }

    let timestamp = in_params.timestamp.timestamp();
    if timestamp != 0 {
        params.push(("timestamp", timestamp.to_string()));
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOKEN_HISTORY_ROUTE.to_string(), address.to_string()],
        params,
    }
}

pub fn get_token_history(
    api_key: &str,
    address: &str,
    params: &GetTokenHistoryParams,
) -> Result<TokenHistory, Error> {
    let config = get_token_history_config(api_key, address, params);
    handle_request(config)
}

// Get Address History

pub fn get_address_history_config(api_key: &'a str, address: &'a str, in_params: &GetAddressHistoryParams) -> RequestConfig<'a> {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    let mut limit = in_params.limit;
    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit", limit.to_string()))
    }

    if in_params.history_type != "" {
        params.push(("type", in_params.history_type.clone()));
    }

    let timestamp = in_params.timestamp.timestamp();
    if timestamp != 0 {
        params.push(("timestamp", timestamp.to_string()));
    }

    if in_params.token != "" {
        params.push(("token", in_params.token.clone()));
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_ADDRESS_HISTORY.to_string(), address.to_string()],
        params,
    }
}

pub fn get_address_history(
    api_key: &str,
    address: &str,
    params: &GetAddressHistoryParams,
) -> Result<TokenHistory, Error> {
    let config = get_address_history_config(api_key, address, params);
    handle_request(config)
}

// Get Address Transactions

pub fn get_address_transactions_config(api_key: &'a str, address: &'a str, in_params: &GetAddressTransactionsParams) -> RequestConfig<'a> {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    let mut limit = in_params.limit;
    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit", limit.to_string()))
    }

    let timestamp = in_params.timestamp.timestamp();
    if timestamp != 0 {
        params.push(("timestamp", timestamp.to_string()));
    }

    let show_zero_values = in_params.show_zero_values.to_string();
    params.push(("showZeroValues", show_zero_values));

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_ADDRESS_TRANSACTIONS_ROUTE.to_string(), address.to_string()],
        params,
    }
}

pub fn get_address_transactions(
    api_key: &str,
    address: &str,
    params: &GetAddressTransactionsParams,
) -> Result<Vec<AddressTransaction>, Error> {
    let config = get_address_transactions_config(api_key, address, params);
    handle_request(config)
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

pub fn get_top_tokens(api_key: &str) -> Result<TopTokens, Error> {
    let config = get_top_tokens_config(api_key);
    handle_request(config)
}

// Get Top

pub fn get_top_config(api_key: &'a str, in_params: &GetTopParams) -> RequestConfig<'a> {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    let mut limit = in_params.limit;
    if limit != 0 {
        limit = clamp(limit, 0, 1000);
        params.push(("limit", limit.to_string()))
    }

    if in_params.criteria != "" {
        params.push(("criteria", in_params.criteria.clone()));
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOP_ROUTE.to_string()],
        params,
    }
}

pub fn get_top(api_key: &str, params: &GetTopParams) -> Result<TopTokens, Error> {
    let config = get_top_config(api_key, params);
    handle_request(config)
}

// Get Token Daily Price History

pub fn get_token_daily_price_history_config(
    api_key: &'a str,
    address: &str,
    mut period: u64
) -> RequestConfig<'a> {
    let key = api_key_param(api_key);
    let mut params = vec![key];

    if period != 0 {
        period = clamp(period, 0, 90);
        params.push(("period", period.to_string()))
    }

    RequestConfig {
        network: NETWORK.to_string(),
        routes: vec![GET_TOKEN_PRICE_HISTORY_GROUPED_ROUTE.to_string(), address.to_string()],
        params,
    }
}

pub fn get_token_daily_price_history(
    api_key: &str,
    address: &str,
    period: u64,
) -> Result<TokenDailyPriceHistory, Error> {
    let config = get_token_daily_price_history_config(api_key, address, period);
    handle_request(config)
}
