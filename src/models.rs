use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub date: NaiveDate,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Clone)]
pub struct IndicatorSnapshot {
    pub symbol: String,

    pub close: f64,

    pub sma50: f64,
    pub sma200: f64,

    pub atr15: f64,
    pub adx14: f64,

    pub volume_ratio: f64,

    pub donchian55: f64,
    pub donchian20: f64,

    pub return6m: f64,
    pub return12m: f64,

    pub rs_rank: usize,
}

#[derive(Debug)]
pub enum Signal {
    Buy,
    Watch,
    Exit,
    Reject,
}