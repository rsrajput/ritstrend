use anyhow::Result;
use chrono::NaiveDate;

use crate::models::Candle;

/// A chronological collection of OHLCV candles for a single stock.
///
/// The series owns the complete historical record and exposes it through a
/// slice-based API so downstream modules can analyze the data without needing
/// to manipulate the underlying vector directly.
#[derive(Debug, Clone)]
pub struct PriceSeries {
    symbol: String,
    #[allow(dead_code)]
    candles: Vec<Candle>,
}

#[allow(dead_code)]
impl PriceSeries {
    /// Create a new price series after validating the candle order.
    ///
    /// # Examples
    ///
    /// ```
    /// use ritstrend::models::Candle;
    /// use ritstrend::price_series::PriceSeries;
    /// use chrono::NaiveDate;
    ///
    /// let candles = vec![
    ///     Candle { date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(), open: 10.0, high: 11.0, low: 9.5, close: 10.5, volume: 1000.0 },
    /// ];
    /// let series = PriceSeries::new("TEST", candles).expect("series should be created");
    /// assert_eq!(series.len(), 1);
    /// ```
    pub fn new(symbol: impl Into<String>, candles: Vec<Candle>) -> Result<Self> {
        let symbol = symbol.into();

        let mut previous_date = None;
        for candle in &candles {
            if let Some(previous) = previous_date {
                anyhow::ensure!(
                    candle.date >= previous,
                    "candles for '{}' are not sorted chronologically",
                    symbol
                );
            }
            previous_date = Some(candle.date);
        }

        Ok(Self { symbol, candles })
    }

    /// Return the symbol associated with the series.
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Return the number of candles in the series.
    pub fn len(&self) -> usize {
        self.candles.len()
    }

    /// Return true when the series is empty.
    pub fn is_empty(&self) -> bool {
        self.candles.is_empty()
    }

    /// Return the latest candle when the series is not empty.
    pub fn latest(&self) -> Option<&Candle> {
        self.candles.last()
    }

    /// Return the first candle when the series is not empty.
    pub fn first(&self) -> Option<&Candle> {
        self.candles.first()
    }

    /// Return a slice containing all candles in the series.
    pub fn candles(&self) -> &[Candle] {
        &self.candles
    }

    /// Return the date at the requested index, if present.
    pub fn date(&self, index: usize) -> Option<NaiveDate> {
        self.candles.get(index).map(|candle| candle.date)
    }

    /// Return the close price at the requested index, if present.
    pub fn close(&self, index: usize) -> Option<f64> {
        self.candles.get(index).map(|candle| candle.close)
    }

    /// Return the high price at the requested index, if present.
    pub fn high(&self, index: usize) -> Option<f64> {
        self.candles.get(index).map(|candle| candle.high)
    }

    /// Return the low price at the requested index, if present.
    pub fn low(&self, index: usize) -> Option<f64> {
        self.candles.get(index).map(|candle| candle.low)
    }

    /// Return the open price at the requested index, if present.
    pub fn open(&self, index: usize) -> Option<f64> {
        self.candles.get(index).map(|candle| candle.open)
    }

    /// Return the volume at the requested index, if present.
    pub fn volume(&self, index: usize) -> Option<f64> {
        self.candles.get(index).map(|candle| candle.volume)
    }

    /// Return an iterator over the candles in the series.
    pub fn iter(&self) -> std::slice::Iter<'_, Candle> {
        self.candles.iter()
    }

    /// Return an iterator over the dates in the series.
    pub fn dates(&self) -> impl Iterator<Item = NaiveDate> + '_ {
        self.candles.iter().map(|candle| candle.date)
    }

    /// Return an iterator over the close prices in the series.
    pub fn closes(&self) -> impl Iterator<Item = f64> + '_ {
        self.candles.iter().map(|candle| candle.close)
    }

    /// Return an iterator over the open prices in the series.
    pub fn opens(&self) -> impl Iterator<Item = f64> + '_ {
        self.candles.iter().map(|candle| candle.open)
    }

    /// Return an iterator over the high prices in the series.
    pub fn highs(&self) -> impl Iterator<Item = f64> + '_ {
        self.candles.iter().map(|candle| candle.high)
    }

    /// Return an iterator over the low prices in the series.
    pub fn lows(&self) -> impl Iterator<Item = f64> + '_ {
        self.candles.iter().map(|candle| candle.low)
    }

    /// Return an iterator over the volumes in the series.
    pub fn volumes(&self) -> impl Iterator<Item = f64> + '_ {
        self.candles.iter().map(|candle| candle.volume)
    }

    /// Return a slice containing the last `period` candles when enough history exists.
    pub fn window(&self, period: usize) -> Option<&[Candle]> {
        if period == 0 {
            return None;
        }

        let start = self.candles.len().checked_sub(period)?;
        self.candles.get(start..)
    }

    /// Return a slice containing the last `period` candles when enough history exists.
    pub fn last(&self, period: usize) -> Option<&[Candle]> {
        self.window(period)
    }

    /// Return a slice containing all candles except the last `n` candles.
    pub fn skip_last(&self, n: usize) -> Option<&[Candle]> {
        if n == 0 {
            return Some(&self.candles);
        }

        let end = self.candles.len().checked_sub(n)?;
        self.candles.get(..end)
    }

    /// Return the highest high value over the last `period` candles.
    pub fn highest_high(&self, period: usize) -> Option<f64> {
        self.window(period)
            .and_then(|window| window.iter().map(|candle| candle.high).reduce(f64::max))
    }

    /// Return the lowest low value over the last `period` candles.
    pub fn lowest_low(&self, period: usize) -> Option<f64> {
        self.window(period)
            .and_then(|window| window.iter().map(|candle| candle.low).reduce(f64::min))
    }

    /// Return the highest close value over the last `period` candles.
    pub fn highest_close(&self, period: usize) -> Option<f64> {
        self.window(period)
            .and_then(|window| window.iter().map(|candle| candle.close).reduce(f64::max))
    }

    /// Return the lowest close value over the last `period` candles.
    pub fn lowest_close(&self, period: usize) -> Option<f64> {
        self.window(period)
            .and_then(|window| window.iter().map(|candle| candle.close).reduce(f64::min))
    }

    /// Return the average volume over the last `period` candles.
    pub fn average_volume(&self, period: usize) -> Option<f64> {
        let window = self.window(period)?;
        let total: f64 = window.iter().map(|candle| candle.volume).sum();
        Some(total / window.len() as f64)
    }

    /// Return the simple return over `period` periods.
    ///
    /// The return is computed as the latest close divided by the close
    /// `period` candles ago minus one.
    pub fn simple_return(&self, period: usize) -> Option<f64> {
        if period == 0 {
            return None;
        }

        let current_index = self.candles.len().checked_sub(1)?;
        let prior_index = current_index.checked_sub(period)?;
        let current_close = self.candles.get(current_index)?.close;
        let prior_close = self.candles.get(prior_index)?.close;

        if prior_close == 0.0 {
            return None;
        }

        Some(current_close / prior_close - 1.0)
    }

    /// Return the percentage return over `period` periods.
    pub fn percent_return(&self, period: usize) -> Option<f64> {
        self.simple_return(period).map(|value| value * 100.0)
    }

    /// Return the absolute point change over `period` periods.
    pub fn price_change(&self, period: usize) -> Option<f64> {
        let current_index = self.candles.len().checked_sub(1)?;
        let prior_index = current_index.checked_sub(period)?;
        let current_close = self.candles.get(current_index)?.close;
        let prior_close = self.candles.get(prior_index)?.close;
        Some(current_close - prior_close)
    }

    /// Return true when the series contains at least `period` candles.
    pub fn has_minimum_history(&self, period: usize) -> bool {
        self.candles.len() >= period
    }

    /// Return the length of history in years, if there is at least one candle.
    pub fn history_years(&self) -> Option<f64> {
        let first = self.candles.first()?;
        let last = self.candles.last()?;
        let days = last.date.signed_duration_since(first.date).num_days();
        Some(days as f64 / 365.25)
    }

    /// Return the latest close price when the series is not empty.
    pub fn latest_close(&self) -> Option<f64> {
        self.latest().map(|candle| candle.close)
    }

    /// Return the latest volume when the series is not empty.
    pub fn latest_volume(&self) -> Option<f64> {
        self.latest().map(|candle| candle.volume)
    }

    /// Return the latest date when the series is not empty.
    pub fn latest_date(&self) -> Option<NaiveDate> {
        self.latest().map(|candle| candle.date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candle(date: &str, close: f64, high: f64, low: f64, volume: f64) -> Candle {
        Candle {
            date: NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("valid test date"),
            open: close - 1.0,
            high,
            low,
            close,
            volume,
        }
    }

    fn series_with_dates(
        dates: &[&str],
        closes: &[f64],
        highs: &[f64],
        lows: &[f64],
        volumes: &[f64],
    ) -> PriceSeries {
        let candles = dates
            .iter()
            .zip(
                closes
                    .iter()
                    .zip(highs.iter().zip(lows.iter().zip(volumes.iter()))),
            )
            .map(|(date, (close, (high, (low, volume))))| {
                candle(date, *close, *high, *low, *volume)
            })
            .collect();

        PriceSeries::new("TEST", candles).expect("valid price series")
    }

    #[test]
    fn empty_series_is_supported() {
        let series = PriceSeries::new("TEST", Vec::new()).expect("empty series is valid");
        assert!(series.is_empty());
        assert_eq!(series.len(), 0);
        assert!(series.latest().is_none());
        assert!(series.first().is_none());
        assert!(series.window(3).is_none());
        assert!(series.average_volume(3).is_none());
        assert!(series.simple_return(1).is_none());
    }

    #[test]
    fn single_candle_is_exposed_correctly() {
        let series = series_with_dates(&["2024-01-02"], &[10.0], &[11.0], &[9.0], &[100.0]);
        assert_eq!(series.symbol(), "TEST");
        assert_eq!(series.len(), 1);
        assert_eq!(series.latest_close(), Some(10.0));
        assert_eq!(series.latest_volume(), Some(100.0));
        assert_eq!(
            series.latest_date(),
            Some(NaiveDate::parse_from_str("2024-01-02", "%Y-%m-%d").expect("valid date"))
        );
        assert!(series.window(1).is_some());
        assert_eq!(series.highest_high(1), Some(11.0));
        assert_eq!(series.lowest_low(1), Some(9.0));
    }

    #[test]
    fn small_series_supports_window_and_scans() {
        let series = series_with_dates(
            &["2024-01-01", "2024-01-02", "2024-01-03"],
            &[10.0, 12.0, 9.0],
            &[11.0, 13.0, 10.0],
            &[9.0, 10.0, 8.5],
            &[100.0, 150.0, 120.0],
        );

        let window = series.window(2).expect("window exists");
        assert_eq!(window.len(), 2);
        assert_eq!(window[0].close, 12.0);
        assert_eq!(window[1].close, 9.0);

        assert_eq!(series.highest_high(2), Some(13.0));
        assert_eq!(series.lowest_low(2), Some(8.5));
        assert_eq!(series.average_volume(2), Some(135.0));
        assert_eq!(series.highest_close(2), Some(12.0));
        assert_eq!(series.lowest_close(2), Some(9.0));
    }

    #[test]
    fn insufficient_history_returns_none() {
        let series = series_with_dates(
            &["2024-01-01", "2024-01-02", "2024-01-03"],
            &[10.0, 12.0, 9.0],
            &[11.0, 13.0, 10.0],
            &[9.0, 10.0, 8.5],
            &[100.0, 150.0, 120.0],
        );

        assert!(series.window(4).is_none());
        assert!(series.highest_high(4).is_none());
        assert!(series.average_volume(4).is_none());
        assert!(series.simple_return(5).is_none());
        assert!(series.price_change(5).is_none());
    }

    #[test]
    fn highest_and_lowest_helpers_work() {
        let series = series_with_dates(
            &["2024-01-01", "2024-01-02", "2024-01-03", "2024-01-04"],
            &[10.0, 12.0, 9.0, 14.0],
            &[11.0, 13.0, 10.0, 15.0],
            &[9.0, 10.0, 8.5, 12.0],
            &[100.0, 150.0, 120.0, 200.0],
        );

        assert_eq!(series.highest_high(3), Some(15.0));
        assert_eq!(series.lowest_low(3), Some(8.5));
        assert_eq!(series.highest_close(3), Some(14.0));
        assert_eq!(series.lowest_close(3), Some(9.0));
    }

    #[test]
    fn average_volume_uses_the_requested_window() {
        let series = series_with_dates(
            &["2024-01-01", "2024-01-02", "2024-01-03", "2024-01-04"],
            &[10.0, 12.0, 9.0, 14.0],
            &[11.0, 13.0, 10.0, 15.0],
            &[9.0, 10.0, 8.5, 12.0],
            &[100.0, 200.0, 300.0, 400.0],
        );

        assert_eq!(series.average_volume(3), Some(300.0));
    }

    #[test]
    fn window_and_skip_last_expose_the_expected_slice() {
        let series = series_with_dates(
            &["2024-01-01", "2024-01-02", "2024-01-03", "2024-01-04"],
            &[10.0, 12.0, 9.0, 14.0],
            &[11.0, 13.0, 10.0, 15.0],
            &[9.0, 10.0, 8.5, 12.0],
            &[100.0, 200.0, 300.0, 400.0],
        );

        let window = series.window(2).expect("window exists");
        assert_eq!(window[0].close, 9.0);
        assert_eq!(window[1].close, 14.0);

        let skipped = series.skip_last(2).expect("skip last exists");
        assert_eq!(skipped.len(), 2);
        assert_eq!(skipped[0].close, 10.0);
        assert_eq!(skipped[1].close, 12.0);
    }

    #[test]
    fn returns_are_computed_from_the_requested_period() {
        let series = series_with_dates(
            &["2024-01-01", "2024-01-02", "2024-01-03", "2024-01-04"],
            &[10.0, 12.0, 9.0, 14.0],
            &[11.0, 13.0, 10.0, 15.0],
            &[9.0, 10.0, 8.5, 12.0],
            &[100.0, 200.0, 300.0, 400.0],
        );

        assert!((series.simple_return(2).unwrap() - 0.16666666666666674).abs() < 1e-12);
        assert!((series.percent_return(2).unwrap() - 16.666666666666675).abs() < 1e-12);
        assert_eq!(series.price_change(2), Some(2.0));
    }

    #[test]
    fn history_years_is_based_on_first_and_last_dates() {
        let series = series_with_dates(
            &["2020-01-01", "2021-01-01"],
            &[10.0, 11.0],
            &[11.0, 12.0],
            &[9.0, 10.0],
            &[100.0, 110.0],
        );

        let years = series.history_years().expect("history exists");
        assert!(years > 0.99 && years < 1.01);
    }
}
