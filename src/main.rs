mod adx;
mod cli;
mod config;
mod indicators;
mod loader;
mod models;
mod portfolio;
mod price_series;
mod ranking;
mod report;
mod screener;
mod utils;

use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::Cli;
use crate::config::Config;
use crate::loader::load_history;
use crate::models::{IndicatorSnapshot, Signal};
use crate::price_series::PriceSeries;

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
    let mut loaded_files = 0usize;

    if data_dir.exists() {
        for entry in std::fs::read_dir(data_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(std::ffi::OsStr::to_str) == Some("csv") {
                let csv_path = path.to_string_lossy().into_owned();
                let candles = load_history(&csv_path)
                    .with_context(|| format!("unable to read candles from {}", csv_path))?;

                let symbol = path
                    .file_stem()
                    .and_then(std::ffi::OsStr::to_str)
                    .map(str::to_string)
                    .unwrap_or_else(|| String::from("unknown"));

                let _series = PriceSeries::new(symbol, candles)
                    .with_context(|| format!("unable to build price series for {}", csv_path))?;
                loaded_files += 1;
            }
        }
    } else {
        println!("No data directory found at {}", cli.data_dir);
    }

    let _snapshot = IndicatorSnapshot::default();
    let _signal = Signal::Watch;

    if let Some(first_series) = std::iter::empty::<PriceSeries>().next() {
        let _ = first_series.symbol();
    }

    println!("CSV files loaded : {}", loaded_files);
    println!("Scan setup complete. Add data files to run the pipeline.");

    Ok(())
}
