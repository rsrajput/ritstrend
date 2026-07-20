//! screener.rs
//! Turtle-style trend following screener for RitsTrend v0.1

use crate::analysis::StockAnalysis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Buy,
    Candidate,
    Exit,
    Reject,
}

pub struct ScreenerConfig {
    pub min_adx: f64,
    pub min_volume_multiple: f64,
    pub top_percent_rank: usize,
}

impl Default for ScreenerConfig {
    fn default() -> Self {
        Self {
            min_adx: 25.0,
            min_volume_multiple: 1.5,
            top_percent_rank: 25,
        }
    }
}

pub struct TurtleScreener;

impl TurtleScreener {
    pub fn evaluate(
        a: &StockAnalysis,
        total_ranked: usize,
        cfg: &ScreenerConfig,
    ) -> Signal {
        // Exit rule has highest priority
        if let (Some(close), Some(dc20)) = (a.latest_close, a.donchian_low20) {
            if close < dc20 {
                return Signal::Exit;
            }
        }

        let mut passed = 0usize;

        if let (Some(close), Some(high55)) = (a.latest_close, a.donchian_high55) {
            if close > high55 {
                passed += 1;
            }
        }

        if let (Some(s50), Some(s200)) = (a.sma50, a.sma200) {
            if s50 > s200 {
                passed += 1;
            }
        }

        if let (Some(close), Some(s200)) = (a.latest_close, a.sma200) {
            if close > s200 {
                passed += 1;
            }
        }

        if let Some(adx) = a.adx14 {
            if adx >= cfg.min_adx {
                passed += 1;
            }
        }

        if let (Some(vol), Some(avg)) = (a.latest_volume, a.average_volume50) {
            if avg > 0.0 && vol >= avg * cfg.min_volume_multiple {
                passed += 1;
            }
        }

        if let Some(rank) = a.relative_strength_rank {
            let cutoff = ((total_ranked * cfg.top_percent_rank) + 99) / 100;
            if rank <= cutoff {
                passed += 1;
            }
        }

        match passed {
            6 => Signal::Buy,
            5 => Signal::Candidate,
            _ => Signal::Reject,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_expected() {
        let cfg = ScreenerConfig::default();
        assert_eq!(cfg.min_adx, 25.0);
        assert_eq!(cfg.min_volume_multiple, 1.5);
        assert_eq!(cfg.top_percent_rank, 25);
    }
}
