mod adx;
mod analysis;
mod cli;
mod config;
mod console_report;
mod indicator_engine;
mod indicators;
mod loader;
mod models;
mod portfolio;
mod price_series;
mod ranking;
mod report;
mod score_engine;
mod screener;
mod utils;
mod wilder;

use anyhow::{Context, Result};
use clap::Parser;

use crate::analysis::StockAnalysisBuilder;
use crate::cli::Cli;
use crate::config::Config;
use crate::indicator_engine::IndicatorEngine;
use crate::loader::load_history;
use crate::price_series::PriceSeries;
use crate::ranking::RankingEngine;
use crate::report::ReportGenerator;
use crate::screener::Screener;
use crate::console_report::ConsoleReport;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::load(&cli.config)
        .with_context(|| format!("unable to load configuration from {}", cli.config))?;

    let tickers = utils::read_tickers(&cli.tickers_file)
        .with_context(|| format!("unable to read tickers from {}", cli.tickers_file))?;

    println!("========================================");
    println!("           RitsTrend v0.1");
    println!("========================================");
    println!("Tickers loaded : {}", tickers.len());
    println!("Data directory : {}", cli.data_dir);
    println!("Breakout       : {}", config.breakout_period);
    println!("Exit           : {}", config.exit_period);
    println!("ATR            : {}", config.atr_period);
    println!("ADX            : {}", config.adx_period);
    println!("========================================");

    let data_dir = std::path::Path::new(&cli.data_dir);
    let mut analyses = Vec::new();
    let mut loaded_files = 0usize;

    if data_dir.exists() {
        for ticker in &tickers {
            let csv_path = data_dir.join(format!("{ticker}.csv"));
            if !csv_path.exists() {
                println!("Skipping {} (missing {})", ticker, csv_path.display());
                continue;
            }

            let csv_path_str = csv_path.to_string_lossy().into_owned();
            let candles = load_history(&csv_path_str)
                .with_context(|| format!("unable to read candles from {}", csv_path_str))?;

            let series = PriceSeries::new(ticker.as_str(), candles)
                .with_context(|| format!("unable to build price series for {}", csv_path_str))?;

            let mut builder = StockAnalysisBuilder::new();
            IndicatorEngine::analyze(&series, &mut builder)
                .with_context(|| format!("unable to analyze {}", ticker))?;

            analyses.push(builder.build());
            loaded_files += 1;
        }
    } else {
        println!("No data directory found at {}", cli.data_dir);
    }

    RankingEngine::rank(&mut analyses);
    println!("\n========== FIRST 10 ANALYSES ==========");

    for analysis in analyses.iter().take(10) {
        println!("{:#?}", analysis);
    }

    let missing_files = tickers.len().saturating_sub(analyses.len());
    ConsoleReport::print(
    &analyses,
    missing_files,
    config.top_percent,
    config.volume_factor,
    );

    let buys = Screener::screen(&analyses, config.top_percent, config.volume_factor);

    let report_dir = std::path::Path::new(&cli.report_dir);
    ReportGenerator::write(&buys, report_dir, "BUY.csv")?;

    println!("CSV files loaded : {}", loaded_files);
    println!("BUY candidates   : {}", buys.len());
    println!("Report written  : {}/BUY.csv", report_dir.display());

    Ok(())
}
