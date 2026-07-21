//! console_report.rs
use crate::analysis::StockAnalysis;
use crate::score_engine::{Rating, ScoreEngine};

pub struct ConsoleReport;

impl ConsoleReport {
    pub fn print(
        analyses: &[StockAnalysis],
        missing_files: usize,
        rs_threshold: usize,
        volume_factor: f64,
    ) {
        let mut scores = ScoreEngine::score_all(analyses, rs_threshold, volume_factor);
        scores.sort_by(|a,b| b.score.cmp(&a.score));

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
        println!("{}", "-".repeat(78));
        println!("{:<4} {:<14} {:>5} {:<9} {}", "Rank","Symbol","Score","Rating","Primary Reason");
        println!("{}", "-".repeat(78));
        for (i,s) in scores.iter().take(20).enumerate() {
            let rating=match s.rating{
                Rating::Buy=>"BUY",
                Rating::Watch=>"WATCH",
                Rating::Monitor=>"MONITOR",
                Rating::Ignore=>"IGNORE",
            };
            let reason=s.reasons.first().map(String::as_str).unwrap_or("All conditions satisfied");
            println!("{:<4} {:<14} {:>5} {:<9} {}",i+1,s.symbol,s.score,rating,reason);
        }
        println!("{}", "-".repeat(78));
    }
}
