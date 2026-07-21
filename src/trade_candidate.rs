//! trade_candidate.rs
//! Trading-ready view built from StockAnalysis + StockScore.

use crate::analysis::StockAnalysis;
use crate::score_engine::{Rating, StockScore};

/// Default ATR multiplier for initial stop.
pub const DEFAULT_STOP_MULTIPLIER: f64 = 2.0;

#[derive(Debug, Clone)]
pub struct TradeCandidate {
    pub symbol: String,
    pub close: f64,
    pub atr15: f64,
    pub initial_stop: f64,
    pub rs_rank: usize,
    pub score: u8,
    pub rating: Rating,
    pub reasons: Vec<String>,
}

impl TradeCandidate {
    pub fn from_analysis(
        analysis: &StockAnalysis,
        score: &StockScore,
    ) -> Self {
        Self::with_stop_multiplier(
            analysis,
            score,
            DEFAULT_STOP_MULTIPLIER,
        )
    }

    pub fn with_stop_multiplier(
        analysis: &StockAnalysis,
        score: &StockScore,
        stop_multiplier: f64,
    ) -> Self {
        let close = analysis.latest_close.unwrap_or(0.0);
        let atr = analysis.atr15.unwrap_or(0.0);

        Self {
            symbol: analysis.symbol.clone(),
            close,
            atr15: atr,
            initial_stop: (close - stop_multiplier * atr).max(0.0),
            rs_rank: analysis.relative_strength_rank.unwrap_or(0),
            score: score.score,
            rating: score.rating,
            reasons: score.reasons.clone(),
        }
    }

    pub fn stop_distance(&self) -> f64 {
        self.close - self.initial_stop
    }

    pub fn risk_percent(&self) -> f64 {
        if self.close <= 0.0 {
            0.0
        } else {
            self.stop_distance() / self.close * 100.0
        }
    }
}
