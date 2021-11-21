use crate::types::LastBlock;
use reqwest::Error;

mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn get_block(api_key: String) -> Result<LastBlock, Error> {
    let client = reqwest::blocking::Client::new();
    let url = "https://api.ethplorer.io/getLastBlock";
    let mut final_api_key = api_key;

    if final_api_key == "" {
        final_api_key = String::from("freekey");
    }

    let query = client.get(url)
        .query(&[("apiKey", final_api_key)])
        .send();

    let res = match query {
        Ok(res)  => res,
        Err(e) => return Err(e),
    };

    let block = match res.json::<types::LastBlock>() {
        Ok(last_block)  => last_block,
        Err(e) => return Err(e),
    };

    Ok(block)
}
