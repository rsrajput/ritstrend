use anyhow::{Context, Result};

use crate::models::Candle;

/// A chronological collection of candles prepared for analysis.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PriceSeries {
    /// Symbol associated with the series.
    pub symbol: String,
    /// Candles in chronological order.
    pub candles: Vec<Candle>,
}

impl PriceSeries {
    /// Create a new price series after validating the candle order.
    pub fn new(symbol: impl Into<String>, candles: Vec<Candle>) -> Result<Self> {
        let symbol = symbol.into();

        if candles.is_empty() {
            anyhow::bail!("price series for '{}' cannot be empty", symbol);
        }

        let mut previous_date = None;
        for candle in &candles {
            if let Some(previous) = previous_date {
                anyhow::ensure!(
                    candle.date >= previous,
                    "candles for '{}' are not sorted chronologically",
                    symbol
                );
            }
            previous_date = Some(candle.date);
        }

        Ok(Self { symbol, candles })
    }

    /// Return the number of candles in the series.
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        self.candles.len()
    }

    /// Return true if the series has no candles.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.candles.is_empty()
    }

    /// Return the latest candle, when present.
    #[allow(dead_code)]
    pub fn latest(&self) -> Option<&Candle> {
        self.candles.last()
    }

    /// Return a slice to the requested range when it is in bounds.
    #[allow(dead_code)]
    pub fn slice(&self, start: usize, end: usize) -> Result<&[Candle]> {
        self.candles
            .get(start..end)
            .with_context(|| format!("requested range {}..{} is out of bounds", start, end))
    }
}
