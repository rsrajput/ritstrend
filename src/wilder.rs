/// Generic Wilder smoothing utilities.
///
/// The implementation follows Welles Wilder's New Concepts in Technical
/// Trading Systems by using the recursive formulation: the first smoothed
/// value is the simple average of the first `period` observations, and each
/// subsequent value uses the prior smoothed average.
#[derive(Debug, Default, Clone, Copy)]
pub struct Wilder;

#[allow(dead_code)]
impl Wilder {
    /// Smooth a slice of values using Wilder's recursive method.
    ///
    /// The returned vector contains one smoothed value for each value after the
    /// initial average is available. A period of `0` or fewer than `period`
    /// observations returns `None`.
    pub fn smooth(values: &[f64], period: usize) -> Option<Vec<f64>> {
        if period == 0 || values.len() < period {
            return None;
        }

        let mut smoothed =
            Vec::with_capacity(values.len().saturating_sub(period.saturating_sub(1)));
        let initial_average = values.iter().take(period).sum::<f64>() / period as f64;
        smoothed.push(initial_average);

        let mut previous = initial_average;
        for value in values.iter().skip(period) {
            let updated = ((previous * (period - 1) as f64) + value) / period as f64;
            smoothed.push(updated);
            previous = updated;
        }

        Some(smoothed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smooth_requires_enough_values() {
        assert!(Wilder::smooth(&[1.0, 2.0], 3).is_none());
    }

    #[test]
    fn smooth_returns_the_initial_average_and_subsequent_values() {
        let values = vec![1.0, 3.0, 5.0, 7.0];
        let smoothed = Wilder::smooth(&values, 3).expect("smoothing should succeed");

        assert_eq!(smoothed.len(), 2);
        assert!((smoothed[0] - 3.0).abs() < 1e-12);
        assert!((smoothed[1] - 13.0 / 3.0).abs() < 1e-12);
    }
}
