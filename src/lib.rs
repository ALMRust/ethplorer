use crate::types::{AddressInfo, AddressTransaction, GetAddressHistoryParams, GetAddressTokenInfoParams, GetAddressTransactionsParams, GetTokenHistoryParams, LastBlock, TokenDailyTransactionCounts, TokenHistory, TokenInfo, TopTokenHolders, TopTokens, GetTopParams, TokenDailyPriceHistory};
use reqwest::Error;

pub mod types;
mod consts;

// TODO: Add status code error handling
// TODO: use macro for repeat values

pub fn get_address_token_info(
    address: &str,
    params: &GetAddressTokenInfoParams,
    api_key: &str,
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

    let query = client.get(url).query(&query_params).send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let counts = match res.json::<AddressInfo>() {
        Ok(txs_counts) => txs_counts,
        Err(e) => return Err(e),
    };

    Ok(counts)
}

pub fn get_token_info(address: &str, api_key: &str) -> Result<TokenInfo, Error> {
    let client = reqwest::blocking::Client::new();
    let url = String::from("https://api.ethplorer.io/getTokenInfo/") + address;

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

    let block = match res.json::<types::TokenInfo>() {
        Ok(last_block) => last_block,
        Err(e) => return Err(e),
    };

    Ok(block)
}

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

    let mut query_params: [(&str, &str); 2] = [("apiKey", final_api_key), ("", "")];

    let limit_string;
    if limit != 0 {
        if limit > 1000 {
            limit = 1000;
        }
        limit_string = limit.to_string();
        query_params[1] = ("limit", limit_string.as_str())
    }

    let query = client.get(url).query(&query_params).send();

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

pub fn get_token_new(api_key: &str) -> Result<Vec<TokenInfo>, Error> {
    let client = reqwest::blocking::Client::new();
    let url = "https://api.ethplorer.io/getTokensNew";

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

    let block = match res.json::<Vec<TokenInfo>>() {
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

    let mut query_params: [(&str, &str); 2] = [("apiKey", final_api_key), ("", "")];

    let period_string;
    if period != 0 {
        if period > 90 {
            period = 90;
        }
        period_string = period.to_string();
        query_params[1] = ("period", period_string.as_str())
    }

    let query = client.get(url).query(&query_params).send();

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

pub fn get_token_history(
    address: &str,
    params: &GetTokenHistoryParams,
    api_key: &str,
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

    let query = client.get(url).query(&query_params).send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let counts = match res.json::<TokenHistory>() {
        Ok(txs_counts) => txs_counts,
        Err(e) => return Err(e),
    };

    Ok(counts)
}

pub fn get_address_history(
    address: &str,
    params: &GetAddressHistoryParams,
    api_key: &str,
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

    let query = client.get(url).query(&query_params).send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let counts = match res.json::<TokenHistory>() {
        Ok(txs_counts) => txs_counts,
        Err(e) => return Err(e),
    };

    Ok(counts)
}

pub fn get_address_transactions(
    address: &str,
    params: &GetAddressTransactionsParams,
    api_key: &str,
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

    let query = client.get(url).query(&query_params).send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let counts = match res.json::<Vec<AddressTransaction>>() {
        Ok(txs_counts) => txs_counts,
        Err(e) => return Err(e),
    };

    Ok(counts)
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

    let query = client.get(url).query(&[("apiKey", final_api_key)]).send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let block = match res.json::<TopTokens>() {
        Ok(last_block) => last_block,
        Err(e) => return Err(e),
    };

    Ok(block)
}

pub fn get_top(params: &GetTopParams, api_key: &str) -> Result<TopTokens, Error> {
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

    let query = client.get(url).query(&query_params).send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let block = match res.json::<TopTokens>() {
        Ok(last_block) => last_block,
        Err(e) => return Err(e),
    };

    Ok(block)
}

pub fn get_token_daily_price_history(
    address: &str,
    mut period: u64,
    api_key: &str,
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

    let query = client.get(url).query(&query_params).send();

    let res = match query {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    let counts = match res.json::<TokenDailyPriceHistory>() {
        Ok(txs_counts) => txs_counts,
        Err(e) => return Err(e),
    };

    Ok(counts)
}
