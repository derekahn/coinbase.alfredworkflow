mod api;

use std::env;

use anyhow::Result;
use powerpack::Item;

pub const URL: &str = "https://api.coinbase.com/v2/prices/{}-USD/spot";

pub struct Coin {
    symbol: Symbol,
    price: String,
}

impl Coin {
    fn into_item(self) -> Item {
        let title = format!("{} {}", self.symbol.to_string(false), self.price);
        let url = format!("https://coinbase.com/price/{}", self.symbol.to_string(true));
        Item::new(title).arg(url)
    }
}

pub enum Symbol {
    ADA,
    BTC,
    DOGE,
    DOT,
    ETH,
    LTC,
    MATIC,
    SOL,
}

impl Symbol {
    pub fn new(s: &str) -> Symbol {
        use Symbol::*;
        match s {
            "ADA" => ADA,
            "BTC" => BTC,
            "ETH" => ETH,
            "DOT" => DOT,
            "LTC" => LTC,
            "DOGE" => DOGE,
            "MATIC" => MATIC,
            "SOL" => SOL,
            _ => BTC,
        }
    }

    pub fn to_string(&self, is_name: bool) -> String {
        use Symbol::*;

        match (self, is_name) {
            (ADA, false) => "ADA".to_owned(),
            (ADA, true) => "cardano".to_owned(),
            (BTC, false) => "BTC".to_owned(),
            (BTC, true) => "bitcoin".to_owned(),
            (ETH, false) => "ETH".to_owned(),
            (ETH, true) => "ethereum".to_owned(),
            (DOGE, false) => "DOGE".to_owned(),
            (DOGE, true) => "dogecoin".to_owned(),
            (DOT, false) => "DOT".to_owned(),
            (DOT, true) => "polkadot".to_owned(),
            (LTC, false) => "LTC".to_owned(),
            (LTC, true) => "litecoin".to_owned(),
            (MATIC, false) => "MATIC".to_owned(),
            (MATIC, true) => "polygon".to_owned(),
            (SOL, false) => "SOL".to_owned(),
            (SOL, true) => "solana".to_owned(),
        }
    }

    pub fn all() -> Vec<Symbol> {
        use Symbol::*;
        vec![ADA, BTC, ETH, DOT, DOGE, LTC, MATIC, SOL]
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let query = env::args()
        .nth(1)
        .as_deref()
        .map(str::trim)
        .map(str::to_uppercase);

    let filter_fn = |coin: &Coin| {
        coin.symbol
            .to_string(false)
            .contains(query.as_deref().unwrap_or(""))
    };

    let responses = api::get_coins().await.unwrap();
    let coins = responses.into_iter().filter(filter_fn);

    powerpack::output(coins.into_iter().map(Coin::into_item))?;

    Ok(())
}
