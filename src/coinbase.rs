use futures::future;
use serde::Deserialize;

use anyhow::Result;

use crate::{Coin, Symbol, URL};

#[derive(Deserialize)]
struct Data {
    base: String,
    amount: String,
}

#[derive(Deserialize)]
struct Payload {
    data: Data,
}

pub async fn get_coins() -> Result<Vec<Coin>> {
    let urls: Vec<String> = Symbol::all()
        .iter()
        .map(|s| URL.replace("{}", &s.to_string(false)))
        .collect();

    let requests = urls.into_iter().map(fetch);
    let responses = future::join_all(requests).await;

    let coins: Result<Vec<Coin>> = responses
        .into_iter()
        .filter_map(|res| match res {
            Ok(coin) => Some(Ok(coin)),
            Err(e) => {
                println!("Error fetching coin: {}", e);
                None
            }
        })
        .collect();

    coins
}

async fn fetch(url: String) -> Result<Coin, String> {
    use reqwest::{header, Client, StatusCode};

    let client = Client::new();
    let response = client
        .get(url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        StatusCode::OK => match response.json::<Payload>().await {
            Ok(parsed) => Ok(Coin {
                symbol: Symbol::new(parsed.data.base.as_str()),
                price: format_price(&parsed.data.amount),
            }),
            Err(_) => Err("Response didn't match the structure we were expecting".to_owned()),
        },
        StatusCode::NOT_FOUND => Err("Not found".to_owned()),
        StatusCode::UNAUTHORIZED => Err("Need to grab an API token".to_owned()),
        _ => Err("Uh oh! something unexpected happened".to_owned()),
    }
}

fn format_price(price: &str) -> String {
    let (dollars, cents) = price.split_once('.').unwrap_or(("0", "00"));

    // no comma required for less than a thousand
    let dollars: String = if dollars.len() < 4 {
        dollars.to_owned()
    } else {
        dollars
            .chars()
            .rev()
            .enumerate()
            .fold(String::new(), |mut acc, (i, c)| {
                if i > 0 && i % 3 == 0 {
                    acc.push(',');
                }
                acc.push(c);
                acc
            })
            .chars()
            .rev()
            .collect()
    };

    format!("${}.{}", dollars, cents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_price_test() {
        let test_cases = [
            ("0.00", "$0.00"),
            ("1.23", "$1.23"),
            ("12.34", "$12.34"),
            ("123.45", "$123.45"),
            ("1234.56", "$1,234.56"),
            ("12345.67", "$12,345.67"),
            ("123456.78", "$123,456.78"),
            ("1000000.00", "$1,000,000.00"),
            ("10000000.00", "$10,000,000.00"),
            ("100000000.00", "$100,000,000.00"),
            ("10000000000", "$0.00"),
        ];

        for (input, expected) in &test_cases {
            let formatted = format_price(input);
            assert_eq!(formatted, *expected);
        }
    }
}
