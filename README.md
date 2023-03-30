# Steam Market API

This is currently a tool that lets you compare the price of an item you bought to the current price on the Steam market. It treats these items as if they are stock investments, and shows you output similar to one you'd find on a stock tracker.

## Quick Start

You can build the CLI tool with: `cargo build --bin cli`

You can run the CLI tool with `cargo run --bin cli <optional currency ratio argument>`

The argument given at the end is a float, such as 1.00. The currency of all the items is in USD. If you want the output to match your currency, for example CAD, which is 1:1.36 (USD:CAD), you can call the tool with `cargo run --bin cli 1.36`.

The items are added in `cs_items.toml`. The name needs to be exact to the item, the game id can be found in the Steam market URL, and the price needs to be set as USD.

## Example Output

![CLI output](./output.png)