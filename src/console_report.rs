//! console_report.rs
//! Console report with ScoreEngine integration.

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
        println!();
        println!("==============================================================");
        println!("                     RITS TREND v0.3");
        println!("==============================================================");
        println!("Stocks Loaded : {}", analyses.len());
        println!("Missing Files : {}", missing_files);
        println!();

        let mut scores = ScoreEngine::score_all(
            analyses,
            rs_threshold,
            volume_factor,
        );

        scores.sort_by(|a, b| b.score.cmp(&a.score));

        println!("{:<5} {:<14} {:>6} {:<9}", "Rank", "Symbol", "Score", "Rating");
        println!("{}", "-".repeat(42));

        for (i, s) in scores.iter().take(20).enumerate() {
            let rating = match s.rating {
                Rating::Buy => "BUY",
                Rating::Watch => "WATCH",
                Rating::Monitor => "MONITOR",
                Rating::Ignore => "IGNORE",
            };

            println!(
                "{:<5} {:<14} {:>6} {:<9}",
                i + 1,
                s.symbol,
                s.score,
                rating
            );
        }

        println!("{}", "-".repeat(42));
    }
}
