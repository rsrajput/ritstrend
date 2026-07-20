use crate::analysis::StockAnalysis;
use crate::models::Signal;

/// Screening and signal generation for buy candidates.
#[derive(Debug, Default)]
pub struct Screener;

impl Screener {
    /// Create a new screener instance.
    pub fn new() -> Self {
        Self
    }

    /// Return the stocks that satisfy the BUY conditions.
    pub fn screen(
        analyses: &[StockAnalysis],
        top_percent: usize,
        volume_factor: f64,
    ) -> Vec<StockAnalysis> {
        if analyses.is_empty() {
            return Vec::new();
        }

        let threshold = ((analyses.len() as f64) * (top_percent as f64 / 100.0)).ceil() as usize;
        let threshold = threshold.max(1);

        analyses
            .iter()
            .filter(|analysis| {
                let latest_close = analysis.latest_close.unwrap_or_default();
                let donchian_high55 = analysis.donchian_high55.unwrap_or_default();
                let sma200 = analysis.sma200.unwrap_or_default();
                let sma50 = analysis.sma50.unwrap_or_default();
                let adx14 = analysis.adx14.unwrap_or_default();
                let latest_volume = analysis.latest_volume.unwrap_or_default();
                let average_volume50 = analysis.average_volume50.unwrap_or_default();
                let rs_rank = analysis.relative_strength_rank.unwrap_or(usize::MAX);

                latest_close > donchian_high55
                    && latest_close > sma200
                    && sma50 > sma200
                    && adx14 > 25.0
                    && latest_volume > volume_factor * average_volume50
                    && rs_rank <= threshold
            })
            .cloned()
            .map(|mut analysis| {
                analysis.signal = Some(Signal::Buy);
                analysis.reasons.push(String::from("met buy criteria"));
                analysis
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::analysis::StockAnalysisBuilder;

    #[test]
    fn keeps_only_stocks_that_satisfy_all_buy_conditions() {
        let analyses = vec![
            StockAnalysisBuilder::new()
                .symbol("BUY")
                .latest_close(120.0)
                .latest_volume(300.0)
                .sma50(110.0)
                .sma200(100.0)
                .adx14(30.0)
                .average_volume50(100.0)
                .donchian_high55(110.0)
                .relative_strength_rank(1)
                .build(),
            StockAnalysisBuilder::new()
                .symbol("REJECT")
                .latest_close(90.0)
                .latest_volume(300.0)
                .sma50(110.0)
                .sma200(100.0)
                .adx14(30.0)
                .average_volume50(100.0)
                .donchian_high55(110.0)
                .relative_strength_rank(1)
                .build(),
        ];

        let buys = Screener::screen(&analyses, 25, 1.5);

        assert_eq!(buys.len(), 1);
        assert_eq!(buys[0].symbol, "BUY");
        assert_eq!(buys[0].signal, Some(Signal::Buy));
    }
}
