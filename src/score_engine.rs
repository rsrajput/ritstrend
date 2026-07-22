//! score_engine_v2.rs
//! Enhanced scoring engine for RitsTrend.
//! Backward-compatible with the current architecture.

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
    pub close: f64,
    pub rs_rank: usize,
    pub entry_price: Option<f64>,
    pub stop_price: Option<f64>,
    pub risk_percent: Option<f64>,
}

pub struct ScoreEngine;

impl ScoreEngine {
    pub fn score(
        analysis: &StockAnalysis,
        rs_threshold: usize,
        volume_factor: f64,
        atr_multiplier: f64,
    ) -> StockScore {
        let mut score = 0u8;
        let mut reasons = Vec::new();

        let close = analysis.latest_close.unwrap_or(0.0);
        let sma50 = analysis.sma50.unwrap_or(0.0);
        let sma200 = analysis.sma200.unwrap_or(0.0);
        let don55 = analysis.donchian_high55.unwrap_or(0.0);
        let adx = analysis.adx14.unwrap_or(0.0);
        let vol = analysis.latest_volume.unwrap_or(0.0);
        let avg = analysis.average_volume50.unwrap_or(0.0);
        let rs = analysis.relative_strength_rank.unwrap_or(usize::MAX);

        if close > don55 {
            score += 25;
        } else {
            reasons.push("Waiting for breakout".into());
        }
        if close > sma200 {
            score += 20;
        } else {
            reasons.push("Below SMA200".into());
        }
        if sma50 > sma200 {
            score += 15;
        } else {
            reasons.push("Weak moving-average trend".into());
        }

        if adx >= 40.0 {
            score += 18;
        } else if adx >= 25.0 {
            score += 15;
        } else {
            reasons.push(format!("ADX {:.1} below 25", adx));
        }

        if avg > 0.0 && vol >= volume_factor * avg {
            score += 10;
        } else {
            reasons.push("Volume confirmation missing".into());
        }

        if rs <= rs_threshold / 2 {
            score += 12;
        } else if rs <= rs_threshold {
            score += 10;
        } else {
            reasons.push("Relative strength outside threshold".into());
        }

        let rating = match score {
            90..=u8::MAX => Rating::Buy,
            75..=89 => Rating::Watch,
            60..=74 => Rating::Monitor,
            _ => Rating::Ignore,
        };

        if reasons.is_empty() {
            reasons.push("All conditions satisfied".into());
        }

        StockScore {
            symbol: analysis.symbol.clone(),
            score,
            rating,
            reasons,
            close,
            rs_rank: rs,
            entry_price: analysis.donchian_high55,
            stop_price: match (analysis.donchian_high55, analysis.atr15) {
                (Some(entry), Some(atr)) => Some(entry - atr * atr_multiplier),
                _ => None,
            },
            risk_percent: match (analysis.donchian_high55, analysis.atr15) {
                (Some(entry), Some(atr)) if entry > 0.0 => {
                    Some((atr * atr_multiplier / entry) * 100.0)
                }
                _ => None,
            },
        }
    }

    pub fn score_all(
        analyses: &[StockAnalysis],
        rs_threshold: usize,
        volume_factor: f64,
        atr_multiplier: f64,
    ) -> Vec<StockScore> {
        analyses
            .iter()
            .map(|a| Self::score(a, rs_threshold, volume_factor, atr_multiplier))
            .collect()
    }
}
