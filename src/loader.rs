use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::Reader;

use crate::models::Candle;

pub fn load_history(path: &str) -> Result<Vec<Candle>> {

    let mut rdr = Reader::from_path(path)
        .with_context(|| format!("Unable to open {}", path))?;

    let mut candles = Vec::new();

    for record in rdr.records() {

        let row = record?;

        candles.push(Candle {

            date: NaiveDate::parse_from_str(
                &row[0],
                "%Y-%m-%d",
            )?,

            open: row[1].parse()?,

            high: row[2].parse()?,

            low: row[3].parse()?,

            close: row[4].parse()?,

            volume: row[5].parse()?,
        });

    }

    Ok(candles)
}