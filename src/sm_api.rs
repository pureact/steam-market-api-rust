use anyhow::Result;
use chrono::{DateTime, TimeZone, Utc};
use regex::Regex;
use reqwest::get;

use std::collections::HashMap;

pub trait ToUtcDateTime {
    fn to_utc_datetime(&self) -> Result<DateTime<Utc>>;
}

impl ToUtcDateTime for &str {
    fn to_utc_datetime(&self) -> Result<DateTime<Utc>> {
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
        let date_split: Vec<&str> = self.split(' ').collect();
        let year = date_split[2].parse::<i32>()?;
        let month = month_str_to_month_int[date_split[0]];
        let day = date_split[1].parse::<u32>()?;
        let hour = date_split[3].trim_end_matches(':').parse::<u32>()?;

        Ok(Utc.with_ymd_and_hms(year, month, day, hour, 0, 0).unwrap())
    }
}

impl ToUtcDateTime for DateTime<Utc> {
    fn to_utc_datetime(&self) -> Result<DateTime<Utc>> {
        Ok(*self)
    }
}

pub struct SteamMarketItemPrice {
    pub date: DateTime<Utc>,
    pub price: f64,
}
impl SteamMarketItemPrice {
    fn new(date: impl ToUtcDateTime, price: f64) -> Result<Self> {
        let date = date.to_utc_datetime()?;
        Ok(Self { date, price })
    }
}
pub struct SteamMarketItem {
    pub game_id: u16,
    pub name: String,
    pub market_prices: Vec<SteamMarketItemPrice>,
}

impl SteamMarketItem {
    pub async fn new(game_id: u16, name: &str) -> Result<Self> {
        let steam_market_item_url = format!(
            "https://steamcommunity.com/market/listings/{}/{}",
            game_id, name
        );
        let market_prices = Self::load_market_prices(&steam_market_item_url).await?;
        if market_prices.len() == 0 {
            panic!("Error retrieving market prices.")
        }
        Ok(Self {
            game_id: game_id,
            name: name.to_string(),
            market_prices: market_prices,
        })
    }
    async fn load_market_prices(steam_market_item_url: &str) -> Result<Vec<SteamMarketItemPrice>> {
        let page_source = get(steam_market_item_url).await?.text().await?;
        let re = Regex::new(r#"\["([A-Za-z\s\d:+]+)".([\d\.]+),"(\d+)"\]"#)?;
        let mut smips: Vec<SteamMarketItemPrice> = vec![];

        for cap in re.captures_iter(&page_source) {
            let date_str = &cap[1];
            let price: f64 = cap[2].parse()?;
            smips.push(SteamMarketItemPrice::new(date_str, price)?)
        }
        Ok(smips)
    }

    pub fn get_current_price(&self) -> Result<f64> {
        let latest_price = self.market_prices.last().unwrap();
        Ok(latest_price.price)
    }

    pub fn get_all_prices(&self) -> Result<Vec<(String, f64)>> {
        let mut prices: Vec<(String, f64)> = vec![];
        for mp in &self.market_prices {
            prices.push((mp.date.to_string(), mp.price));
        }
        Ok(prices)
    }
}
