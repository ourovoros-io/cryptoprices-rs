/// Represents the [`Currency`] type in the system
#[derive(Debug)]
pub enum Currency {
    USD,
    EUR,
    ETH,
    BTC,
}

impl Default for Currency {
    /// `Default` is [`Currency::USD`]
    fn default() -> Self {
        Self::USD
    }
}

impl Currency {
    /// Returns a [`Currency::USD`]
    pub fn new() -> Self {
        Self::default()
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Currency::USD => write!(f, "usd"),
            Currency::EUR => write!(f, "eur"),
            Currency::ETH => write!(f, "eth"),
            Currency::BTC => write!(f, "btc"),
        }
    }
}
