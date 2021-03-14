use yahoo_finance_api as yahoo;
use futures::executor::block_on;
use std::env;
use textplots::{Chart, Plot, Shape};
use chrono::{Duration, Utc};

fn main() {
    let args: Vec<String> = env::args().collect();
    let ticker = &args[1];

    let last_month = Utc::now() - Duration::days(45);
    let provider = yahoo::YahooConnector::new();
    let response = block_on(provider.get_quote_history(ticker, last_month, Utc::now())).unwrap();

    let quote = response.last_quote().unwrap();
    let quotes = response.quotes().unwrap();
    let mut quotes_formatted : Vec<(f32, f32)> = Vec::new(); 

    for (pos, e) in quotes.iter().enumerate() {
        quotes_formatted.push((pos as f32, e.close as f32));
    }

    Chart::new(180, 60, 0.0, quotes.len() as f32).lineplot(&Shape::Lines(quotes_formatted.as_slice())).nice();

    println!("{} quote", ticker.to_uppercase());
    println!("----------");
    println!("Open: {:.2}, High: {:.2}, Low: {:.2}", quote.open, quote.high, quote.low);
    println!("Volume: {:.2}", quote.volume);
}
