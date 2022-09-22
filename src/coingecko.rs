use serde_derive::Deserialize;
use serde_json::Value;

use crate::currency::Currency;

/// Represents the return type for a coingecko price derived from the api in form of [`crate::currency::Currency`]
/// and in coin per currency -> 1 currency to x amount of token
#[derive(Debug)]
pub struct CoinGeckoPrice {
    pub currency_price: String,
    pub coin_per_dollar: String,
}

/// Calculates the [`crate::Currency`] price per [`Token`]
pub async fn token_price_per_currency(
    token_price: &f64,
    currency: &Currency,
) -> Result<f64, crate::Error> {
    match currency {
        Currency::USD => Ok(1.0 * 1.0 / token_price),
        Currency::EUR => todo!(),
        Currency::ETH => {
            let ether_price = create_request(None, "Ethereum", &Currency::USD).await?;
            Ok(token_price / ether_price)
        }
        Currency::BTC => {
            let btc_price = create_request(None, "Bitcoin", &Currency::BTC).await?;
            Ok(token_price / btc_price)
        }
    }
}

pub fn get_currency_value(
    value: &Value,
    token_id: &str,
    currency: &Currency,
) -> Result<f64, crate::Error> {
    let out = &value[token_id];
    match currency {
        Currency::USD => {
            let result: GeckoUsdResult = serde_json::from_value(out.clone())?;
            Ok(result.usd)
        }
        Currency::EUR => todo!(),
        Currency::ETH => {
            let result: GeckoEthResult = serde_json::from_value(out.clone())?;
            Ok(result.eth)
        }
        Currency::BTC => {
            let result: GeckoBtcResult = serde_json::from_value(out.clone())?;
            Ok(result.btc)
        }
    }
}

/// Creates a request against the coingecko api that returns the [`crate::currency::Currency`] value for the [`Coin`] given
pub async fn create_request(
    chain_type: Option<&str>,
    token: &str,
    currency: &Currency,
) -> Result<f64, crate::Error> {
    let token_list = TokenList::new()?;
    if chain_type.is_none() {
        let token = token_list.get_token_id(token)?;

        let resp = reqwest::get(format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}",
            token, currency
        ))
        .await?
        .json::<serde_json::Value>()
        .await?;

        return get_currency_value(&resp, &token, currency);
    }
    let resp = reqwest::get(format!(
        "https://api.coingecko.com/api/v3/simple/token_price/{}?contract_addresses={}&vs_currencies={}",
        chain_type.unwrap(), token, currency
    ))
    .await?
    .json::<serde_json::Value>()
    .await?;

    get_currency_value(&resp, token, currency)
}

pub struct TokenList(Vec<Token>);

impl TokenList {
    pub fn new() -> Result<Self, crate::Error> {
        let mut v = Vec::new();
        let mut rdr = csv::Reader::from_path("CoinGecko_Token_API_List.csv")?;
        for result in rdr.deserialize() {
            let token: Token = match result {
                Ok(t) => t,
                Err(_) => continue,
            };
            v.push(token);
        }

        Ok(Self(v))
    }

    pub fn get_token_id(&self, token: &str) -> Result<String, crate::Error> {
        for t in &self.0 {
            if t.name == token {
                return Ok(t.id.clone());
            }
        }
        Ok("".to_string())
    }
}

#[derive(Debug, Deserialize)]
pub struct Token {
    #[serde(alias = "Id")]
    id: String,
    #[serde(alias = "Symbol")]
    symbol: String,
    #[serde(alias = "Name")]
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct GeckoUsdResult {
    usd: f64,
}

#[derive(Debug, Deserialize)]
pub struct GeckoEthResult {
    eth: f64,
}

#[derive(Debug, Deserialize)]
pub struct GeckoBtcResult {
    btc: f64,
}
