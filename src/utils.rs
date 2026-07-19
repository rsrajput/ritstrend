use anyhow::Result;

pub fn read_tickers(path: &str) -> Result<Vec<String>> {

    let text = std::fs::read_to_string(path)?;

    Ok(text
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect())
}