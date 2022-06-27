use serde::{Deserialize, Serialize};

/// Represents the return of the [`crate::get_uniswap_prices_for`] function
pub struct UniswapPairPrice {
    pub pair: String,
    pub token0: String,
    pub token0_price: String,
    pub token0_per_dollar: String,
    pub token1: String,
    pub token1_price: String,
    pub token1_per_dollar: String,
}

/// Represents the current supported versions of uniswap
pub enum UniswapVersion {
    V2,
    V3,
}

impl UniswapVersion {
    /// Returns the [`UniswapVersion`] version as a [`usize`]
    pub fn get_version(&self) -> usize {
        match self {
            UniswapVersion::V2 => 2,
            UniswapVersion::V3 => 3,
        }
    }
}

impl Default for UniswapVersion {
    /// Returns the [`UniswapVersion::V2`] as the default version
    fn default() -> Self {
        Self::V2
    }
}

impl ToString for UniswapVersion {
    /// Returns the [`UniswapVersion`] as [`String`]
    fn to_string(&self) -> String {
        match self {
            UniswapVersion::V2 => {
                "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v2".to_string()
            }
            UniswapVersion::V3 => {
                "https://api.thegraph.com/subgraphs/name/uniswap/uniswap-v3".to_string()
            }
        }
    }
}

/// Creates a request against the uniswap api that returns the [`Pair`] struct of info for the [`pair_address`] given
pub(crate) async fn create_uniswap_request(
    pair_address: &str,
    version_provider: crate::uniswap::UniswapVersion,
) -> Result<Pair, crate::Error> {
    let ident = match version_provider {
        UniswapVersion::V2 => "pair".to_string(),
        UniswapVersion::V3 => "pool".to_string(),
    };
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let query = format!(
        r#"{{ {}(id: "{}") {{
            id
            token0 {{
              id
            }}
            token1 {{
              id
            }}
            token0Price
            token1Price
          }} }}"#,
        ident, pair_address
    );
    let data = QLQuery { query };
    let data_str = serde_json::to_string(&data)?;
    let res = reqwest::Client::new()
        .post(version_provider.to_string())
        .headers(headers)
        .body(data_str)
        .send()
        .await?
        .json::<UniswapResponse>()
        .await?;
    Ok(match res.data.inner {
        InnerData::V2 { pair } => pair,
        InnerData::V3 { pool } => pool,
    })
}

/// Represents a query against the uniswap graphQL
#[derive(Serialize, Deserialize)]
struct QLQuery {
    query: String,
}

/// Represents the return from uniswap api request
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniswapResponse {
    pub data: Data,
}

/// Represents the incoming [`Data`] and contains a flatten untagged [`InnerData`] enum
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Data {
    #[serde(flatten)]
    inner: InnerData,
}

/// Represents the different [`UniswapVersion`] structures
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InnerData {
    #[serde(rename = "pair")]
    V2 { pair: Pair },
    #[serde(rename = "pool")]
    V3 { pool: Pair },
}

impl Default for InnerData {
    fn default() -> Self {
        Self::V2 {
            pair: Pair::default(),
        }
    }
}

/// Represents the uniswap pair
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pair {
    pub id: String,
    pub token0: Token0,
    pub token1: Token1,
    #[serde(rename = "token0Price")]
    pub token0_price: String,
    #[serde(rename = "token1Price")]
    pub token1_price: String,
}

/// Represents token0 of a uniswap [`Pair`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token0 {
    pub id: String,
}

/// Represents token1 of a uniswap [`Pair`]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token1 {
    pub id: String,
}
