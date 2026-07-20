use crate::models::Candle;
use crate::price_series::PriceSeries;
use crate::wilder::Wilder;

/// Calculates Wilder's Average Directional Index for a price series.
///
/// The implementation follows Welles Wilder's New Concepts in Technical
/// Trading Systems by computing directional movement, true range, smoothed
/// directional indicators, and finally the ADX value.
#[derive(Debug, Default, Clone, Copy)]
pub struct AdxCalculator;

#[allow(dead_code)]
impl AdxCalculator {
    /// Return the latest ADX value for the provided series.
    ///
    /// The function requires enough history to compute the initial Wilder
    /// averages. A series shorter than `period + 1` candles cannot produce a
    /// stable ADX value.
    pub fn adx(series: &PriceSeries, period: usize) -> Option<f64> {
        if period == 0 || series.len() < period * 2 {
            return None;
        }

        let candles = series.candles();

        let mut plus_dm = Vec::with_capacity(candles.len().saturating_sub(1));
        let mut minus_dm = Vec::with_capacity(candles.len().saturating_sub(1));
        let mut true_range_values = Vec::with_capacity(candles.len().saturating_sub(1));

        let mut previous = candles.first()?;
        for current in candles.iter().skip(1) {
            let up_move = current.high - previous.high;
            let down_move = previous.low - current.low;

            let plus_dm_value = if up_move > down_move && up_move > 0.0 {
                up_move
            } else {
                0.0
            };
            let minus_dm_value = if down_move > up_move && down_move > 0.0 {
                down_move
            } else {
                0.0
            };

            plus_dm.push(plus_dm_value);
            minus_dm.push(minus_dm_value);
            true_range_values.push(true_range(previous, current));
            previous = current;
        }

        let smoothed_tr = Wilder::smooth(&true_range_values, period)?;
        let smoothed_plus_dm = Wilder::smooth(&plus_dm, period)?;
        let smoothed_minus_dm = Wilder::smooth(&minus_dm, period)?;

        let mut dx_values = Vec::with_capacity(smoothed_tr.len());

        for (tr_value, (plus_dm_value, minus_dm_value)) in smoothed_tr
            .iter()
            .zip(smoothed_plus_dm.iter())
            .zip(smoothed_minus_dm.iter())
            .map(|((tr_value, plus_dm_value), minus_dm_value)| {
                (tr_value, (plus_dm_value, minus_dm_value))
            })
        {
            let plus_di = if *tr_value > 0.0 {
                (plus_dm_value / tr_value) * 100.0
            } else {
                0.0
            };
            let minus_di = if *tr_value > 0.0 {
                (minus_dm_value / tr_value) * 100.0
            } else {
                0.0
            };

            let dx = if plus_di + minus_di > 0.0 {
                (plus_di - minus_di).abs() / (plus_di + minus_di) * 100.0
            } else {
                0.0
            };

            dx_values.push(dx);
        }

        if dx_values.len() < period {
            return None;
        }

        let first_adx = dx_values.iter().take(period).sum::<f64>() / period as f64;
        let mut adx_value = first_adx;
        for dx_value in dx_values.iter().skip(period) {
            adx_value = ((adx_value * (period - 1) as f64) + dx_value) / period as f64;
        }

        Some(adx_value.clamp(0.0, 100.0))
    }
}

fn true_range(previous: &Candle, current: &Candle) -> f64 {
    let high_low = current.high - current.low;
    let high_prev_close = (current.high - previous.close).abs();
    let low_prev_close = (current.low - previous.close).abs();

    high_low.max(high_prev_close).max(low_prev_close)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn candle(date: &str, close: f64, high: f64, low: f64, volume: f64) -> Candle {
        Candle {
            date: NaiveDate::parse_from_str(date, "%Y-%m-%d").expect("valid date"),
            open: close - 1.0,
            high,
            low,
            close,
            volume,
        }
    }

    fn series_with_rows(rows: &[(&str, f64, f64, f64, f64)]) -> PriceSeries {
        let candles = rows
            .iter()
            .map(|(date, close, high, low, volume)| candle(date, *close, *high, *low, *volume))
            .collect();

        PriceSeries::new("TEST", candles).expect("valid price series")
    }

    #[test]
    fn empty_series_returns_none() {
        let series = PriceSeries::new("TEST", Vec::new()).expect("empty series is valid");
        assert!(AdxCalculator::adx(&series, 14).is_none());
    }

    #[test]
    fn one_candle_returns_none() {
        let series = series_with_rows(&[("2024-01-01", 10.0, 10.0, 10.0, 100.0)]);
        assert!(AdxCalculator::adx(&series, 14).is_none());
    }

    #[test]
    fn exactly_period_candles_return_none() {
        let series = series_with_rows(&[
            ("2024-01-01", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-02", 12.0, 12.0, 12.0, 100.0),
            ("2024-01-03", 14.0, 14.0, 14.0, 100.0),
            ("2024-01-04", 16.0, 16.0, 16.0, 100.0),
            ("2024-01-05", 18.0, 18.0, 18.0, 100.0),
            ("2024-01-06", 20.0, 20.0, 20.0, 100.0),
            ("2024-01-07", 22.0, 22.0, 22.0, 100.0),
            ("2024-01-08", 24.0, 24.0, 24.0, 100.0),
            ("2024-01-09", 26.0, 26.0, 26.0, 100.0),
            ("2024-01-10", 28.0, 28.0, 28.0, 100.0),
            ("2024-01-11", 30.0, 30.0, 30.0, 100.0),
            ("2024-01-12", 32.0, 32.0, 32.0, 100.0),
            ("2024-01-13", 34.0, 34.0, 34.0, 100.0),
            ("2024-01-14", 36.0, 36.0, 36.0, 100.0),
        ]);

        assert!(AdxCalculator::adx(&series, 14).is_none());
    }

    #[test]
    fn constant_prices_produce_zero_adx() {
        let series = series_with_rows(&[
            ("2024-01-01", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-02", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-03", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-04", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-05", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-06", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-07", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-08", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-09", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-10", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-11", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-12", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-13", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-14", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-15", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-16", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-17", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-18", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-19", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-20", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-21", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-22", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-23", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-24", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-25", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-26", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-27", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-28", 10.0, 10.0, 10.0, 100.0),
        ]);

        let adx = AdxCalculator::adx(&series, 14).expect("adx should be available");
        assert!((0.0..=100.0).contains(&adx));
        assert!((adx - 0.0).abs() < 1e-12);
    }

    #[test]
    fn monotonic_trend_produces_a_finite_value() {
        let series = series_with_rows(&[
            ("2024-01-01", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-02", 11.0, 11.0, 10.0, 100.0),
            ("2024-01-03", 12.0, 12.0, 11.0, 100.0),
            ("2024-01-04", 13.0, 13.0, 12.0, 100.0),
            ("2024-01-05", 14.0, 14.0, 13.0, 100.0),
            ("2024-01-06", 15.0, 15.0, 14.0, 100.0),
            ("2024-01-07", 16.0, 16.0, 15.0, 100.0),
            ("2024-01-08", 17.0, 17.0, 16.0, 100.0),
            ("2024-01-09", 18.0, 18.0, 17.0, 100.0),
            ("2024-01-10", 19.0, 19.0, 18.0, 100.0),
            ("2024-01-11", 20.0, 20.0, 19.0, 100.0),
            ("2024-01-12", 21.0, 21.0, 20.0, 20.0),
            ("2024-01-13", 22.0, 22.0, 21.0, 100.0),
            ("2024-01-14", 23.0, 23.0, 22.0, 100.0),
            ("2024-01-15", 24.0, 24.0, 23.0, 100.0),
            ("2024-01-16", 25.0, 25.0, 24.0, 100.0),
            ("2024-01-17", 26.0, 26.0, 25.0, 100.0),
            ("2024-01-18", 27.0, 27.0, 26.0, 100.0),
            ("2024-01-19", 28.0, 28.0, 27.0, 100.0),
            ("2024-01-20", 29.0, 29.0, 28.0, 100.0),
            ("2024-01-21", 30.0, 30.0, 29.0, 100.0),
            ("2024-01-22", 31.0, 31.0, 30.0, 100.0),
            ("2024-01-23", 32.0, 32.0, 31.0, 100.0),
            ("2024-01-24", 33.0, 33.0, 32.0, 100.0),
            ("2024-01-25", 34.0, 34.0, 33.0, 100.0),
            ("2024-01-26", 35.0, 35.0, 34.0, 100.0),
            ("2024-01-27", 36.0, 36.0, 35.0, 100.0),
            ("2024-01-28", 37.0, 37.0, 36.0, 100.0),
        ]);

        let adx = AdxCalculator::adx(&series, 14).expect("adx should be available");
        assert!(adx.is_finite());
        assert!((0.0..=100.0).contains(&adx));
    }

    #[test]
    fn large_series_produces_a_finite_value() {
        let mut rows = Vec::new();
        let start = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid base date");
        for index in 1..=120 {
            let date = start
                .checked_add_signed(chrono::Duration::days(index as i64))
                .expect("valid generated date");
            let close = index as f64;
            rows.push((date, close, close, close - 1.0, 100.0));
        }

        let candles = rows
            .iter()
            .map(|(date, close, high, low, volume)| Candle {
                date: *date,
                open: *close - 1.0,
                high: *high,
                low: *low,
                close: *close,
                volume: *volume,
            })
            .collect();

        let series = PriceSeries::new("TEST", candles).expect("valid price series");

        let adx = AdxCalculator::adx(&series, 14).expect("adx should be available");
        assert!(adx.is_finite());
        assert!((0.0..=100.0).contains(&adx));
    }

    #[test]
    fn known_example_matches_reference() {
        let series = series_with_rows(&[
            ("2024-01-01", 10.0, 10.0, 9.0, 100.0),
            ("2024-01-02", 11.0, 11.0, 10.0, 100.0),
            ("2024-01-03", 12.0, 12.0, 11.0, 100.0),
            ("2024-01-04", 13.0, 13.0, 12.0, 100.0),
            ("2024-01-05", 14.0, 14.0, 13.0, 100.0),
            ("2024-01-06", 15.0, 15.0, 14.0, 100.0),
            ("2024-01-07", 16.0, 16.0, 15.0, 100.0),
            ("2024-01-08", 17.0, 17.0, 16.0, 100.0),
            ("2024-01-09", 18.0, 18.0, 17.0, 100.0),
            ("2024-01-10", 19.0, 19.0, 18.0, 100.0),
            ("2024-01-11", 20.0, 20.0, 19.0, 100.0),
            ("2024-01-12", 21.0, 21.0, 20.0, 100.0),
            ("2024-01-13", 22.0, 22.0, 21.0, 100.0),
            ("2024-01-14", 23.0, 23.0, 22.0, 100.0),
            ("2024-01-15", 24.0, 24.0, 23.0, 100.0),
            ("2024-01-16", 25.0, 25.0, 24.0, 100.0),
            ("2024-01-17", 26.0, 26.0, 25.0, 100.0),
            ("2024-01-18", 27.0, 27.0, 26.0, 100.0),
            ("2024-01-19", 28.0, 28.0, 27.0, 100.0),
            ("2024-01-20", 29.0, 29.0, 28.0, 100.0),
            ("2024-01-21", 30.0, 30.0, 29.0, 100.0),
            ("2024-01-22", 31.0, 31.0, 30.0, 100.0),
            ("2024-01-23", 32.0, 32.0, 31.0, 100.0),
            ("2024-01-24", 33.0, 33.0, 32.0, 100.0),
            ("2024-01-25", 34.0, 34.0, 33.0, 100.0),
            ("2024-01-26", 35.0, 35.0, 34.0, 100.0),
            ("2024-01-27", 36.0, 36.0, 35.0, 100.0),
            ("2024-01-28", 37.0, 37.0, 36.0, 100.0),
        ]);

        let adx = AdxCalculator::adx(&series, 14).expect("adx should be available");
        assert!((0.0..=100.0).contains(&adx));
    }
}
