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
        atr_multiplier: f64,
        near_breakouts: &[NearBreakout],
    ) {
        let mut scores =
            ScoreEngine::score_all(analyses, rs_threshold, volume_factor, atr_multiplier);
        scores.sort_by(|a, b| b.score.cmp(&a.score));

        let _lookup: HashMap<&str, &StockAnalysis> =
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
        println!("                           TRADE SETUP DASHBOARD");
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
        println!("{}", "-".repeat(94));
        println!(
            "{:<4} {:<14} {:>10} {:>12} {:>8} {:>4} {:>5}",
            "Rank", "Symbol", "Entry", "Initial Stop", "Risk %", "RS", "Score"
        );
        println!("{}", "-".repeat(94));

        let buy_candidates: Vec<&crate::score_engine::StockScore> = scores
            .iter()
            .filter(|s| matches!(s.rating, Rating::Buy))
            .collect();

        for (i, s) in buy_candidates.iter().enumerate() {
            let entry = s.entry_price.unwrap_or(0.0);
            let stop = s.stop_price.unwrap_or(0.0);
            let risk = s.risk_percent.unwrap_or(0.0);
            let rs_rank = s.rs_rank;
            let score = s.score;

            println!(
                "{:<4} {:<14} {:>10.2} {:>12.2} {:>7.1}% {:>4} {:>5}",
                i + 1,
                s.symbol,
                entry,
                stop,
                risk,
                rs_rank,
                score
            );
        }

        println!("{}", "-".repeat(94));

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

        println!("{}", "-".repeat(94));
    }
}
