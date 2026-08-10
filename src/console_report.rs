//! console_report.rs
use crate::analysis::StockAnalysis;
use crate::market_summary::MarketSummary;
use crate::near_breakout::NearBreakout;
use crate::score_engine::{Rating, ScoreEngine};
use std::collections::HashMap;

pub struct ConsoleReport;

impl ConsoleReport {
    pub fn print(
        analyses: &[StockAnalysis],
        summary: &MarketSummary,
        rs_threshold: usize,
        volume_factor: f64,
        near_breakouts: &[NearBreakout],
    ) {
        let mut scores = ScoreEngine::score_all(analyses, rs_threshold, volume_factor);
        scores.sort_by(|a, b| b.score.cmp(&a.score));

        let lookup: HashMap<&str, &StockAnalysis> =
            analyses.iter().map(|a| (a.symbol.as_str(), a)).collect();

        let buy = scores
            .iter()
            .filter(|s| matches!(s.rating, Rating::Buy))
            .count();
        let watch = scores
            .iter()
            .filter(|s| matches!(s.rating, Rating::Watch))
            .count();
        let monitor = scores
            .iter()
            .filter(|s| matches!(s.rating, Rating::Monitor))
            .count();
        let ignore = scores
            .iter()
            .filter(|s| matches!(s.rating, Rating::Ignore))
            .count();

        println!();
        println!("==========================================================================");
        println!("                           SCAN SUMMARY");
        println!("==========================================================================");
        println!(
            "Stocks Loaded : {}    Missing Files : {}",
            summary.stocks_loaded, summary.missing_files,
        );
        println!(
            "BUY:{}  WATCH:{}  MONITOR:{}  IGNORE:{}",
            summary.buy_count, summary.watch_count, summary.monitor_count, summary.ignore_count
        );
        println!(
            "Strongest     : {} (Score {})",
            summary.strongest_symbol, summary.highest_score
        );
        println!("==========================================================================");

        println!();
        println!("==========================================================================");
        println!("                        TRADE SETUPS & WATCHLIST");
        println!("==========================================================================");
        println!(
            "Stocks Loaded : {}    Missing Files : {}",
            analyses.len(),
            summary.missing_files
        );
        println!(
            "BUY:{}  WATCH:{}  MONITOR:{}  IGNORE:{}",
            buy, watch, monitor, ignore
        );
        println!("{}", "-".repeat(132));
        println!(
            "{:<4} {:<14} {:>10} {:>10} {:>10} {:>10} {:>11} {:>13} {:>11} {:>4} {:>5} {:<9} {}",
            "Rank",
            "Symbol",
            "Close",
            "ATR(15)%",
            "ATR×1.5%",
            "ATR×2%",
            "Swing Low",
            "Structure SL",
            "Chandelier",
            "RS",
            "Score",
            "Rating",
            "Primary Reason"
        );
        println!("(BUY, WATCH, MONITOR and IGNORE are shown together in this version.)");
        println!("{}", "-".repeat(132));
        for (i, s) in scores.iter().take(20).enumerate() {
            let analysis = lookup.get(s.symbol.as_str()).copied();

            let close = analysis.and_then(|a| a.latest_close).unwrap_or(0.0);

            let rs_rank = analysis.and_then(|a| a.relative_strength_rank).unwrap_or(0);

            let atr15 = analysis.and_then(|a| a.atr15).unwrap_or(0.0);
            let atr_percent = if close > 0.0 {
                (atr15 / close) * 100.0
            } else {
                0.0
            };
            let atr_1_5_percent = atr_percent * 1.5;
            let atr_2_percent = atr_percent * 2.0;

            // Stop levels are displayed only as percentages below the current
            // close, matching Zerodha's percentage-based TSL workflow.
            //
            // These values now come from the dedicated smart-stop calculations
            // in indicator_engine.rs rather than being recalculated from the
            // old Donchian fields here.
            //
            // Swing Low: most recent confirmed pivot low.
            // Structure SL: Swing Low - 0.5 x ATR(15).
            // Chandelier: Chandelier high - 3 x ATR(15).
            let swing_low_percent = analysis
                .and_then(|a| a.swing_low)
                .map(|swing_low| {
                    if close > 0.0 {
                        ((close - swing_low) / close) * 100.0
                    } else {
                        0.0
                    }
                })
                .unwrap_or(0.0);

            let structure_sl_percent = analysis
                .and_then(|a| a.swing_low)
                .map(|swing_low| {
                    if close > 0.0 {
                        let structure_sl = swing_low - (atr15 * 0.5);
                        ((close - structure_sl) / close) * 100.0
                    } else {
                        0.0
                    }
                })
                .unwrap_or(0.0);

            let chandelier_percent = analysis
                .and_then(|a| a.chandelier_high)
                .map(|highest_high| {
                    if close > 0.0 {
                        let chandelier = highest_high - (atr15 * 3.0);
                        Some(((close - chandelier) / close) * 100.0)
                    } else {
                        None
                    }
                })
                .flatten();

            let rating = match s.rating {
                Rating::Buy => "BUY",
                Rating::Watch => "WATCH",
                Rating::Monitor => "MONITOR",
                Rating::Ignore => "IGNORE",
            };
            let reason = s
                .reasons
                .first()
                .map(String::as_str)
                .unwrap_or("All conditions satisfied");
            println!(
                "{:<4} {:<14} {:>10.2} {:>9.2}% {:>9.2}% {:>9.2}% {:>10.2}% {:>12.2}% {:>10} {:>4} {:>5} {:<9} {}",
                i + 1,
                s.symbol,
                close,
                atr_percent,
                atr_1_5_percent,
                atr_2_percent,
                swing_low_percent,
                structure_sl_percent,
                match chandelier_percent {
                    Some(value) => format!("{:.2}%", value),
                    None => "N/A".to_string(),
                },
                rs_rank,
                s.score,
                rating,
                reason
            );
        }

        println!("{}", "-".repeat(132));

        println!();
        println!("==========================================================================");
        println!("             HIGH-QUALITY NEAR BREAKOUTS (≤ 2%)");
        println!("==========================================================================");
        println!(
            "{:<4} {:<14} {:>10} {:>8} {:>6} {:>5}",
            "Rank", "Symbol", "Close", "Dist%", "ADX", "RS"
        );
        println!("{}", "-".repeat(94));
        for (i, nb) in near_breakouts.iter().take(10).enumerate() {
            println!(
                "{:<4} {:<14} {:>10.2} {:>7.2}% {:>6.1} {:>5}",
                i + 1,
                nb.symbol,
                nb.close,
                nb.distance_percent,
                nb.adx,
                nb.rs_rank
            );
        }

        println!("{}", "-".repeat(108));
    }
}
