mod sm_api;
use anyhow::Result;
use colored::Colorize;
use serde::Deserialize;
use std::env;
use std::fs;
use tokio;

#[derive(Deserialize)]
struct SmConfig {
    items: Vec<SmItem>,
}
#[derive(Deserialize)]
struct SmItem {
    name: String,
    price: f64,
    game_id: u16,
}

async fn load_items() -> Result<SmConfig> {
    let cs_item_config: SmConfig = toml::from_str(&(fs::read_to_string("./cs_items.toml")?))?;
    Ok(cs_item_config)
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("\n{}\n", "Skin Tracker (Rust Edition)".red().on_blue());
    let args: Vec<String> = env::args().collect();
    let currency_ratio: f64 = args[1].parse()?;
    let cs_items = load_items().await?.items;
    for item in cs_items {
        let item_class = sm_api::SteamMarketItem::new(item.game_id, &item.name).await?;
        let current_price = item_class.get_current_price().unwrap();
        let profit = current_price - item.price;
        let indicator = if profit > 0.0 {
            "up".green()
        } else {
            "down".red()
        };
        let percentage = (profit / item.price) * 100.0;
        println!(
            "{}\n{} ${:.2}/{:.2}%\ncurrent: ${:.2} paid: ${:.2}\n",
            item.name.yellow(),
            indicator,
            profit * currency_ratio,
            percentage,
            current_price * currency_ratio,
            item.price * currency_ratio
        );
    }
    Ok(())
}
