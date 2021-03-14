use yahoo_finance_api as yahoo;
use futures::executor::block_on;
use std::env;
use textplots::{Chart, Plot, Shape};
use chrono::{Duration, Utc};

async fn run() {
    let args: Vec<String> = env::args().collect();
    let ticker = &args[1];

    let last_month = Utc::now() - Duration::days(45);
    let provider = yahoo::YahooConnector::new();
    let response = provider.get_quote_history(ticker, last_month, Utc::now()).await.unwrap();
    let meta_response = provider.search_ticker(ticker).await.unwrap();
    let meta = &meta_response.quotes[0];

    let quote = response.last_quote().unwrap();
    let quotes = response.quotes().unwrap();
    let mut quotes_formatted : Vec<(f32, f32)> = Vec::new(); 

    for (pos, e) in quotes.iter().enumerate() {
        quotes_formatted.push((pos as f32, e.close as f32));
    }

    Chart::new(100, 60, 0.0, quotes.len() as f32).lineplot(&Shape::Lines(quotes_formatted.as_slice())).nice();


    println!("${} | {}", ticker.to_uppercase(), meta.long_name);
    println!("----------");
    println!("Open: {:.2}, High: {:.2}, Low: {:.2}", quote.open, quote.high, quote.low);
    println!("Volume: {:.2}", quote.volume);
    println!("__________");
    
    for i in 0..5 {
        if i > meta_response.news.len() {
            break;
        }
        println!("{}", meta_response.news[i].title);
    }
}

fn main() {
    block_on(run());
}
