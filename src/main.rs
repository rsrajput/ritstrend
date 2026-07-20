//! main.rs
//! RitsTrend v0.1 entry point

use anyhow::Result;

use ritstrend::{
    analysis::StockAnalysisBuilder,
    indicator_engine::IndicatorEngine,
    loader,
    ranking::RelativeStrengthRanker,
    report::ReportWriter,
    screener::{ScreenerConfig, Signal, TurtleScreener},
};

fn main() -> Result<()> {
    // TODO: replace with your existing CLI/config parsing.
    let data_dir = "data";
    let output_dir = "reports";

    println!("RitsTrend v0.1");
    println!("Loading market data...");

    // Expected to return Vec<PriceSeries>
    let market = loader::load_market(data_dir)?;

    let mut analyses = Vec::new();

    for series in &market {
        let mut builder = StockAnalysisBuilder::new();

        builder
            .symbol(series.symbol().to_string());

        IndicatorEngine::analyze(series, &mut builder)?;

        analyses.push(builder.build());
    }

    RelativeStrengthRanker::rank(&mut analyses);

    let total_ranked = analyses
        .iter()
        .filter(|a| a.relative_strength_rank.is_some())
        .count();

    let cfg = ScreenerConfig::default();

    let mut results = Vec::with_capacity(analyses.len());

    for analysis in analyses {
        let signal = TurtleScreener::evaluate(
            &analysis,
            total_ranked,
            &cfg,
        );

        results.push((analysis, signal));
    }

    ReportWriter::write_reports(output_dir, &results)?;

    let buy = results.iter().filter(|(_, s)| *s == Signal::Buy).count();
    let cand = results.iter().filter(|(_, s)| *s == Signal::Candidate).count();
    let exit = results.iter().filter(|(_, s)| *s == Signal::Exit).count();

    println!();
    println!("Scan completed.");
    println!("BUY        : {}", buy);
    println!("CANDIDATES : {}", cand);
    println!("EXIT       : {}", exit);
    println!("Reports written to '{}'", output_dir);

    Ok(())
}
