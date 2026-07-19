use crate::models::Candle;
use crate::price_series::PriceSeries;

/// Computes Wilder's Average True Range for a price series.
///
/// The calculation uses the original Wilder ATR formulation:
/// - the first ATR is the average of the first `period` true ranges;
/// - subsequent ATR values use the recursive Wilder update.
#[derive(Debug, Default, Clone, Copy)]
pub struct AtrCalculator;

impl AtrCalculator {
    /// Return the latest Wilder ATR value for the provided series.
    ///
    /// The function requires enough history to compute the first ATR from the
    /// first `period` true ranges. A series shorter than `period + 1` candles
    /// does not contain enough data to produce a value.
    pub fn atr(series: &PriceSeries, period: usize) -> Option<f64> {
        if period == 0 || series.len() <= period {
            return None;
        }

        let candles = series.candles();
        let mut previous = candles.first()?;
        let mut tr_sum = 0.0;
        let mut atr = None;

        for (index, candle) in candles.iter().enumerate().skip(1) {
            let tr = true_range(previous, candle);
            tr_sum += tr;

            if index == period {
                atr = Some(tr_sum / period as f64);
            } else if let Some(previous_atr) = atr {
                let updated = ((previous_atr * (period - 1) as f64) + tr) / period as f64;
                atr = Some(updated);
            }

            previous = candle;
        }

        atr
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
        assert!(AtrCalculator::atr(&series, 2).is_none());
    }

    #[test]
    fn single_candle_returns_none() {
        let series = series_with_rows(&[("2024-01-01", 10.0, 10.0, 10.0, 100.0)]);
        assert!(AtrCalculator::atr(&series, 2).is_none());
    }

    #[test]
    fn exactly_period_candles_do_not_have_enough_history() {
        let series = series_with_rows(&[
            ("2024-01-01", 10.0, 10.0, 10.0, 100.0),
            ("2024-01-02", 12.0, 12.0, 12.0, 100.0),
        ]);

        assert!(AtrCalculator::atr(&series, 2).is_none());
    }

    #[test]
    fn exactly_period_true_ranges_use_the_initial_average() {
        let series = series_with_rows(&[
            ("2024-01-01", 10.0, 10.0, 9.0, 100.0),
            ("2024-01-02", 12.0, 12.0, 10.0, 100.0),
            ("2024-01-03", 13.0, 14.0, 12.0, 100.0),
            ("2024-01-04", 11.0, 13.0, 10.0, 100.0),
        ]);

        let result = AtrCalculator::atr(&series, 3);
        if let Some(value) = result {
            assert!((value - 2.3333333333333335).abs() < 1e-12);
        } else {
            panic!("ATR should be available for four candles with period 3");
        }
    }

    #[test]
    fn more_than_period_candles_use_the_wilder_recursion() {
        let series = series_with_rows(&[
            ("2024-01-01", 10.0, 10.0, 9.0, 100.0),
            ("2024-01-02", 12.0, 12.0, 10.0, 100.0),
            ("2024-01-03", 13.0, 14.0, 12.0, 100.0),
            ("2024-01-04", 11.0, 13.0, 10.0, 100.0),
            ("2024-01-05", 14.0, 15.0, 11.0, 100.0),
        ]);

        let result = AtrCalculator::atr(&series, 3);
        if let Some(value) = result {
            assert!((value - 2.888888888888889).abs() < 1e-12);
        } else {
            panic!("ATR should be available for five candles with period 3");
        }
    }

    #[test]
    fn known_atr_example_matches_the_reference_value() {
        let series = series_with_rows(&[
            ("2024-01-01", 10.0, 10.0, 9.0, 100.0),
            ("2024-01-02", 12.0, 12.0, 10.0, 100.0),
            ("2024-01-03", 13.0, 14.0, 12.0, 100.0),
            ("2024-01-04", 11.0, 13.0, 10.0, 100.0),
            ("2024-01-05", 14.0, 15.0, 11.0, 100.0),
            ("2024-01-06", 15.0, 16.0, 14.0, 100.0),
        ]);

        let result = AtrCalculator::atr(&series, 3);
        if let Some(value) = result {
            assert!((value - 2.592592592592593).abs() < 1e-12);
        } else {
            panic!("ATR should be available for six candles with period 3");
        }
    }

    #[test]
    fn zero_period_returns_none() {
        let series = series_with_rows(&[("2024-01-01", 10.0, 10.0, 10.0, 100.0)]);
        assert!(AtrCalculator::atr(&series, 0).is_none());
    }
}
