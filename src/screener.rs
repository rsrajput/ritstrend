use crate::analysis::StockAnalysis;
use crate::models::Signal;

#[derive(Debug, Default)]
pub struct Screener;

impl Screener {
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

        println!("\n========== BREAKOUT STOCK DIAGNOSTICS ==========");

        let mut buys = Vec::new();

        for a in analyses {
            let c = a.latest_close.unwrap_or_default();
            let d = a.donchian_high55.unwrap_or_default();
            let s50 = a.sma50.unwrap_or_default();
            let s200 = a.sma200.unwrap_or_default();
            let adx = a.adx14.unwrap_or_default();
            let vol = a.latest_volume.unwrap_or_default();
            let avg = a.average_volume50.unwrap_or_default();
            let rs = a.relative_strength_rank.unwrap_or(usize::MAX);

            let breakout = c > d;

            if breakout {
                let trend = c > s200;
                let ma = s50 > s200;
                let adx_ok = adx > 25.0;
                let vol_ok = vol > volume_factor * avg;
                let rs_ok = rs <= threshold;

                println!(
                    "{:<15} Trend:{} MA:{} ADX:{} Vol:{} RS:{}  Close={:.2} Don={:.2}",
                    a.symbol,
                    if trend { "Y" } else { "N" },
                    if ma { "Y" } else { "N" },
                    if adx_ok { "Y" } else { "N" },
                    if vol_ok { "Y" } else { "N" },
                    if rs_ok { "Y" } else { "N" },
                    c,
                    d
                );

                if trend && ma && adx_ok && vol_ok && rs_ok {
                    let mut buy = a.clone();
                    buy.signal = Some(Signal::Buy);
                    buy.reasons.push("Met BUY criteria".to_string());
                    buys.push(buy);
                }
            }
        }

        println!("===============================================\n");
        buys
    }
}
