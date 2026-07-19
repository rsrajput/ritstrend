use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {

    #[arg(long)]
    pub tickers_file: String,

    #[arg(long)]
    pub data_dir: String,

    #[arg(long, default_value = "config.toml")]
    pub config: String,

    #[arg(long, default_value = "reports")]
    pub report_dir: String,
}