//! console_report.rs
use crate::analysis::StockAnalysis;

pub struct ConsoleReport;

impl ConsoleReport {
    pub fn print(analyses: &[StockAnalysis], missing_files: usize) {
        println!();
        println!("==============================================================");
        println!("                     RITS TREND v0.2");
        println!("==============================================================");
        println!("Stocks Loaded : {}", analyses.len());
        println!("Missing Files : {}", missing_files);
        println!();
        println!("{:<5} {:<14} {:>10} {:>10} {:>10} {:>8} {:>6}",
                 "Rank","Symbol","Close","SMA50","SMA200","ADX","RS");
        println!("{}", "-".repeat(74));
        for (i,a) in analyses.iter().take(20).enumerate() {
            println!("{:<5} {:<14} {:>10.2} {:>10.2} {:>10.2} {:>8.1} {:>6}",
                i+1,a.symbol,a.latest_close.unwrap_or_default(),
                a.sma50.unwrap_or_default(),a.sma200.unwrap_or_default(),
                a.adx14.unwrap_or_default(),
                a.relative_strength_rank.unwrap_or(0));
        }
        println!("{}", "-".repeat(74));
    }
}
