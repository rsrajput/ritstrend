//! market_summary.rs
//!
//! Computes market-wide statistics from ScoreEngine output.
//! This module contains no presentation logic.

use crate::score_engine::{Rating, StockScore};

#[derive(Debug, Clone)]
pub struct MarketSummary {
    pub stocks_loaded: usize,
    pub missing_files: usize,

    pub buy_count: usize,
    pub watch_count: usize,
    pub monitor_count: usize,
    pub ignore_count: usize,

    pub highest_score: u8,
    pub strongest_symbol: String,
}

impl MarketSummary {
    pub fn from_scores(
        scores: &[StockScore],
        missing_files: usize,
    ) -> Self {
        let buy_count = scores.iter().filter(|s| matches!(s.rating, Rating::Buy)).count();
        let watch_count = scores.iter().filter(|s| matches!(s.rating, Rating::Watch)).count();
        let monitor_count = scores.iter().filter(|s| matches!(s.rating, Rating::Monitor)).count();
        let ignore_count = scores.iter().filter(|s| matches!(s.rating, Rating::Ignore)).count();

        let (highest_score, strongest_symbol) = scores
            .iter()
            .max_by_key(|s| s.score)
            .map(|s| (s.score, s.symbol.clone()))
            .unwrap_or((0, String::from("-")));

        Self {
            stocks_loaded: scores.len(),
            missing_files,
            buy_count,
            watch_count,
            monitor_count,
            ignore_count,
            highest_score,
            strongest_symbol,
        }
    }
}
