//! score_engine.rs
//!
//! Scores every stock based on the screening rules instead of returning
//! a simple BUY / NO BUY decision.

use crate::analysis::StockAnalysis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rating {
    Buy,
    Watch,
    Monitor,
    Ignore,
}

#[derive(Debug, Clone)]
pub struct StockScore {
    pub symbol: String,
    pub score: u8,
    pub rating: Rating,

    pub breakout: bool,
    pub trend: bool,
    pub moving_average: bool,
    pub adx: bool,
    pub volume: bool,
    pub relative_strength: bool,

    pub reasons: Vec<String>,
}

pub struct ScoreEngine;

impl ScoreEngine {
    pub fn score(
        analysis: &StockAnalysis,
        rs_threshold: usize,
        volume_factor: f64,
        adx_min: f64,
    ) -> StockScore {

        let close = analysis.latest_close.unwrap_or_default();
        let sma50 = analysis.sma50.unwrap_or_default();
        let sma200 = analysis.sma200.unwrap_or_default();
        let don = analysis.donchian_high55.unwrap_or_default();
        let adx = analysis.adx14.unwrap_or_default();
        let vol = analysis.latest_volume.unwrap_or_default();
        let avg = analysis.average_volume50.unwrap_or_default();
        let rs = analysis.relative_strength_rank.unwrap_or(usize::MAX);

        let breakout = close > don;
        let trend = close > sma200;
        let moving_average = sma50 > sma200;
        let adx_ok = adx >= adx_min;
        let volume_ok = vol >= volume_factor * avg;
        let rs_ok = rs <= rs_threshold;

        let mut score: u8 = 0;
        let mut reasons = Vec::new();

        if breakout {
            score += 30;
        } else {
            reasons.push("No Donchian breakout".into());
        }

        if trend {
            score += 20;
        } else {
            reasons.push("Below SMA200".into());
        }

        if moving_average {
            score += 15;
        } else {
            reasons.push("SMA50 below SMA200".into());
        }

        if adx_ok {
            score += 15;
        } else {
            reasons.push(format!("ADX {:.1} below {:.1}", adx, adx_min));
        }

        if rs_ok {
            score += 15;
        } else {
            reasons.push("Relative Strength rank too low".into());
        }

        if volume_ok {
            score += 5;
        } else {
            reasons.push("Volume confirmation missing".into());
        }

        let rating = match score {
            95..=100 => Rating::Buy,
            80..=94 => Rating::Watch,
            65..=79 => Rating::Monitor,
            _ => Rating::Ignore,
        };

        StockScore {
            symbol: analysis.symbol.clone(),
            score,
            rating,
            breakout,
            trend,
            moving_average,
            adx: adx_ok,
            volume: volume_ok,
            relative_strength: rs_ok,
            reasons,
        }
    }
}
