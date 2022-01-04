#[cfg(test)]
mod tests {
    use ethplorer::{
        api_key_param, get_address_history_config, get_address_info_config,
        get_address_transactions_config, get_last_block_config,
        get_token_daily_price_history_config, get_token_daily_transaction_count_config,
        get_token_history_config, get_token_info_config, get_tokens_new_config, get_top_config,
        get_top_token_holders_config, get_top_tokens_config, GetAddressHistoryParams,
        GetAddressInfoParams, GetAddressTransactionsParams, GetTokenHistoryParams, GetTopParams,
        GET_ADDRESS_HISTORY, GET_ADDRESS_INFO_ROUTE, GET_ADDRESS_TRANSACTIONS_ROUTE,
        GET_LAST_BLOCK_ROUTE, GET_TOKENS_NEW_ROUTE, GET_TOKEN_DAILY_TRANSACTION_COUNT_ROUTE,
        GET_TOKEN_HISTORY_ROUTE, GET_TOKEN_INFO_ROUTE, GET_TOKEN_PRICE_HISTORY_GROUPED_ROUTE,
        GET_TOP_ROUTE, GET_TOP_TOKENS_ROUTE, GET_TOP_TOKEN_HOLDERS_ROUTE, NETWORK,
    };

    #[test]
    fn api_key_param_works() {
        let key1 = api_key_param("");
        assert_eq!(("apiKey".to_string(), "freekey".to_string()), key1);

        let key2 = api_key_param("test");
        assert_eq!(("apiKey".to_string(), "test".to_string()), key2);
    }

    #[test]
    fn get_address_info_config_works() {
        let config = get_address_info_config(
            "",
            "0x0",
            &GetAddressInfoParams {
                token: "token".to_string(),
                show_eth_totals: false,
            },
        );
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_ADDRESS_INFO_ROUTE, "0x0"]);
        assert_eq!(
            config.params,
            vec![
                ("apiKey".to_string(), "freekey".to_string()),
                ("token".to_string(), "token".to_string()),
                ("showETHTotals".to_string(), "false".to_string()),
            ]
        );
        assert_eq!(
            config.to_string(),
            "https://api.ethplorer.io/getAddressInfo/0x0"
        );
    }

    #[test]
    fn get_token_info_config_works() {
        let config = get_token_info_config("", "0x0");
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_TOKEN_INFO_ROUTE, "0x0"]);
        assert_eq!(
            config.params,
            vec![("apiKey".to_string(), "freekey".to_string())]
        );
        assert_eq!(
            config.to_string(),
            "https://api.ethplorer.io/getTokenInfo/0x0"
        );
    }

    #[test]
    fn get_top_token_holders_config_works() {
        let config = get_top_token_holders_config("", "0x0", 100);
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_TOP_TOKEN_HOLDERS_ROUTE, "0x0"]);
        assert_eq!(
            config.params,
            vec![
                ("apiKey".to_string(), "freekey".to_string()),
                ("limit".to_string(), "100".to_string()),
            ]
        );
        assert_eq!(
            config.to_string(),
            "https://api.ethplorer.io/getTopTokenHolders/0x0"
        );
    }

    #[test]
    fn get_last_block_config_works() {
        let config = get_last_block_config("");
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_LAST_BLOCK_ROUTE]);
        assert_eq!(
            config.params,
            vec![("apiKey".to_string(), "freekey".to_string())]
        );
        assert_eq!(config.to_string(), "https://api.ethplorer.io/getLastBlock");
    }

    #[test]
    fn get_tokens_new_config_works() {
        let config = get_tokens_new_config("");
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_TOKENS_NEW_ROUTE]);
        assert_eq!(
            config.params,
            vec![("apiKey".to_string(), "freekey".to_string())]
        );
        assert_eq!(config.to_string(), "https://api.ethplorer.io/getTokensNew");
    }

    #[test]
    fn get_token_daily_transaction_count_config_works() {
        let config = get_token_daily_transaction_count_config("", "0x0", 50);
        assert_eq!(config.network, NETWORK);
        assert_eq!(
            config.routes,
            vec![GET_TOKEN_DAILY_TRANSACTION_COUNT_ROUTE, "0x0"]
        );
        assert_eq!(
            config.params,
            vec![
                ("apiKey".to_string(), "freekey".to_string()),
                ("period".to_string(), "50".to_string()),
            ]
        );
        assert_eq!(
            config.to_string(),
            "https://api.ethplorer.io/getTokenHistoryGrouped/0x0"
        );
    }

    #[test]
    fn get_token_history_config_works() {
        let config = get_token_history_config(
            "",
            "0x0",
            &GetTokenHistoryParams {
                history_type: "type".to_string(),
                limit: 50,
                timestamp: Default::default(),
            },
        );
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_TOKEN_HISTORY_ROUTE, "0x0"]);
        assert_eq!(
            config.params,
            vec![
                ("apiKey".to_string(), "freekey".to_string()),
                ("limit".to_string(), "50".to_string()),
                ("type".to_string(), "type".to_string()),
            ]
        );
        assert_eq!(
            config.to_string(),
            "https://api.ethplorer.io/getTokenHistory/0x0"
        );
    }

    #[test]
    fn get_address_history_config_works() {
        let config = get_address_history_config(
            "",
            "0x0",
            &GetAddressHistoryParams {
                history_type: "type".to_string(),
                limit: 50,
                timestamp: Default::default(),
                token: "token".to_string(),
            },
        );
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_ADDRESS_HISTORY, "0x0"]);
        assert_eq!(
            config.params,
            vec![
                ("apiKey".to_string(), "freekey".to_string()),
                ("limit".to_string(), "50".to_string()),
                ("type".to_string(), "type".to_string()),
                ("token".to_string(), "token".to_string()),
            ]
        );
        assert_eq!(
            config.to_string(),
            "https://api.ethplorer.io/getAddressHistory/0x0"
        );
    }

    #[test]
    fn get_address_transactions_config_works() {
        let config = get_address_transactions_config(
            "",
            "0x0",
            &GetAddressTransactionsParams {
                limit: 50,
                timestamp: Default::default(),
                show_zero_values: false,
            },
        );
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_ADDRESS_TRANSACTIONS_ROUTE, "0x0"]);
        assert_eq!(
            config.params,
            vec![
                ("apiKey".to_string(), "freekey".to_string()),
                ("limit".to_string(), "50".to_string()),
                ("showZeroValues".to_string(), "false".to_string()),
            ]
        );
        assert_eq!(
            config.to_string(),
            "https://api.ethplorer.io/getAddressTransactions/0x0"
        );
    }

    #[test]
    fn get_top_tokens_config_works() {
        let config = get_top_tokens_config("");
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_TOP_TOKENS_ROUTE]);
        assert_eq!(
            config.params,
            vec![("apiKey".to_string(), "freekey".to_string())]
        );
        assert_eq!(config.to_string(), "https://api.ethplorer.io/getTopTokens");
    }

    #[test]
    fn get_top_config_works() {
        let config = get_top_config(
            "",
            &GetTopParams {
                limit: 50,
                criteria: "crit".to_string(),
            },
        );
        assert_eq!(config.network, NETWORK);
        assert_eq!(config.routes, vec![GET_TOP_ROUTE]);
        assert_eq!(
            config.params,
            vec![
                ("apiKey".to_string(), "freekey".to_string()),
                ("limit".to_string(), "50".to_string()),
                ("criteria".to_string(), "crit".to_string()),
            ]
        );
        assert_eq!(config.to_string(), "https://api.ethplorer.io/getTop");
    }

    #[test]
    fn get_token_daily_price_history_config_works() {
        let config = get_token_daily_price_history_config("", "0x0", 50);
        assert_eq!(config.network, NETWORK);
        assert_eq!(
            config.routes,
            vec![GET_TOKEN_PRICE_HISTORY_GROUPED_ROUTE, "0x0"]
        );
        assert_eq!(
            config.params,
            vec![
                ("apiKey".to_string(), "freekey".to_string()),
                ("period".to_string(), "50".to_string()),
            ]
        );
        assert_eq!(
            config.to_string(),
            "https://api.ethplorer.io/getTokenPriceHistoryGrouped/0x0"
        );
    }
}
