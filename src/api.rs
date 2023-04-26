use futures::future;
use serde::Deserialize;

use anyhow::Result;

use crate::{Coin, Symbol, URL};

#[derive(Debug, Deserialize)]
struct Data {
    base: String,
    amount: String,
}

#[derive(Debug, Deserialize)]
struct Payload {
    data: Data,
}

pub async fn get_coins() -> Result<Vec<Coin>> {
    let urls: Vec<String> = Symbol::all()
        .into_iter()
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
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<Payload>().await {
            Ok(parsed) => {
                Ok(Coin {
                    symbol: Symbol::new(parsed.data.base.as_str()),
                    price: format_price(&parsed.data.amount),
                })
            }
            Err(_) => {
                Err("Response didn't match the structure we were expecting".to_owned())
            }
        },
        reqwest::StatusCode::NOT_FOUND => {
            Err("Not found".to_owned())
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            Err("Need to grab an API token".to_owned())
        }
        _ => Err("Uh oh! something unexpected happened".to_owned()),
    }
}

fn format_price(price: &str) -> String {
    let (dollars, cents) = price.split_once('.').unwrap_or(("0", "00"));

    let dollars: String = if dollars.len() < 4 {
        dollars.to_owned()
    } else {
        dollars
            .chars()
            .enumerate()
            .fold(String::new(), |mut acc, (i, c)| {
                acc.push(c);
                if dollars.len() % 3 == (i + 1) {
                    acc.push(',');
                }
                acc
            })
    };

    format!("${}.{}", dollars, cents)
}
