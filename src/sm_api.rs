use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;
use reqwest::get;

use std::collections::HashMap;

pub(crate) struct SteamMarketItemPrice {
    date: DateTime<Utc>,
    price: f64,
}
impl SteamMarketItemPrice {
    fn new(date_str: &str, price: f64) -> Result<Self> {
        let date = Self::steam_market_string_to_datetime(date_str)?;
        Ok(Self { date, price })
    }
    fn steam_market_string_to_datetime(date: &str) -> Result<DateTime<Utc>> {
        let month_str_to_month_int: HashMap<&str, u32> = HashMap::from([
            ("Jan", 1),
            ("Feb", 2),
            ("Mar", 3),
            ("Apr", 4),
            ("May", 5),
            ("Jun", 6),
            ("Jul", 7),
            ("Aug", 8),
            ("Sep", 9),
            ("Oct", 10),
            ("Nov", 11),
            ("Dec", 12),
        ]);
        let date_split: Vec<&str> = date.split(' ').collect();
        let year = date_split[2].parse::<i32>()?;
        let month = month_str_to_month_int[date_split[0]];
        let day = date_split[1].parse::<u32>()?;
        let hour = date_split[3].trim_end_matches(':').parse::<u32>()?;

        Ok(Utc.with_ymd_and_hms(year, month, day, hour, 0, 0).unwrap())
    }
}
pub(crate) struct SteamMarketItem {
    market_prices: Vec<SteamMarketItemPrice>,
}

impl SteamMarketItem {
    pub async fn new(game_id: u16, name: &str) -> Result<Self> {
        let steam_market_item_url = format!(
            "https://steamcommunity.com/market/listings/{}/{}",
            game_id, name
        );
        let market_prices = Self::load_market_prices(&steam_market_item_url).await?;
        Ok(Self { market_prices })
    }
    async fn load_market_prices(steam_market_item_url: &str) -> Result<Vec<SteamMarketItemPrice>> {
        let page_source = get(steam_market_item_url).await?.text().await?;
        let re = Regex::new(r#"\["([A-Za-z\s\d:+]+)".([\d\.]+),"(\d+)"\]"#)?;
        let mut smips: Vec<SteamMarketItemPrice> = vec![];

        for cap in re.captures_iter(&page_source) {
            let date_str = &cap[1];
            let price_str = &cap[2];
            let price = price_str.parse()?;
            smips.push(SteamMarketItemPrice::new(date_str, price)?)
        }
        Ok(smips)
    }

    pub fn print_all(&self) {
        for mp in &self.market_prices {
            println!("date: {}, price: {}", mp.date.with_timezone(&Utc), mp.price);
        }
    }
    pub fn get_latest_price(&self) -> Result<f64> {
        if self.market_prices.len() <= 0 {
            panic!("Error retrieving market prices.");
        }
        let latest_price = self.market_prices.last().unwrap();
        Ok(latest_price.price)
    }

    pub fn get_latest_prices(&self) -> Result<Vec<(String, f64)>> {
        if self.market_prices.len() <= 0 {
            panic!("Error retrieving market prices.");
        }
        let mut prices: Vec<(String, f64)> = vec![];
        for mp in &self.market_prices {
            prices.push((mp.date.to_string(), mp.price));
        }
        Ok(prices)
    }
}
