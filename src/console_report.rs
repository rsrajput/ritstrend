//! console_report.rs
//!
//! Pretty console reporting for RitsTrend.
//! This module is independent of the existing report.rs.

use crate::analysis::StockAnalysis;

pub struct ConsoleReport;

impl ConsoleReport {
    pub fn print(analyses: &[StockAnalysis]) {
        println!();
        println!("==========================================================================");
        println!("                            RITS TREND v1.0");
        println!("==========================================================================");
        println!(
            "{:<4} {:<15} {:>10} {:>6} {:>8} {:>10}",
            "Rank", "Symbol", "Close", "RS", "ADX", "Ret6M%"
        );
        println!("--------------------------------------------------------------------------");

        for (i, a) in analyses.iter().enumerate().take(25) {
            println!(
                "{:<4} {:<15} {:>10.2} {:>6} {:>8.1} {:>10.1}",
                i + 1,
                a.symbol,
                a.latest_close.unwrap_or_default(),
                a.relative_strength_rank.unwrap_or(0),
                a.adx14.unwrap_or_default(),
                a.return6m.unwrap_or_default() * 100.0,
            );
        }

        println!("--------------------------------------------------------------------------");
        println!("Stocks analysed : {}", analyses.len());
        println!("==========================================================================");
    }
}
