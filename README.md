# cryptoprices-rs

## Info

A small lib able to fetch crypto coin prices and coin per currency amount.
The api is very small and currently supports:

### Tokens

[CoinGeckoTokensList](CoinGecko_Token_API_List.csv)

### Currencies

```rust
pub enum Currency {
    USD,
    EUR,
    ETH,
    BTC,
}
```

### API

- Get prices for coin uses the coingecko api to return a `CoinGeckoPrice` that contains currency values and coin amount per currency result.

```rust
pub async fn get_prices_for(token: &String, currency: &Currency) -> Result<CoinGeckoPrice, Error> {...}
```

```rust
pub struct CoinGeckoPrice {
    pub currency_price: String,
    pub coin_per_dollar: String,
}
```

- Get uniswap prices for uses the uniswap graphQL and returns a structure of `UniswapPairPrice` with all the relative to the pair information.

```rust
pub async fn get_uniswap_prices_for(
    pair_address: &str,
    version_provider: crate::uniswap::UniswapVersion,
    currency: &Currency,
) -> Result<crate::uniswap::UniswapPairPrice, Error> {...}
```

```rust
pub struct UniswapPairPrice {
    pub pair: String,
    pub token0: String,
    pub token0_price: String,
    pub token0_per_dollar: String,
    pub token1: String,
    pub token1_price: String,
    pub token1_per_dollar: String,
}
```

## Usage

### CoinGecko

```rust
let ethereum_prices = get_prices_for("Ethereum", &Currency::USD)?;
```

### Uniswap

```rust
let pair_address = "0xb4e16d0168e52d35cacd2c6185b44281ec28c9dc";
let uniswap_pair_price = get_uniswap_prices_for(
    pair_address,
    crate::uniswap::UniswapVersion::V2,
    &Currency::USD,
)?;
```
