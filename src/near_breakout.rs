//! near_breakout.rs
//! Finds stocks that are closest to a 55-day breakout.

use crate::analysis::StockAnalysis;

#[derive(Debug, Clone)]
pub struct NearBreakout {
    pub symbol: String,
    pub close: f64,
    pub breakout_price: f64,
    pub distance_percent: f64,
    pub adx: f64,
    pub rs_rank: usize,
}

pub struct NearBreakoutEngine;

impl NearBreakoutEngine {
    /// Return stocks within `max_distance_percent` of a breakout,
    /// sorted by nearest breakout.
    pub fn find(
        analyses: &[StockAnalysis],
        max_distance_percent: f64,
        min_adx: f64,
        max_rs_rank: usize,
    ) -> Vec<NearBreakout> {
        let mut candidates: Vec<NearBreakout> = analyses
            .iter()
            .filter_map(|a| {
                let close = a.latest_close?;
                let breakout = a.donchian_high55?;
                let sma50 = a.sma50?;
                let sma200 = a.sma200?;
                let adx = a.adx14?;
                let rs_rank = a.relative_strength_rank.unwrap_or(usize::MAX);

                // Quality filters
                if close <= sma200 || sma50 <= sma200 {
                    return None;
                }

                if adx < min_adx {
                    return None;
                }

                if rs_rank > max_rs_rank {
                    return None;
                }

                // Already broken out
                if close >= breakout {
                    return None;
                }

                let distance = ((breakout - close) / close) * 100.0;

                if distance <= max_distance_percent {
                    Some(NearBreakout {
                        symbol: a.symbol.clone(),
                        close,
                        breakout_price: breakout,
                        distance_percent: distance,
                        adx,
                        rs_rank,
                    })
                } else {
                    None
                }
            })
            .collect();

        candidates.sort_by(|a, b| a.distance_percent.partial_cmp(&b.distance_percent).unwrap());

        candidates
    }
}
