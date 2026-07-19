# RitsTrend Architecture

## Overview

RitsTrend is a professional End-of-Day (EOD) trend-following scanner for Indian stocks. It is designed for the NSE500 universe and focuses on robust, deterministic analysis of historical OHLCV data. The application is intentionally built for daily, non-intraday analysis and does not perform market data downloads itself.

RitsTrend follows a strict pipeline architecture:

```text
CSV Loader
  ↓
PriceSeries
  ↓
Indicator Engine
  ↓
Relative Strength Ranking
  ↓
Screening Engine
  ↓
Report Generator
  ↓
Portfolio
```

The system is modular, one-way in data flow, and avoids circular dependencies.

---

## Purpose and Scope

### Primary goals

- Read historical OHLCV data from local CSV files
- Calculate trend-following indicators
- Rank stocks by Relative Strength
- Generate BUY, WATCH, EXIT, and REJECT reports
- Support fast scanning over a large universe of stocks

### Non-goals

- Intraday trading
- Live market data ingestion
- Automatic data download from within Rust
- Complex portfolio optimization

### Data ownership model

- Python (via yfinance) is responsible for downloading market data
- Rust is responsible only for reading local CSV files and analyzing them

This separation keeps the Rust application deterministic, simple, and easy to test.

---

## Design Principles

RitsTrend is built around the following principles:

1. Modular architecture
   - Each module has a single responsibility.
   - Dependencies flow in one direction only.

2. Strong typing and explicit data models
   - Candles, price series, signals, and reports are represented as typed domain objects.

3. No unsafe code
   - The implementation must remain memory-safe and idiomatic.

4. Error handling with anyhow
   - Production code uses structured error propagation instead of panics.

5. Performance-oriented scanning
   - The architecture is designed to scan approximately 500 stocks with about 300 candles each in under one second of compute time, excluding disk I/O.

6. Readability and maintainability
   - Public APIs are documented.
   - The codebase is aligned with Rust 2021 conventions.

---

## High-Level Architecture

### Runtime flow

1. The CLI parses configuration and input arguments.
2. The configuration loader reads settings such as thresholds and output paths.
3. The loader reads one CSV file per symbol.
4. The price series layer validates and prepares historical data.
5. The indicator engine computes trend and momentum signals.
6. The ranking layer evaluates Relative Strength across the universe.
7. The screener applies trading rules and emits signal results.
8. The report generator writes BUY, WATCH, EXIT, and REJECT outputs.
9. The portfolio module can later manage trade state and positions.

### Architectural direction

The system should be arranged so that each stage depends only on the stage before it. No stage should need to reach backward into a later stage.

---

## Repository Structure

```text
ritstrend/
├── Cargo.toml
├── config.toml
├── update_data.py
├── README.md
├── docs/
│   └── ARCHITECTURE.md
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── config.rs
│   ├── models.rs
│   ├── loader.rs
│   ├── price_series.rs
│   ├── indicators.rs
│   ├── adx.rs
│   ├── ranking.rs
│   ├── screener.rs
│   ├── report.rs
│   ├── portfolio.rs
│   └── utils.rs
├── data/
├── reports/
├── tests/
└── target/
```

---

## Module Responsibilities

### main.rs

The composition root of the application.

Responsibilities:
- Parse CLI arguments
- Load configuration
- Initialize the scan pipeline
- Invoke report generation

It should remain thin and orchestrate the flow rather than perform business logic directly.

### cli.rs

Handles command-line interaction.

Responsibilities:
- Define CLI flags and subcommands
- Parse user-configurable paths and options
- Provide a clear interface for running scans

### config.rs

Contains runtime configuration and threshold settings.

Responsibilities:
- Load configuration from TOML
- Expose values such as lookback windows, ATR settings, and output paths
- Provide defaults for all parameters

### models.rs

Defines the core domain model.

Responsibilities:
- Represent a candle as OHLCV data
- Define scan results and signal records
- Model output reports and metadata

This file should house the shared domain types that other modules depend on.

### loader.rs

Responsible for ingesting historical data from CSV.

Responsibilities:
- Open CSV files from disk
- Parse rows into Candle records
- Validate structure and data integrity
- Return typed results with meaningful errors

The loader must not perform indicator computation or ranking.

### price_series.rs

Transforms raw candles into a usable price-series representation.

Responsibilities:
- Ensure chronological ordering
- Normalize or validate missing values
- Prepare rolling-window access patterns
- Expose helper methods for lookbacks and sliding windows

This module ensures downstream analysis operates on consistent time-series data.

### indicators.rs

The generic indicator layer.

Responsibilities:
- Implement reusable rolling calculations
- Expose helper functions for moving averages and basic statistics
- Serve as the shared foundation for more specialized indicators

### adx.rs

Contains the ADX-specific implementation details.

Responsibilities:
- Compute ATR (Wilder)
- Compute DX
- Compute ADX

The ADX engine should remain isolated so the broader indicator engine can remain focused on reusable components.

### ranking.rs

Performs Relative Strength ranking across the selected universe.

Responsibilities:
- Calculate relative strength values
- Rank securities against one another
- Produce an ordered list suitable for screening

This module should be independent of reporting output.

### screener.rs

Applies the trading rules to each symbol.

Responsibilities:
- Evaluate all conditions for BUY, WATCH, EXIT, and REJECT
- Produce one signal result per stock
- Keep screening rules centralized and testable

### report.rs

Responsible for rendering the final output.

Responsibilities:
- Format BUY/WATCH/EXIT/REJECT reports
- Write text or CSV outputs to disk
- Present results in a consistent and professional layout

### portfolio.rs

Planned portfolio-management layer.

Responsibilities:
- Track positions and trade state
- Manage stop-loss logic and exits
- Support future portfolio-oriented features

This module is intentionally separated so that the core scanner can evolve independently.

### utils.rs

Shared utility functions.

Responsibilities:
- File and path helpers
- Parsing helpers
- Common formatting or conversion logic

---

## Data Flow

The pipeline is intentionally linear.

```text
Raw CSV
  → Parsed candles
  → Price series preparation
  → Indicator computation
  → Relative Strength ranking
  → Signal generation
  → Report rendering
```

### Invariant

No module should depend on a downstream module. The only permitted direction is forward through the pipeline.

That means:
- loader does not depend on screener
- ranking does not depend on report generation
- report generation does not affect indicator computation

---

## Core Data Model

### Candle

Each candle represents one trading day.

```rust
struct Candle {
    date: NaiveDate,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}
```

### PriceSeries

A price series is a chronological collection of candles with helper methods for lookback access.

### Signal

A signal captures the output of the screening engine.

Example fields:
- symbol
- decision: BUY, WATCH, EXIT, REJECT
- reasons: vector of rule explanations
- entry_price
- stop_price
- strength_rank

---

## Indicator Design

### Implemented indicators

The architecture explicitly supports the following indicators:

- SMA
- ATR (Wilder)
- ADX
- Donchian High
- Donchian Low
- Average Volume
- 6-month return
- 12-month return

### Indicator strategy

Indicators should be computed from a consistent price-series object so that the same data is used throughout the pipeline.

The implementation should favor:
- small, composable functions
- predictable time complexity
- clear naming that maps directly to trading logic

---

## Trading Rule Model

The screener should evaluate a deterministic set of conditions.

### BUY

A BUY signal is generated when:
- Close > Previous 55-day High
- Close > SMA200
- SMA50 > SMA200
- ADX14 > 25
- Volume > 1.5 × Average Volume50
- Stock ranks in the top 25% by Relative Strength

### Initial Stop

- Entry − 3 × ATR15

### EXIT

An EXIT signal is generated when:
- Close < Previous 20-day Donchian Low

### WATCH and REJECT

- WATCH is used for near-miss conditions or intermediate strength
- REJECT is used for stocks that fail one or more critical rules

### Rule separation

Each rule should be expressed as a named function or small evaluation step. This improves testability and makes it easier to update rules over time.

---

## Error Handling and Validation

### Error policy

- Use anyhow::Result for propagating recoverable errors
- Avoid unwrap() in production code
- Surface clear context when CSV parsing or configuration loading fails

### Validation rules

The loader and price-series layer should validate:
- required columns are present
- dates are parseable and sorted
- numeric values are finite
- volume and price values are non-negative

---

## Configuration Model

Configuration should be loaded from a TOML file such as config.toml.

Suggested settings:
- breakout period
- exit period
- ATR period
- ADX period
- SMA periods
- minimum volume multiplier
- output directory paths

Configuration values should be explicit and documented so new contributors can adjust the scanner safely.

---

## Performance Considerations

RitsTrend is designed for large-scale scanning.

### Performance targets

- Scan 500 stocks
- Approximately 300 candles per stock
- Complete scan in under one second excluding disk I/O

### Design choices to support this

- Use iterators and simple numeric operations
- Avoid unnecessary allocations inside hot loops
- Prefer batch processing where possible
- Consider rayon for parallel scanning once the single-threaded path is stable

Because disk I/O dominates this workflow, the architecture should keep compute lightweight and predictable.

---

## Testing Strategy

The project should follow a layered testing approach.

### Unit tests

Cover the following:
- CSV parsing
- Indicator calculations
- ADX and ATR logic
- Ranking behavior
- Screening rules

### Integration tests

Verify end-to-end behavior with representative fixture data.

### Testing standards

- Use cargo test for automated verification
- Use cargo fmt for formatting
- Use cargo clippy for linting
- Keep tests deterministic and fixture-driven

---

## Coding Standards

The project should follow these standards:

- Rust 2021
- No unsafe code
- No unwrap() in production code
- Use anyhow for errors
- Use serde where appropriate
- Public APIs must have documentation comments
- Prefer small, focused modules over large monolithic files

---

## Future Roadmap

The architecture is intentionally extensible.

### Planned enhancements

- Portfolio management
- Backtesting
- HTML dashboard
- SQLite persistence
- Telegram notifications
- Email reports
- Zerodha Kite integration

### Architectural implication

These features should be added as separate layers rather than merged into the scanner core. This keeps the core pipeline stable and testable while supporting future product growth.

---

## Summary

RitsTrend is designed as a clean, modular, and professional Rust application for EOD trend-following analysis. Its architecture emphasizes deterministic data processing, clear responsibility boundaries, and a simple pipeline from CSV input to actionable trading reports.

The design is intentionally straightforward: read local data, compute indicators, rank strength, score opportunities, and publish results. That makes the project easy to understand, maintain, and extend over time.
