//! report.rs
//! CSV report generation for RitsTrend v0.1

use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::Result;

use crate::analysis::StockAnalysis;
use crate::screener::Signal;

pub struct ReportWriter;

impl ReportWriter {
    pub fn write_reports<P: AsRef<Path>>(
        output_dir: P,
        results: &[(StockAnalysis, Signal)],
    ) -> Result<()> {
        fs::create_dir_all(&output_dir)?;

        let mut buy = Self::writer(output_dir.as_ref().join("BUY.csv"))?;
        let mut cand = Self::writer(output_dir.as_ref().join("CANDIDATES.csv"))?;
        let mut exit = Self::writer(output_dir.as_ref().join("EXIT.csv"))?;
        let mut reject = Self::writer(output_dir.as_ref().join("REJECT.csv"))?;

        Self::header(&mut buy)?;
        Self::header(&mut cand)?;
        Self::header(&mut exit)?;
        Self::header(&mut reject)?;

        for (a, signal) in results {
            let w = match signal {
                Signal::Buy => &mut buy,
                Signal::Candidate => &mut cand,
                Signal::Exit => &mut exit,
                Signal::Reject => &mut reject,
            };

            writeln!(
                w,
                "{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2},{},{:.2}",
                a.symbol,
                a.latest_close.unwrap_or_default(),
                a.sma50.unwrap_or_default(),
                a.sma200.unwrap_or_default(),
                a.adx14.unwrap_or_default(),
                a.return6m.unwrap_or_default(),
                a.relative_strength_score.unwrap_or_default(),
                a.relative_strength_rank.unwrap_or_default(),
                a.average_volume50.unwrap_or_default()
            )?;
        }

        Ok(())
    }

    fn writer(path: impl AsRef<Path>) -> Result<BufWriter<File>> {
        Ok(BufWriter::new(File::create(path)?))
    }

    fn header(w: &mut BufWriter<File>) -> Result<()> {
        writeln!(
            w,
            "Symbol,Close,SMA50,SMA200,ADX14,Return6M,RS Score,RS Rank,AvgVolume50"
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_is_written() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("t.csv");
        let mut w = ReportWriter::writer(&file).unwrap();
        ReportWriter::header(&mut w).unwrap();
        w.flush().unwrap();
        assert!(file.exists());
    }
}
