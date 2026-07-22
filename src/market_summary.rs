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
    pub market_verdict: String,
    pub trend_confidence: u8,
}

impl MarketSummary {
    pub fn from_scores(scores: &[StockScore], missing_files: usize) -> Self {
        let buy_count = scores
            .iter()
            .filter(|s| matches!(s.rating, Rating::Buy))
            .count();
        let watch_count = scores
            .iter()
            .filter(|s| matches!(s.rating, Rating::Watch))
            .count();
        let monitor_count = scores
            .iter()
            .filter(|s| matches!(s.rating, Rating::Monitor))
            .count();
        let ignore_count = scores
            .iter()
            .filter(|s| matches!(s.rating, Rating::Ignore))
            .count();

        let (highest_score, strongest_symbol) = scores
            .iter()
            .max_by_key(|s| s.score)
            .map(|s| (s.score, s.symbol.clone()))
            .unwrap_or((0, String::from("-")));

        let trend_confidence = (((buy_count * 100) + (watch_count * 70) + (monitor_count * 30))
            / scores.len().max(1)) as u8;

        let market_verdict = if trend_confidence >= 70 {
            "FAVORABLE"
        } else if trend_confidence >= 50 {
            "NEUTRAL"
        } else {
            "DEFENSIVE"
        }
        .to_string();

        Self {
            stocks_loaded: scores.len(),
            missing_files,
            buy_count,
            watch_count,
            monitor_count,
            ignore_count,
            highest_score,
            strongest_symbol,
            market_verdict,
            trend_confidence,
        }
    }
}
