use anyhow::{Context, Result};

/// Read the ticker list from a text file.
pub fn read_tickers(path: &str) -> Result<Vec<String>> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("unable to read ticker file {}", path))?;

    Ok(text
        .lines()
        .map(str::trim)
        .filter(|symbol| !symbol.is_empty())
        .map(String::from)
        .collect())
}
