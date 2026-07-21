use anyhow::{Context, Result};
use serde::Deserialize;

/// Runtime configuration for the scanner pipeline.
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Lookback period used for breakout detection.
    pub breakout_period: usize,
    /// Lookback period used for exit rules.
    pub exit_period: usize,
    /// Fast moving average period.
    pub sma_fast: usize,
    /// Slow moving average period.
    pub sma_slow: usize,
    /// ATR calculation period.
    pub atr_period: usize,
    /// ADX calculation period.
    pub adx_period: usize,
    /// Lookback period used for average-volume calculations.
    pub volume_period: usize,
    /// Minimum volume multiplier for the breakout condition.
    pub volume_factor: f64,
    /// Number of months used for Relative Strength ranking.
    pub relative_strength_months: usize,
    /// Top percentage threshold for ranking-based filtering.
    pub top_percent: usize,
    /// Near breakout maximum distance (%)
    pub near_breakout_distance: f64,
    /// Minimum ADX for near breakout
    pub near_breakout_min_adx: f64,
    /// Maximum RS rank for near breakout
    pub near_breakout_max_rs_rank: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            breakout_period: 55,
            exit_period: 20,
            sma_fast: 50,
            sma_slow: 200,
            atr_period: 15,
            adx_period: 14,
            volume_period: 50,
            volume_factor: 1.5,
            relative_strength_months: 6,
            top_percent: 25,
            near_breakout_distance: 2.0,
            near_breakout_min_adx: 20.0,
            near_breakout_max_rs_rank: 125,
        }
    }
}

impl Config {
    /// Load configuration from a TOML file.
    pub fn load(path: &str) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("unable to read configuration file {}", path))?;

        toml::from_str(&text)
            .with_context(|| format!("unable to parse configuration file {}", path))
    }
}
