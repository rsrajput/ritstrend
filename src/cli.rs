use clap::Parser;

/// Command-line arguments for the RitsTrend scanner.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "RitsTrend - an EOD trend-following scanner for NSE500 stocks"
)]
pub struct Cli {
    /// Path to the file containing the list of ticker symbols.
    #[arg(long, default_value = "nse500.txt")]
    pub tickers_file: String,

    /// Directory that contains local CSV price histories.
    #[arg(long, default_value = "data")]
    pub data_dir: String,

    /// Path to the TOML configuration file.
    #[arg(long, default_value = "config.toml")]
    pub config: String,

    /// Directory where reports should be written.
    #[arg(long, default_value = "reports")]
    pub report_dir: String,
}
