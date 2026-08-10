use anyhow::Result;

use crate::adx::AdxCalculator;
use crate::atr::AtrCalculator;
use crate::analysis::StockAnalysisBuilder;
use crate::models::Candle;
use crate::price_series::PriceSeries;

/// Computes the simple trend and momentum indicators for a single price series.
///
/// This engine is intentionally limited to lightweight, non-overlapping
/// calculations that can be applied to a price series and written into a
/// stock analysis builder without introducing trading logic or ranking.
#[derive(Debug, Default, Clone, Copy)]
pub struct IndicatorEngine;

impl IndicatorEngine {
    /// Analyze a price series and populate the builder with the supported
    /// simple indicators.
    pub fn analyze(series: &PriceSeries, builder: &mut StockAnalysisBuilder) -> Result<()> {
        builder.set_symbol(series.symbol());

        if let Some(latest) = series.latest() {
            builder.set_latest_close(latest.close);
            builder.set_latest_volume(latest.volume);
            builder.set_latest_date(latest.date);
        }

        if let Some(sma50) = Self::sma(series, 50) {
            builder.set_sma50(sma50);
        }

        if let Some(sma200) = Self::sma(series, 200) {
            builder.set_sma200(sma200);
        }

        if let Some(donchian_high55) = Self::donchian_high(series, 55) {
            builder.set_donchian_high55(donchian_high55);
        }

        if let Some(donchian_low20) = Self::donchian_low(series, 20) {
            builder.set_donchian_low20(donchian_low20);
        }

        if let Some(average_volume50) = Self::average_volume(series, 50) {
            builder.set_average_volume50(average_volume50);
        }

        if let Some(adx14) = AdxCalculator::adx(series, 14) {
            builder.set_adx14(adx14);
        }

        if let Some(atr15) = AtrCalculator::atr(series, 15) {
            builder.set_atr15(atr15);
        }

        if let Some(return6m) = Self::return_period(series, 126) {
            builder.set_return6m(return6m);
        }

        if let Some(return12m) = Self::return_period(series, 252) {
            builder.set_return12m(return12m);
        }

        Ok(())
    }

    /// Compute a simple moving average over the latest `period` closing prices.
    pub fn sma(series: &PriceSeries, period: usize) -> Option<f64> {
        if period == 0 || series.len() < period {
            return None;
        }

        let (count, total) = Self::window_values(series, period, false, |candle| candle.close)
            .fold((0usize, 0.0), |(count, total), value| {
                (count + 1, total + value)
            });

        if count == 0 {
            None
        } else {
            Some(total / count as f64)
        }
    }

    /// Compute the previous `period`-day Donchian high, excluding the latest candle.
    pub fn donchian_high(series: &PriceSeries, period: usize) -> Option<f64> {
        if period == 0 || series.len() <= period {
            return None;
        }

        Self::window_values(series, period, true, |candle| candle.high).reduce(f64::max)
    }

    /// Compute the previous `period`-day Donchian low, excluding the latest candle.
    pub fn donchian_low(series: &PriceSeries, period: usize) -> Option<f64> {
        if period == 0 || series.len() <= period {
            return None;
        }

        Self::window_values(series, period, true, |candle| candle.low).reduce(f64::min)
    }

    /// Compute the average volume over the previous `period` trading days.
    pub fn average_volume(series: &PriceSeries, period: usize) -> Option<f64> {
        if period == 0 || series.len() <= period {
            return None;
        }

        let (count, total) = Self::window_values(series, period, true, |candle| candle.volume)
            .fold((0usize, 0.0), |(count, total), value| {
                (count + 1, total + value)
            });

        if count == 0 {
            None
        } else {
            Some(total / count as f64)
        }
    }

    /// Compute a simple return over the previous `period` candles.
    pub fn return_period(series: &PriceSeries, period: usize) -> Option<f64> {
        if period == 0 || series.len() <= period {
            return None;
        }

        let current_index = series.len().checked_sub(1)?;
        let prior_index = current_index.checked_sub(period)?;
        let current_close = series.close(current_index)?;
        let prior_close = series.close(prior_index)?;

        if prior_close == 0.0 {
            return None;
        }

        Some(current_close / prior_close - 1.0)
    }

    fn window_values<'a, F>(
        series: &'a PriceSeries,
        period: usize,
        exclude_current: bool,
        selector: F,
    ) -> impl Iterator<Item = f64> + 'a
    where
        F: Fn(&'a Candle) -> f64 + 'a,
    {
        let skip = usize::from(exclude_current);
        series
            .candles()
            .iter()
            .rev()
            .skip(skip)
            .take(period)
            .map(selector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn parse_date(date: &str) -> Option<NaiveDate> {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()
    }

    fn candle(date: &str, close: f64, high: f64, low: f64, volume: f64) -> Option<Candle> {
        let date = parse_date(date)?;
        Some(Candle {
            date,
            open: close - 1.0,
            high,
            low,
            close,
            volume,
        })
    }

    fn price_series_from_rows(rows: &[(&str, f64, f64, f64, f64)]) -> Option<PriceSeries> {
        let candles = rows
            .iter()
            .map(|(date, close, high, low, volume)| candle(date, *close, *high, *low, *volume))
            .collect::<Option<Vec<_>>>()?;

        PriceSeries::new("TEST", candles).ok()
    }

    #[test]
    fn sma_uses_the_latest_period_of_closes() {
        let series = match price_series_from_rows(&[
            ("2024-01-01", 10.0, 11.0, 9.0, 100.0),
            ("2024-01-02", 12.0, 13.0, 10.0, 100.0),
            ("2024-01-03", 14.0, 15.0, 13.0, 100.0),
            ("2024-01-04", 16.0, 17.0, 15.0, 100.0),
        ]) {
            Some(series) => series,
            None => return,
        };

        assert_eq!(IndicatorEngine::sma(&series, 2), Some(15.0));
    }

    #[test]
    fn donchian_high_skips_the_latest_candle() {
        let series = match price_series_from_rows(&[
            ("2024-01-01", 10.0, 11.0, 9.0, 100.0),
            ("2024-01-02", 12.0, 13.0, 10.0, 100.0),
            ("2024-01-03", 14.0, 15.0, 13.0, 100.0),
            ("2024-01-04", 16.0, 17.0, 15.0, 100.0),
        ]) {
            Some(series) => series,
            None => return,
        };

        assert_eq!(IndicatorEngine::donchian_high(&series, 2), Some(15.0));
    }

    #[test]
    fn donchian_low_skips_the_latest_candle() {
        let series = match price_series_from_rows(&[
            ("2024-01-01", 10.0, 11.0, 9.0, 100.0),
            ("2024-01-02", 12.0, 13.0, 10.0, 100.0),
            ("2024-01-03", 14.0, 15.0, 13.0, 100.0),
            ("2024-01-04", 16.0, 17.0, 15.0, 100.0),
        ]) {
            Some(series) => series,
            None => return,
        };

        assert_eq!(IndicatorEngine::donchian_low(&series, 2), Some(10.0));
    }

    #[test]
    fn average_volume_uses_the_previous_period_only() {
        let series = match price_series_from_rows(&[
            ("2024-01-01", 10.0, 11.0, 9.0, 100.0),
            ("2024-01-02", 12.0, 13.0, 10.0, 200.0),
            ("2024-01-03", 14.0, 15.0, 13.0, 300.0),
            ("2024-01-04", 16.0, 17.0, 15.0, 400.0),
        ]) {
            Some(series) => series,
            None => return,
        };

        assert_eq!(IndicatorEngine::average_volume(&series, 2), Some(250.0));
    }

    #[test]
    fn returns_are_computed_from_the_requested_period() {
        let series = match price_series_from_rows(&[
            ("2024-01-01", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-02", 12.0, 12.0, 12.0, 100.0),
            ("2024-01-03", 14.0, 14.0, 14.0, 100.0),
            ("2024-01-04", 16.0, 16.0, 16.0, 100.0),
        ]) {
            Some(series) => series,
            None => return,
        };

        let result = IndicatorEngine::return_period(&series, 2);
        assert!(result.is_some());
        let value = result.unwrap();
        assert!((value - 0.3333333333333333).abs() < 1e-12);
    }

    #[test]
    fn insufficient_history_returns_none() {
        let series = match price_series_from_rows(&[("2024-01-01", 10.0, 10.0, 10.0, 100.0)]) {
            Some(series) => series,
            None => return,
        };

        assert!(IndicatorEngine::sma(&series, 2).is_none());
        assert!(IndicatorEngine::donchian_high(&series, 2).is_none());
        assert!(IndicatorEngine::donchian_low(&series, 2).is_none());
        assert!(IndicatorEngine::average_volume(&series, 2).is_none());
        assert!(IndicatorEngine::return_period(&series, 2).is_none());
    }

    #[test]
    fn empty_series_is_supported() {
        let series = match PriceSeries::new("TEST", Vec::new()) {
            Ok(series) => series,
            Err(_) => return,
        };

        assert!(IndicatorEngine::sma(&series, 2).is_none());
        assert!(IndicatorEngine::donchian_high(&series, 2).is_none());
        assert!(IndicatorEngine::donchian_low(&series, 2).is_none());
        assert!(IndicatorEngine::average_volume(&series, 2).is_none());
        assert!(IndicatorEngine::return_period(&series, 2).is_none());
    }

    #[test]
    fn single_candle_is_not_accepted_for_periodic_windows() {
        let series = match price_series_from_rows(&[("2024-01-01", 10.0, 10.0, 10.0, 100.0)]) {
            Some(series) => series,
            None => return,
        };

        assert!(IndicatorEngine::sma(&series, 1).is_some());
        assert!(IndicatorEngine::donchian_high(&series, 1).is_none());
        assert!(IndicatorEngine::donchian_low(&series, 1).is_none());
        assert!(IndicatorEngine::average_volume(&series, 1).is_none());
        assert!(IndicatorEngine::return_period(&series, 1).is_none());
    }

    #[test]
    fn analyze_populates_only_the_requested_fields() {
        let series = match price_series_from_rows(&[
            ("2024-01-01", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-02", 12.0, 12.0, 12.0, 200.0),
            ("2024-01-03", 14.0, 14.0, 14.0, 300.0),
            ("2024-01-04", 16.0, 16.0, 16.0, 400.0),
        ]) {
            Some(series) => series,
            None => return,
        };

        let mut builder = StockAnalysisBuilder::new();
        let result = IndicatorEngine::analyze(&series, &mut builder);

        assert!(result.is_ok());
        let analysis = builder.build();
        assert_eq!(analysis.latest_close, Some(16.0));
        assert_eq!(analysis.latest_volume, Some(400.0));
        assert_eq!(
            analysis.latest_date,
            Some(parse_date("2024-01-04").unwrap_or_default())
        );
        assert_eq!(analysis.sma50, None);
        assert_eq!(analysis.sma200, None);
        assert_eq!(analysis.donchian_high55, None);
        assert_eq!(analysis.donchian_low20, None);
        assert_eq!(analysis.average_volume50, None);
        assert_eq!(analysis.return6m, None);
        assert_eq!(analysis.return12m, None);
        assert_eq!(analysis.signal, None);
    }
}
