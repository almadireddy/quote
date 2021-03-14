use std::env;
use chrono::{NaiveDate};
use alpha_vantage;
use alpha_vantage::util::*;
use std::io;
use termion::raw::IntoRawMode;
use tui::{Terminal, text::{Span}, symbols, style::{Modifier, Color, Style}, backend::TermionBackend};
use tui::widgets::{Axis, Chart, Dataset, GraphType, Table, Block, Borders, Row};
use tui::layout::{Layout, Constraint, Direction};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let ticker = &args[1];

    let api = alpha_vantage::set_with_env("ALPHA_VANTAGE_KEY");

    let stock = api.stock_time(
        StockFunction::Daily,
        ticker,
        TimeSeriesInterval::None,
        OutputSize::Compact
    ).await.unwrap();

    let quote = api.quote(ticker).await.unwrap();
    
    let stdout = io::stdout().into_raw_mode().unwrap();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();

    let open_price = &*quote.open().to_string();
    let high = &*quote.high().to_string();
    let low = &*quote.low().to_string();
    let volume = &*quote.volume().to_string();

    let mut stock_copy = stock.entry().to_vec();
    stock_copy.sort_by(|a, b| {
        let a = NaiveDate::parse_from_str(a.time(), "%Y-%m-%d").unwrap();
        let b = NaiveDate::parse_from_str(b.time(), "%Y-%m-%d").unwrap();

        return a.cmp(&b);
    });

    let mut transformed_history: Vec<(f64, f64)> = vec![];

    let mut upper_bound = stock_copy[0].close();
    let mut lower_bound = stock_copy[0].close();

    for (pos, e) in stock_copy.iter().enumerate() {
       transformed_history.push((pos as f64, e.close()));
       if e.close() > upper_bound {
            upper_bound = e.close();
       }

       if e.close() < lower_bound {
           lower_bound = e.close();
       }
    }

    let chart_color = if stock_copy.last().unwrap().close() < stock_copy[0].close() { 
        Style::default().fg(Color::Magenta) 
    } else { 
        Style::default().fg(Color::Green)
    };

    terminal.clear().unwrap();

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(30),
                    Constraint::Percentage(100),
                ].as_ref()
            )
            .split(f.size());

        let quote_table = Table::new(vec![
            Row::new(vec!["Open", open_price]),
            Row::new(vec!["High", high]),
            Row::new(vec!["Low", low]),
            Row::new(vec!["Volume", volume]),
        ])
            .block(
                Block::default()
                    .title(Span::styled(
                        "Quote",
                        Style::default()
                            .fg(Color::Cyan)
                    ))
                    .borders(Borders::ALL))
            .widths(&[Constraint::Percentage(50), Constraint::Percentage(50)]);

        f.render_widget(quote_table, chunks[0]);


        let datasets = vec![
            Dataset::default()
                .name("100 day history")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(chart_color)
                .data(transformed_history.as_slice())
        ];

       let chart = Chart::new(datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart",
                            Style::default()
                                .fg(Color::Cyan)
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0.0, stock_copy.len() as f64 - 1.0])
                )
                .y_axis(
                    Axis::default()
                        .title("$")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([lower_bound, upper_bound])
                        .labels(vec![
                            Span::styled(lower_bound.to_string(), Style::default().add_modifier(Modifier::BOLD)),
                            Span::styled(upper_bound.to_string(), Style::default().add_modifier(Modifier::BOLD)),
                        ]),
                ); 
        f.render_widget(chart, chunks[1]);
    }).unwrap();
    
}
