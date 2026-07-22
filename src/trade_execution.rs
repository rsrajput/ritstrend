//! execution.rs
//!
//! Trading execution helpers.
//!
//! These functions derive actionable trading information from an
//! existing `StockAnalysis` without duplicating data.

use crate::analysis::StockAnalysis;

/// Returns the breakout entry price.
///
/// Current implementation:
/// Entry = 55-day Donchian High
pub fn entry_price(analysis: &StockAnalysis) -> Option<f64> {
    analysis.donchian_high55
}

/// Returns the initial stop price.
///
/// Formula:
/// Stop = Entry − (ATR × Multiplier)
pub fn initial_stop(analysis: &StockAnalysis, atr_multiplier: f64) -> Option<f64> {
    let entry = analysis.donchian_high55?;
    let atr = analysis.atr15?;

    Some(entry - atr * atr_multiplier)
}

/// Returns the initial risk percentage.
///
/// Formula:
///
/// ((Entry - Stop) / Entry) × 100
pub fn risk_percent(analysis: &StockAnalysis, atr_multiplier: f64) -> Option<f64> {
    let entry = entry_price(analysis)?;
    let stop = initial_stop(analysis, atr_multiplier)?;

    if entry <= 0.0 {
        return None;
    }

    Some(((entry - stop) / entry) * 100.0)
}

/// Returns the reward/risk distance in points.
///
/// This is simply:
///
/// Entry − Stop
pub fn risk_points(analysis: &StockAnalysis, atr_multiplier: f64) -> Option<f64> {
    let entry = entry_price(analysis)?;
    let stop = initial_stop(analysis, atr_multiplier)?;

    Some(entry - stop)
}

/// Returns true if the stock has already broken out.
pub fn breakout_triggered(analysis: &StockAnalysis) -> bool {
    match (analysis.latest_close, analysis.donchian_high55) {
        (Some(close), Some(entry)) => close >= entry,
        _ => false,
    }
}
