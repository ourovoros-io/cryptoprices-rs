//! A simple lib to get prices for crypto tokens against [`crate::currency::Currency`]
//! It will return a tuple of the price of token in [`crate::currency::Currency`]
//! and the price of 1 [`crate::currency::Currency`] in [`crate::coingecko::Token`] amount.
//! For Uniswap Pairs the return type is more complex and returns a [`crate::uniswap::UniswapPairPrice`]
//! that holds information about the [`crate::uniswap::Pair`] and the underlying tokens [`crate::uniswap::Token0`] and [`crate::uniswap::Token1`]

use coingecko::{create_request, token_price_per_currency, CoinGeckoPrice};
use currency::Currency;

pub mod coingecko;
pub mod currency;
pub mod uniswap;

type Error = Box<dyn std::error::Error + Send + Sync>;

/// The function uses the coingecko API to fetch price for the given [`crate::coingecko::Token`] and [`crate::Currency`]
/// It will return a [`crate::coingecko::CoinGeckoPrice`] result
#[tokio::main]
pub async fn get_prices_for(token: &str, currency: &Currency) -> Result<CoinGeckoPrice, Error> {
    let price = create_request(token, currency).await?;
    let coin_per_dollar = token_price_per_currency(&price, currency).await?;

    Ok(CoinGeckoPrice {
        currency_price: price.to_string(),
        coin_per_dollar: coin_per_dollar.to_string(),
    })
}

/// The function uses the uniswap api for [`crate::uniswap::UniswapVersion::V2`] and [`crate::uniswap::UniswapVersion::V2`]
/// Returns a result of [`crate::uniswap::UniswapPairPrice`]
#[tokio::main]
pub async fn get_uniswap_prices_for(
    pair_address: &str,
    version_provider: crate::uniswap::UniswapVersion,
    currency: &Currency,
) -> Result<crate::uniswap::UniswapPairPrice, Error> {
    // Todo: precision should be dynamic based on pair
    let pair = crate::uniswap::create_uniswap_request(pair_address, version_provider).await?;
    let coin_per_dollar_token0 =
        token_price_per_currency(&pair.token0_price.parse::<f64>()?, currency).await?;
    let coin_per_dollar_token1 =
        token_price_per_currency(&pair.token1_price.parse::<f64>()?, currency).await?;
    Ok(crate::uniswap::UniswapPairPrice {
        pair: pair.id,
        token0: pair.token0.id,
        token0_price: pair.token0_price,
        token0_per_dollar: coin_per_dollar_token0.to_string(),
        token1: pair.token1.id,
        token1_price: pair.token1_price,
        token1_per_dollar: coin_per_dollar_token1.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_usd_prices_coingecko_ethereum() -> Result<(), Error> {
        let eth_string = "Ethereum";
        let ethereum_prices = get_prices_for(eth_string, &Currency::USD)?;
        println!(
            "[{}] is : {:.} in [{}] and coin price per [{}] : {:.}",
            eth_string,
            ethereum_prices.currency_price,
            Currency::USD,
            Currency::USD,
            ethereum_prices.coin_per_dollar
        );
        Ok(())
    }

    #[test]
    fn get_usd_prices_coingecko_bitcoin() -> Result<(), Error> {
        let btc_string = "Bitcoin";
        let btc_prices = get_prices_for(btc_string, &Currency::USD)?;
        println!(
            "[{}] is : {:.} in [{}] and coin price per [{}] : {:.}",
            btc_string,
            btc_prices.currency_price,
            Currency::USD,
            Currency::USD,
            btc_prices.coin_per_dollar
        );
        Ok(())
    }

    #[test]
    fn get_eth_prices_coingecko_aave() -> Result<(), Error> {
        let aave_string = "Aave";
        let aave_prices = get_prices_for(aave_string, &Currency::ETH)?;
        println!(
            "[{}] is : {:.} in [{}] and coin price per [{}] : {:.}",
            aave_string,
            aave_prices.currency_price,
            Currency::ETH,
            Currency::ETH,
            aave_prices.coin_per_dollar
        );

        Ok(())
    }

    #[test]
    fn get_btc_prices_coingecko_ethereum() -> Result<(), Error> {
        let eth_string = "Ethereum";
        let eth_prices = get_prices_for(eth_string, &Currency::BTC)?;
        println!(
            "[{}] is : {:.} in [{}] and coin price per [{}] : {:.}",
            eth_string,
            eth_prices.currency_price,
            Currency::BTC,
            Currency::BTC,
            eth_prices.coin_per_dollar
        );

        Ok(())
    }

    #[test]
    fn get_prices_uniswap() -> Result<(), Error> {
        let pair_address = "0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc";
        let uniswap_pair_price = get_uniswap_prices_for(
            pair_address,
            crate::uniswap::UniswapVersion::V2,
            &Currency::USD,
        )?;
        println!(
            "Uniswap Pair : {} - Token0 Address : {} - Token0 Balance: {} - Token0 Price Per USD : {}",
            pair_address, uniswap_pair_price.token0, uniswap_pair_price.token0_price, uniswap_pair_price.token0_per_dollar,
        );
        println!(
            "Uniswap Pair : {} - Token1 Address : {} - Token1 Balance: {} - Token1 Price Per USD : {}",
            pair_address, uniswap_pair_price.token1, uniswap_pair_price.token1_price, uniswap_pair_price.token1_per_dollar,
        );

        let pair_address2 = "0xbb2b8038a1640196fbe3e38816f3e67cba72d940";
        let uniswap_pair_price = get_uniswap_prices_for(
            pair_address2,
            crate::uniswap::UniswapVersion::V2,
            &Currency::USD,
        )?;
        println!(
            "Uniswap Pair : {} - Token0 Address : {} - Token0 Balance: {} - Token0 Price Per USD : {}",
            pair_address2, uniswap_pair_price.token0, uniswap_pair_price.token0_price, uniswap_pair_price.token0_per_dollar,
        );
        println!(
            "Uniswap Pair : {} - Token1 Address : {} - Token1 Balance: {} - Token1 Price Per USD : {}",
            pair_address2, uniswap_pair_price.token1, uniswap_pair_price.token1_price, uniswap_pair_price.token1_per_dollar,
        );

        let pair_address3 = "0x5777d92f208679db4b9778590fa3cab3ac9e2168";
        let uniswap_pair_price = get_uniswap_prices_for(
            pair_address3,
            crate::uniswap::UniswapVersion::V3,
            &Currency::USD,
        )?;
        println!(
            "Uniswap Pair : {} - Token0 Address : {} - Token0 Balance: {} - Token0 Price Per USD : {}",
            pair_address3, uniswap_pair_price.token0, uniswap_pair_price.token0_price, uniswap_pair_price.token0_per_dollar,
        );
        println!(
            "Uniswap Pair : {} - Token1 Address : {} - Token1 Balance: {} - Token1 Price Per USD : {}",
            pair_address3, uniswap_pair_price.token1, uniswap_pair_price.token1_price, uniswap_pair_price.token1_per_dollar,
        );
        Ok(())
    }
}
