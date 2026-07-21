//! market_health.rs
//! Enhanced market health analysis for RitsTrend.

use crate::analysis::StockAnalysis;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarketRegime {
    StrongBull,
    Bull,
    Sideways,
    Bear,
    StrongBear,
}

#[derive(Debug, Clone)]
pub struct MarketHealth {
    pub total_stocks: usize,
    pub missing_files: usize,

    pub above_sma200: usize,
    pub above_sma50: usize,
    pub adx_above_25: usize,
    pub breakout_55: usize,

    pub bullish_alignment: usize,
    pub bearish_alignment: usize,

    pub average_adx: f64,
    pub average_rs_score: f64,

    pub pct_above_sma200: f64,
    pub pct_above_sma50: f64,
    pub pct_adx_above_25: f64,
    pub pct_breakout_55: f64,

    pub regime: MarketRegime,
}

impl MarketHealth {
    pub fn analyze(analyses: &[StockAnalysis], missing_files: usize) -> Self {
        let total = analyses.len().max(1);

        let mut above200 = 0usize;
        let mut above50 = 0usize;
        let mut adx25 = 0usize;
        let mut breakout = 0usize;
        let mut bull_align = 0usize;
        let mut bear_align = 0usize;

        let mut adx_sum = 0.0;
        let mut adx_count = 0usize;
        let mut rs_sum = 0.0;
        let mut rs_count = 0usize;

        for a in analyses {
            let close = a.latest_close.unwrap_or(0.0);
            let sma50 = a.sma50.unwrap_or(f64::MAX);
            let sma200 = a.sma200.unwrap_or(f64::MAX);

            if close > sma200 { above200 += 1; }
            if close > sma50 { above50 += 1; }

            if close > sma50 && sma50 > sma200 {
                bull_align += 1;
            }
            if close < sma50 && sma50 < sma200 {
                bear_align += 1;
            }

            if let Some(adx) = a.adx14 {
                adx_sum += adx;
                adx_count += 1;
                if adx >= 25.0 {
                    adx25 += 1;
                }
            }

            if let Some(rs) = a.relative_strength_score {
                rs_sum += rs;
                rs_count += 1;
            }

            if close > a.donchian_high55.unwrap_or(f64::MAX) {
                breakout += 1;
            }
        }

        let pct200 = above200 as f64 * 100.0 / total as f64;
        let pct50 = above50 as f64 * 100.0 / total as f64;
        let pctadx = adx25 as f64 * 100.0 / total as f64;
        let pctbreak = breakout as f64 * 100.0 / total as f64;

        let regime = if pct200 >= 70.0 && pctadx >= 35.0 {
            MarketRegime::StrongBull
        } else if pct200 >= 55.0 {
            MarketRegime::Bull
        } else if pct200 >= 40.0 {
            MarketRegime::Sideways
        } else if pct200 >= 25.0 {
            MarketRegime::Bear
        } else {
            MarketRegime::StrongBear
        };

        Self {
            total_stocks: analyses.len(),
            missing_files,
            above_sma200: above200,
            above_sma50: above50,
            adx_above_25: adx25,
            breakout_55: breakout,
            bullish_alignment: bull_align,
            bearish_alignment: bear_align,
            average_adx: if adx_count > 0 { adx_sum / adx_count as f64 } else { 0.0 },
            average_rs_score: if rs_count > 0 { rs_sum / rs_count as f64 } else { 0.0 },
            pct_above_sma200: pct200,
            pct_above_sma50: pct50,
            pct_adx_above_25: pctadx,
            pct_breakout_55: pctbreak,
            regime,
        }
    }

    pub fn regime_name(&self) -> &'static str {
        match self.regime {
            MarketRegime::StrongBull => "★★★★★ Strong Bull",
            MarketRegime::Bull => "★★★★☆ Bull",
            MarketRegime::Sideways => "★★★☆☆ Sideways",
            MarketRegime::Bear => "★★☆☆☆ Bear",
            MarketRegime::StrongBear => "★☆☆☆☆ Strong Bear",
        }
    }
}
