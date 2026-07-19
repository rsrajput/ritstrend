use chrono::NaiveDate;
use ritstrend::models::Candle;
use ritstrend::price_series::PriceSeries;

#[test]
fn price_series_api_is_exercised() {
    let candles = vec![
        Candle {
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            open: 10.0,
            high: 11.0,
            low: 9.5,
            close: 10.5,
            volume: 1000.0,
        },
        Candle {
            date: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
            open: 10.5,
            high: 12.0,
            low: 10.0,
            close: 11.0,
            volume: 1500.0,
        },
    ];

    let series = PriceSeries::new("TEST", candles).expect("series should be created");
    let _ = series.symbol();
    let _ = series.len();
    let _ = series.is_empty();
    let _ = series.latest();
    let _ = series.first();
    let _ = series.candles();
    let _ = series.date(0);
    let _ = series.close(0);
    let _ = series.high(0);
    let _ = series.low(0);
    let _ = series.open(0);
    let _ = series.volume(0);
    let _ = series.iter();
    let _ = series.dates();
    let _ = series.closes();
    let _ = series.opens();
    let _ = series.highs();
    let _ = series.lows();
    let _ = series.volumes();
    let _ = series.window(2);
    let _ = series.last(2);
    let _ = series.skip_last(1);
    let _ = series.highest_high(2);
    let _ = series.lowest_low(2);
    let _ = series.highest_close(2);
    let _ = series.lowest_close(2);
    let _ = series.average_volume(2);
    let _ = series.simple_return(1);
    let _ = series.percent_return(1);
    let _ = series.price_change(1);
    let _ = series.has_minimum_history(2);
    let _ = series.history_years();
    let _ = series.latest_close();
    let _ = series.latest_volume();
    let _ = series.latest_date();
}
