use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::Signal;

/// Immutable analysis data for a single stock.
///
/// This model captures the latest market snapshot, optional indicator values,
/// and the screening metadata for a stock without performing any calculation.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct StockAnalysis {
    /// Ticker symbol for the analyzed stock.
    pub symbol: String,
    /// Latest close price, when available.
    pub latest_close: Option<f64>,
    /// Latest traded volume, when available.
    pub latest_volume: Option<f64>,
    /// Latest available date, when available.
    pub latest_date: Option<NaiveDate>,
    /// 50-period simple moving average, when available.
    pub sma50: Option<f64>,
    /// 200-period simple moving average, when available.
    pub sma200: Option<f64>,
    /// 15-period ATR value, when available.
    pub atr15: Option<f64>,
    /// 14-period ADX value, when available.
    pub adx14: Option<f64>,
    /// 50-period average volume, when available.
    pub average_volume50: Option<f64>,
    /// 55-period Donchian high, when available.
    pub donchian_high55: Option<f64>,
    /// 20-period Donchian low, when available.
    pub donchian_low20: Option<f64>,
    /// Six-month return, when available.
    pub return6m: Option<f64>,
    /// Twelve-month return, when available.
    pub return12m: Option<f64>,
    /// Relative strength score, when available.
    pub relative_strength_score: Option<f64>,
    /// Relative strength rank, when available.
    pub relative_strength_rank: Option<usize>,
    /// Signal assigned by the screener, when available.
    pub signal: Option<Signal>,
    /// Composite score, when available.
    pub score: Option<f64>,
    /// Human-readable reasons behind the analysis.
    pub reasons: Vec<String>,
}

impl std::fmt::Display for StockAnalysis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StockAnalysis(symbol={}, latest_close={:?}, signal={:?}, score={:?})",
            self.symbol, self.latest_close, self.signal, self.score
        )
    }
}

/// Builder for constructing an immutable StockAnalysis value.
#[derive(Debug, Clone, Default)]
pub struct StockAnalysisBuilder {
    symbol: Option<String>,
    latest_close: Option<f64>,
    latest_volume: Option<f64>,
    latest_date: Option<NaiveDate>,
    sma50: Option<f64>,
    sma200: Option<f64>,
    atr15: Option<f64>,
    adx14: Option<f64>,
    average_volume50: Option<f64>,
    donchian_high55: Option<f64>,
    donchian_low20: Option<f64>,
    return6m: Option<f64>,
    return12m: Option<f64>,
    relative_strength_score: Option<f64>,
    relative_strength_rank: Option<usize>,
    signal: Option<Signal>,
    score: Option<f64>,
    reasons: Option<Vec<String>>,
}

#[allow(dead_code)]
impl StockAnalysisBuilder {
    /// Create a new builder with all fields unset.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the ticker symbol.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set the latest close price.
    pub fn latest_close(mut self, latest_close: f64) -> Self {
        self.latest_close = Some(latest_close);
        self
    }

    /// Set the latest volume.
    pub fn latest_volume(mut self, latest_volume: f64) -> Self {
        self.latest_volume = Some(latest_volume);
        self
    }

    /// Set the latest date.
    pub fn latest_date(mut self, latest_date: NaiveDate) -> Self {
        self.latest_date = Some(latest_date);
        self
    }

    /// Set the 50-period moving average.
    pub fn sma50(mut self, sma50: f64) -> Self {
        self.sma50 = Some(sma50);
        self
    }

    /// Set the 200-period moving average.
    pub fn sma200(mut self, sma200: f64) -> Self {
        self.sma200 = Some(sma200);
        self
    }

    /// Set the 15-period ATR value.
    pub fn atr15(mut self, atr15: f64) -> Self {
        self.atr15 = Some(atr15);
        self
    }

    /// Set the 14-period ADX value.
    pub fn adx14(mut self, adx14: f64) -> Self {
        self.adx14 = Some(adx14);
        self
    }

    /// Set the 50-period average volume value.
    pub fn average_volume50(mut self, average_volume50: f64) -> Self {
        self.average_volume50 = Some(average_volume50);
        self
    }

    /// Set the 55-period Donchian high value.
    pub fn donchian_high55(mut self, donchian_high55: f64) -> Self {
        self.donchian_high55 = Some(donchian_high55);
        self
    }

    /// Set the 20-period Donchian low value.
    pub fn donchian_low20(mut self, donchian_low20: f64) -> Self {
        self.donchian_low20 = Some(donchian_low20);
        self
    }

    /// Set the six-month return value.
    pub fn return6m(mut self, return6m: f64) -> Self {
        self.return6m = Some(return6m);
        self
    }

    /// Set the twelve-month return value.
    pub fn return12m(mut self, return12m: f64) -> Self {
        self.return12m = Some(return12m);
        self
    }

    /// Set the relative strength score.
    pub fn relative_strength_score(mut self, relative_strength_score: f64) -> Self {
        self.relative_strength_score = Some(relative_strength_score);
        self
    }

    /// Set the relative strength rank.
    pub fn relative_strength_rank(mut self, relative_strength_rank: usize) -> Self {
        self.relative_strength_rank = Some(relative_strength_rank);
        self
    }

    /// Set the screener signal.
    pub fn signal(mut self, signal: Signal) -> Self {
        self.signal = Some(signal);
        self
    }

    /// Set the composite score.
    pub fn score(mut self, score: f64) -> Self {
        self.score = Some(score);
        self
    }

    /// Set one or more reasons.
    pub fn reasons(mut self, reasons: Vec<String>) -> Self {
        self.reasons = Some(reasons);
        self
    }

    /// Set the ticker symbol through a mutable builder reference.
    pub fn set_symbol(&mut self, symbol: impl Into<String>) {
        self.symbol = Some(symbol.into());
    }

    /// Set the latest close price through a mutable builder reference.
    pub fn set_latest_close(&mut self, latest_close: f64) {
        self.latest_close = Some(latest_close);
    }

    /// Set the latest volume through a mutable builder reference.
    pub fn set_latest_volume(&mut self, latest_volume: f64) {
        self.latest_volume = Some(latest_volume);
    }

    /// Set the latest date through a mutable builder reference.
    pub fn set_latest_date(&mut self, latest_date: NaiveDate) {
        self.latest_date = Some(latest_date);
    }

    /// Set the 50-period moving average through a mutable builder reference.
    pub fn set_sma50(&mut self, sma50: f64) {
        self.sma50 = Some(sma50);
    }

    /// Set the 200-period moving average through a mutable builder reference.
    pub fn set_sma200(&mut self, sma200: f64) {
        self.sma200 = Some(sma200);
    }

    /// Set the 15-period ATR value through a mutable builder reference.
    pub fn set_atr15(&mut self, atr15: f64) {
        self.atr15 = Some(atr15);
    }

    /// Set the 50-period average volume through a mutable builder reference.
    pub fn set_average_volume50(&mut self, average_volume50: f64) {
        self.average_volume50 = Some(average_volume50);
    }

    /// Set the 14-period ADX value through a mutable builder reference.
    pub fn set_adx14(&mut self, adx14: f64) {
        self.adx14 = Some(adx14);
    }

    /// Set the 55-period Donchian high through a mutable builder reference.
    pub fn set_donchian_high55(&mut self, donchian_high55: f64) {
        self.donchian_high55 = Some(donchian_high55);
    }

    /// Set the 20-period Donchian low through a mutable builder reference.
    pub fn set_donchian_low20(&mut self, donchian_low20: f64) {
        self.donchian_low20 = Some(donchian_low20);
    }

    /// Set the six-month return through a mutable builder reference.
    pub fn set_return6m(&mut self, return6m: f64) {
        self.return6m = Some(return6m);
    }

    /// Set the twelve-month return through a mutable builder reference.
    pub fn set_return12m(&mut self, return12m: f64) {
        self.return12m = Some(return12m);
    }

    /// Build the immutable StockAnalysis value.
    pub fn build(self) -> StockAnalysis {
        StockAnalysis {
            symbol: self.symbol.unwrap_or_default(),
            latest_close: self.latest_close,
            latest_volume: self.latest_volume,
            latest_date: self.latest_date,
            sma50: self.sma50,
            sma200: self.sma200,
            atr15: self.atr15,
            adx14: self.adx14,
            average_volume50: self.average_volume50,
            donchian_high55: self.donchian_high55,
            donchian_low20: self.donchian_low20,
            return6m: self.return6m,
            return12m: self.return12m,
            relative_strength_score: self.relative_strength_score,
            relative_strength_rank: self.relative_strength_rank,
            signal: self.signal,
            score: self.score,
            reasons: self.reasons.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_can_create_a_default_analysis() {
        let analysis = StockAnalysisBuilder::new().build();

        assert!(analysis.symbol.is_empty());
        assert!(analysis.latest_close.is_none());
        assert!(analysis.sma50.is_none());
        assert!(analysis.signal.is_none());
        assert!(analysis.reasons.is_empty());
    }

    #[test]
    fn builder_can_populate_optional_fields() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let analysis = StockAnalysisBuilder::new()
            .symbol("TEST")
            .latest_close(100.0)
            .latest_volume(250_000.0)
            .latest_date(date)
            .sma50(98.5)
            .sma200(95.0)
            .atr15(2.1)
            .adx14(24.0)
            .average_volume50(200_000.0)
            .donchian_high55(110.0)
            .donchian_low20(90.0)
            .return6m(12.5)
            .return12m(25.0)
            .relative_strength_score(1.75)
            .relative_strength_rank(7)
            .signal(Signal::Buy)
            .score(88.0)
            .reasons(vec![
                String::from("Momentum improving"),
                String::from("Volume rising"),
            ])
            .build();

        assert_eq!(analysis.symbol, "TEST");
        assert_eq!(analysis.latest_close, Some(100.0));
        assert_eq!(analysis.latest_volume, Some(250_000.0));
        assert_eq!(analysis.latest_date, Some(date));
        assert_eq!(analysis.sma50, Some(98.5));
        assert_eq!(analysis.sma200, Some(95.0));
        assert_eq!(analysis.atr15, Some(2.1));
        assert_eq!(analysis.adx14, Some(24.0));
        assert_eq!(analysis.average_volume50, Some(200_000.0));
        assert_eq!(analysis.donchian_high55, Some(110.0));
        assert_eq!(analysis.donchian_low20, Some(90.0));
        assert_eq!(analysis.return6m, Some(12.5));
        assert_eq!(analysis.return12m, Some(25.0));
        assert_eq!(analysis.relative_strength_score, Some(1.75));
        assert_eq!(analysis.relative_strength_rank, Some(7));
        assert_eq!(analysis.signal, Some(Signal::Buy));
        assert_eq!(analysis.score, Some(88.0));
        assert_eq!(analysis.reasons.len(), 2);
    }

    #[test]
    fn display_and_serialization_are_supported() {
        let analysis = StockAnalysisBuilder::new()
            .symbol("XYZ")
            .latest_close(42.5)
            .signal(Signal::Watch)
            .score(70.0)
            .build();

        let rendered = analysis.to_string();
        assert!(rendered.contains("XYZ"));
        assert!(rendered.contains("42.5"));

        let serialized = toml::to_string(&analysis).expect("toml serialization should succeed");
        let round_trip: StockAnalysis = match toml::from_str(&serialized) {
            Ok(value) => value,
            Err(err) => panic!("toml deserialization failed: {err}"),
        };

        assert_eq!(round_trip.symbol, analysis.symbol);
        assert_eq!(round_trip.latest_close, analysis.latest_close);
        assert_eq!(round_trip.signal, analysis.signal);
        assert_eq!(round_trip.score, analysis.score);
    }
}
