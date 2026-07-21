//! console_report.rs
use crate::analysis::StockAnalysis;
use crate::score_engine::{Rating, ScoreEngine};
use crate::near_breakout::NearBreakout;
use std::collections::HashMap;

pub struct ConsoleReport;

impl ConsoleReport {
    pub fn print(
        analyses: &[StockAnalysis],
        missing_files: usize,
        rs_threshold: usize,
        volume_factor: f64,
        near_breakouts: &[NearBreakout],
    ) {
        let mut scores = ScoreEngine::score_all(analyses, rs_threshold, volume_factor);
        scores.sort_by(|a,b| b.score.cmp(&a.score));

        let lookup: HashMap<&str, &StockAnalysis> =
            analyses.iter().map(|a| (a.symbol.as_str(), a)).collect();

        let buy=scores.iter().filter(|s| matches!(s.rating,Rating::Buy)).count();
        let watch=scores.iter().filter(|s| matches!(s.rating,Rating::Watch)).count();
        let monitor=scores.iter().filter(|s| matches!(s.rating,Rating::Monitor)).count();
        let ignore=scores.iter().filter(|s| matches!(s.rating,Rating::Ignore)).count();

        println!();
        println!("==========================================================================");
        println!("                           RITS TREND v0.4");
        println!("==========================================================================");
        println!("Stocks Loaded : {}    Missing Files : {}", analyses.len(), missing_files);
        println!("BUY:{}  WATCH:{}  MONITOR:{}  IGNORE:{}", buy,watch,monitor,ignore);
        println!("{}", "-".repeat(94));
        println!("{:<4} {:<14} {:>10} {:>4} {:>5} {:<9} {}", "Rank","Symbol","Close","RS","Score","Rating","Primary Reason");
        println!("{}", "-".repeat(94));
        for (i,s) in scores.iter().take(20).enumerate() {
            let analysis = lookup.get(s.symbol.as_str()).copied();

            let close = analysis
                .and_then(|a| a.latest_close)
                .unwrap_or(0.0);

            let rs_rank = analysis
                .and_then(|a| a.relative_strength_rank)
                .unwrap_or(0);

            let rating=match s.rating{
                Rating::Buy=>"BUY",
                Rating::Watch=>"WATCH",
                Rating::Monitor=>"MONITOR",
                Rating::Ignore=>"IGNORE",
            };
            let reason=s.reasons.first().map(String::as_str).unwrap_or("All conditions satisfied");
            println!("{:<4} {:<14} {:>10.2} {:>4} {:>5} {:<9} {}", i+1, s.symbol, close, rs_rank, s.score, rating, reason);
        }

        println!("{}", "-".repeat(94));

        println!();
        println!("==========================================================================");
        println!("             HIGH-QUALITY NEAR BREAKOUTS (≤ 2%)");
        println!("==========================================================================");
        println!("{:<4} {:<14} {:>10} {:>8} {:>6} {:>5}",
                 "Rank","Symbol","Close","Dist%","ADX","RS");
        println!("{}", "-".repeat(94));
        for (i, nb) in near_breakouts.iter().take(10).enumerate() {
            println!(
                "{:<4} {:<14} {:>10.2} {:>7.2}% {:>6.1} {:>5}",
                i+1,
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
