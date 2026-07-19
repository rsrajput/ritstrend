use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::ReaderBuilder;

use crate::models::Candle;

/// Load historical OHLCV data from a local CSV file.
pub fn load_history(path: &str) -> Result<Vec<Candle>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)
        .with_context(|| format!("unable to open {}", path))?;

    let mut candles = Vec::new();

    for record in reader.records() {
        let row = record.with_context(|| format!("unable to parse row in {}", path))?;

        if row.len() != 6 {
            anyhow::bail!("expected 6 columns in {}, found {}", path, row.len());
        }

        let date_value = row
            .get(0)
            .with_context(|| format!("missing date column in {}", path))?;
        let open_value = row
            .get(1)
            .with_context(|| format!("missing open column in {}", path))?;
        let high_value = row
            .get(2)
            .with_context(|| format!("missing high column in {}", path))?;
        let low_value = row
            .get(3)
            .with_context(|| format!("missing low column in {}", path))?;
        let close_value = row
            .get(4)
            .with_context(|| format!("missing close column in {}", path))?;
        let volume_value = row
            .get(5)
            .with_context(|| format!("missing volume column in {}", path))?;

        candles.push(Candle {
            date: NaiveDate::parse_from_str(date_value, "%Y-%m-%d")
                .with_context(|| format!("invalid date '{}' in {}", date_value, path))?,
            open: open_value
                .parse::<f64>()
                .with_context(|| format!("invalid open value '{}' in {}", open_value, path))?,
            high: high_value
                .parse::<f64>()
                .with_context(|| format!("invalid high value '{}' in {}", high_value, path))?,
            low: low_value
                .parse::<f64>()
                .with_context(|| format!("invalid low value '{}' in {}", low_value, path))?,
            close: close_value
                .parse::<f64>()
                .with_context(|| format!("invalid close value '{}' in {}", close_value, path))?,
            volume: volume_value
                .parse::<f64>()
                .with_context(|| format!("invalid volume value '{}' in {}", volume_value, path))?,
        });
    }

    Ok(candles)
}
