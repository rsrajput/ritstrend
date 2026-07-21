//! score_engine.rs
//!
//! Non-invasive scoring layer for RitsTrend.
//! Does not change BUY logic; it only scores existing analyses.

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
    pub reasons: Vec<String>,
}

pub struct ScoreEngine;

impl ScoreEngine {
    pub fn score(
        analysis: &StockAnalysis,
        rs_threshold: usize,
        volume_factor: f64,
    ) -> StockScore {
        let mut score = 0u8;
        let mut reasons = Vec::new();

        let close = analysis.latest_close.unwrap_or(0.0);
        let sma50 = analysis.sma50.unwrap_or(0.0);
        let sma200 = analysis.sma200.unwrap_or(0.0);
        let don55 = analysis.donchian_high55.unwrap_or(0.0);
        let adx = analysis.adx14.unwrap_or(0.0);
        let vol = analysis.latest_volume.unwrap_or(0.0);
        let avg50 = analysis.average_volume50.unwrap_or(0.0);
        let rs = analysis.relative_strength_rank.unwrap_or(usize::MAX);

        if close > don55 { score += 30; } else { reasons.push("No breakout".into()); }
        if close > sma200 { score += 20; } else { reasons.push("Below SMA200".into()); }
        if sma50 > sma200 { score += 15; } else { reasons.push("SMA50 ≤ SMA200".into()); }
        if adx >= 25.0 { score += 15; } else { reasons.push(format!("ADX {:.1}<25", adx)); }
        if avg50 > 0.0 && vol >= volume_factor * avg50 {
            score += 10;
        } else {
            reasons.push("Volume weak".into());
        }
        if rs <= rs_threshold {
            score += 10;
        } else {
            reasons.push("RS outside threshold".into());
        }

        let rating = match score {
            90..=100 => Rating::Buy,
            75..=89 => Rating::Watch,
            60..=74 => Rating::Monitor,
            _ => Rating::Ignore,
        };

        StockScore {
            symbol: analysis.symbol.clone(),
            score,
            rating,
            reasons,
        }
    }

    pub fn score_all(
        analyses: &[StockAnalysis],
        rs_threshold: usize,
        volume_factor: f64,
    ) -> Vec<StockScore> {
        analyses.iter()
            .map(|a| Self::score(a, rs_threshold, volume_factor))
            .collect()
    }
}
