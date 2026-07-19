use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub breakout_period: usize,
    pub exit_period: usize,

    pub sma_fast: usize,
    pub sma_slow: usize,

    pub atr_period: usize,
    pub adx_period: usize,

    pub volume_period: usize,
    pub volume_factor: f64,

    pub relative_strength_months: usize,
    pub top_percent: usize,
}

impl Config {
    pub fn load(path: &str) -> anyhow::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Ok(toml::from_str(&text)?)
    }
}