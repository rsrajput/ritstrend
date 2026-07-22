use anyhow::Result;
use csv::WriterBuilder;
use std::fs;
use std::path::Path;

use crate::analysis::StockAnalysis;

/// Report generation for BUY candidates.
#[derive(Debug, Default)]
pub struct ReportGenerator;

impl ReportGenerator {
    /// Write buy candidates to a CSV report in the target directory.
    pub fn write(
        analyses: &[StockAnalysis],
        report_dir: impl AsRef<Path>,
        filename: &str,
    ) -> Result<()> {
        let report_dir = report_dir.as_ref();
        fs::create_dir_all(report_dir)?;

        let path = report_dir.join(filename);
        let mut writer = WriterBuilder::new().has_headers(true).from_path(&path)?;

        writer.write_record([
            "Symbol", "Date", "Close", "SMA50", "SMA200", "ADX14", "Return6M", "RS Rank",
            "RS Score",
        ])?;

        for analysis in analyses {
            let date = analysis
                .latest_date
                .map(|value| value.to_string())
                .unwrap_or_default();
            let close = analysis
                .latest_close
                .map(|value| format!("{value:.2}"))
                .unwrap_or_default();
            let sma50 = analysis
                .sma50
                .map(|value| format!("{value:.2}"))
                .unwrap_or_default();
            let sma200 = analysis
                .sma200
                .map(|value| format!("{value:.2}"))
                .unwrap_or_default();
            let adx14 = analysis
                .adx14
                .map(|value| format!("{value:.2}"))
                .unwrap_or_default();
            let return6m = analysis
                .return6m
                .map(|value| format!("{value:.4}"))
                .unwrap_or_default();
            let rs_rank = analysis
                .relative_strength_rank
                .map(|value| value.to_string())
                .unwrap_or_default();
            let rs_score = analysis
                .relative_strength_score
                .map(|value| format!("{value:.4}"))
                .unwrap_or_default();

            writer.write_record([
                analysis.symbol.clone(),
                date,
                close,
                sma50,
                sma200,
                adx14,
                return6m,
                rs_rank,
                rs_score,
            ])?;
        }

        writer.flush()?;
        Ok(())
    }
}
