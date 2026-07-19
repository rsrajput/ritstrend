mod cli;
mod config;
mod loader;
mod models;
mod utils;

use clap::Parser;

fn main() -> anyhow::Result<()> {

    let cli = cli::Cli::parse();

    let cfg = config::Config::load(&cli.config)?;

    let tickers = utils::read_tickers(&cli.tickers_file)?;

    println!("======================================");
    println!(" RitsTrend");
    println!("======================================");
    println!("Tickers      : {}", tickers.len());
    println!("Data Folder  : {}", cli.data_dir);
    println!("Breakout     : {}", cfg.breakout_period);
    println!("ATR Period   : {}", cfg.atr_period);
    println!("ADX Period   : {}", cfg.adx_period);

    Ok(())
}