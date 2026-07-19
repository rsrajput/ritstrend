mod cli;
mod config;
mod loader;
mod models;
mod utils;

use anyhow::Result;
use clap::Parser;

use cli::Cli;
use config::Config;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::load(&cli.config)?;

    let tickers = utils::read_tickers(&cli.tickers_file)?;

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

    Ok(())
}