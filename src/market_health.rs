//! market_health.rs
//!
//! Computes overall market health metrics from StockAnalysis.
//! Presentation is left to console_report.rs / html.rs.

use crate::analysis::StockAnalysis;

#[derive(Debug, Clone)]
pub struct MarketHealth {
    pub total_stocks: usize,
    pub missing_files: usize,

    pub above_sma200: usize,
    pub above_sma50: usize,
    pub adx_above_25: usize,
    pub breakout_55: usize,

    pub pct_above_sma200: f64,
    pub pct_above_sma50: f64,
    pub pct_adx_above_25: f64,
    pub pct_breakout_55: f64,
}

impl MarketHealth {
    pub fn analyze(analyses: &[StockAnalysis], missing_files: usize) -> Self {
        let total = analyses.len().max(1);

        let above_sma200 = analyses.iter().filter(|a|
            a.latest_close.unwrap_or(0.0) > a.sma200.unwrap_or(f64::MAX)
        ).count();

        let above_sma50 = analyses.iter().filter(|a|
            a.latest_close.unwrap_or(0.0) > a.sma50.unwrap_or(f64::MAX)
        ).count();

        let adx_above_25 = analyses.iter().filter(|a|
            a.adx14.unwrap_or(0.0) >= 25.0
        ).count();

        let breakout_55 = analyses.iter().filter(|a|
            a.latest_close.unwrap_or(0.0) >
            a.donchian_high55.unwrap_or(f64::MAX)
        ).count();

        Self {
            total_stocks: analyses.len(),
            missing_files,
            above_sma200,
            above_sma50,
            adx_above_25,
            breakout_55,
            pct_above_sma200: above_sma200 as f64 * 100.0 / total as f64,
            pct_above_sma50: above_sma50 as f64 * 100.0 / total as f64,
            pct_adx_above_25: adx_above_25 as f64 * 100.0 / total as f64,
            pct_breakout_55: breakout_55 as f64 * 100.0 / total as f64,
        }
    }

    pub fn market_rating(&self) -> &'static str {
        if self.pct_above_sma200 >= 60.0 && self.pct_adx_above_25 >= 30.0 {
            "★★★★★ Strong Uptrend"
        } else if self.pct_above_sma200 >= 50.0 {
            "★★★★☆ Healthy Uptrend"
        } else if self.pct_above_sma200 >= 40.0 {
            "★★★☆☆ Mixed Market"
        } else if self.pct_above_sma200 >= 30.0 {
            "★★☆☆☆ Weak Market"
        } else {
            "★☆☆☆☆ Defensive"
        }
    }
}
