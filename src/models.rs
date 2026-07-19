use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

/// A single daily OHLCV candle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    /// Trading date for the candle.
    pub date: NaiveDate,
    /// Opening price.
    pub open: f64,
    /// Highest price of the day.
    pub high: f64,
    /// Lowest price of the day.
    pub low: f64,
    /// Closing price.
    pub close: f64,
    /// Traded volume.
    pub volume: f64,
}

/// The computed state used to evaluate a stock for screening.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct IndicatorSnapshot {
    /// Ticker symbol associated with the snapshot.
    pub symbol: String,
    /// Latest close price.
    pub close: f64,
    /// Fast moving average value.
    pub sma50: f64,
    /// Slow moving average value.
    pub sma200: f64,
    /// ATR value.
    pub atr15: f64,
    /// ADX value.
    pub adx14: f64,
    /// Volume ratio versus the moving average baseline.
    pub volume_ratio: f64,
    /// 55-day Donchian high.
    pub donchian55: f64,
    /// 20-day Donchian low.
    pub donchian20: f64,
    /// Six-month return.
    pub return6m: f64,
    /// Twelve-month return.
    pub return12m: f64,
    /// Relative Strength rank.
    pub rs_rank: usize,
}

impl Default for IndicatorSnapshot {
    fn default() -> Self {
        Self {
            symbol: String::from("UNKNOWN"),
            close: 0.0,
            sma50: 0.0,
            sma200: 0.0,
            atr15: 0.0,
            adx14: 0.0,
            volume_ratio: 0.0,
            donchian55: 0.0,
            donchian20: 0.0,
            return6m: 0.0,
            return12m: 0.0,
            rs_rank: 0,
        }
    }
}

/// The decision emitted by the screening stage.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    /// A strong buy candidate.
    Buy,
    /// A candidate that merits monitoring.
    Watch,
    /// An exit signal for an existing position.
    Exit,
    /// A stock that does not meet the screening criteria.
    Reject,
}
