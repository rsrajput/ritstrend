use crate::analysis::StockAnalysis;
use crate::models::Signal;

#[derive(Debug, Default)]
pub struct Screener;

impl Screener {
    pub fn new() -> Self { Self }

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

        let mut pass_close_sma200 = 0usize;
        let mut pass_sma = 0usize;
        let mut pass_adx = 0usize;
        let mut pass_volume = 0usize;
        let mut pass_rs = 0usize;
        let mut pass_donchian = 0usize;

        let mut all_except_volume = 0usize;
        let mut all_except_donchian = 0usize;
        let mut all_except_adx = 0usize;

        for a in analyses {
            let c = a.latest_close.unwrap_or_default();
            let d = a.donchian_high55.unwrap_or_default();
            let s50 = a.sma50.unwrap_or_default();
            let s200 = a.sma200.unwrap_or_default();
            let adx = a.adx14.unwrap_or_default();
            let vol = a.latest_volume.unwrap_or_default();
            let avg = a.average_volume50.unwrap_or_default();
            let rs = a.relative_strength_rank.unwrap_or(usize::MAX);

            let r_close = c > s200;
            let r_sma = s50 > s200;
            let r_adx = adx > 25.0;
            let r_vol = vol > volume_factor * avg;
            let r_rs = rs <= threshold;
            let r_don = c > d;

            if r_close { pass_close_sma200 += 1; }
            if r_sma { pass_sma += 1; }
            if r_adx { pass_adx += 1; }
            if r_vol { pass_volume += 1; }
            if r_rs { pass_rs += 1; }
            if r_don { pass_donchian += 1; }

            if r_close && r_sma && r_adx && r_rs && r_don { all_except_volume += 1; }
            if r_close && r_sma && r_adx && r_rs && r_vol { all_except_donchian += 1; }
            if r_close && r_sma && r_don && r_rs && r_vol { all_except_adx += 1; }
        }

        println!("\n===== SCREENING DIAGNOSTICS =====");
        println!("Close > SMA200          : {}", pass_close_sma200);
        println!("SMA50 > SMA200          : {}", pass_sma);
        println!("ADX > 25                : {}", pass_adx);
        println!("Volume > factor*Avg50   : {}", pass_volume);
        println!("RS Rank <= threshold    : {}", pass_rs);
        println!("Close > Donchian High55 : {}", pass_donchian);
        println!("--------------------------------");
        println!("All except Volume       : {}", all_except_volume);
        println!("All except Donchian     : {}", all_except_donchian);
        println!("All except ADX          : {}", all_except_adx);
        println!("===============================\n");

        analyses.iter()
            .filter(|analysis| {
                let c = analysis.latest_close.unwrap_or_default();
                let d = analysis.donchian_high55.unwrap_or_default();
                let s200 = analysis.sma200.unwrap_or_default();
                let s50 = analysis.sma50.unwrap_or_default();
                let adx = analysis.adx14.unwrap_or_default();
                let vol = analysis.latest_volume.unwrap_or_default();
                let avg = analysis.average_volume50.unwrap_or_default();
                let rs = analysis.relative_strength_rank.unwrap_or(usize::MAX);

                c > d &&
                c > s200 &&
                s50 > s200 &&
                adx > 25.0 &&
                vol > volume_factor * avg &&
                rs <= threshold
            })
            .cloned()
            .map(|mut a| {
                a.signal = Some(Signal::Buy);
                a.reasons.push("met buy criteria".to_string());
                a
            })
            .collect()
    }
}
