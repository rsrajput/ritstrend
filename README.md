# RitsTrend

A terminal-first End-of-Day trend following dashboard for NSE stocks.

## Daily Workflow

### 1. Activate the Python virtual environment

**Linux / macOS**

``` bash
source .venv/bin/activate
```

**Windows (PowerShell)**

``` powershell
.venv\Scripts\Activate.ps1
```

------------------------------------------------------------------------

### 2. Update daily market data

Run after the market closes.

``` bash
python update_data.py \
    --tickers-file tickers/nse500.txt \
    --output-dir data \
    --period 5y
```

This downloads the latest EOD data for all configured stocks.

------------------------------------------------------------------------

### 3. Run RitsTrend

``` bash
cargo run -- \
    --tickers-file tickers/nse500.txt \
    --data-dir data \
    --report-dir reports
```

------------------------------------------------------------------------

# Understanding the Dashboard

Read the sections in this order.

## 1. Scan Summary

This summarizes the screening run.

Typical information:

-   Stocks loaded
-   Missing files
-   BUY count
-   WATCH count
-   MONITOR count
-   IGNORE count
-   Strongest stock

This is **not** market health. It simply summarizes today's scan.

------------------------------------------------------------------------

## 2. Top Opportunities

This is your primary decision table.

Columns:

  Column           Meaning
  ---------------- --------------------------------------------
  Rank             Highest score first
  Symbol           Stock symbol
  Close            Latest closing price
  RS               Relative Strength Rank (lower is stronger)
  Score            Overall technical quality
  Rating           BUY / WATCH / MONITOR / IGNORE
  Primary Reason   Main reason affecting the rating

### How to use it

This answers:

> Which stocks deserve my attention today?

Priority:

1.  BUY
2.  Highest Score
3.  Better RS Rank

Use this as your shortlist.

------------------------------------------------------------------------

## 3. High-Quality Near Breakouts

Shows stocks that are close to a valid breakout but have not broken out
yet.

Columns:

  Column   Meaning
  -------- ------------------------
  Close    Latest close
  Dist%    Distance from breakout
  ADX      Trend strength
  RS       Relative Strength Rank

### How to use it

This answers:

> Which stocks are likely to become BUY candidates soon?

These are **watchlist** stocks.

Do not buy merely because they appear here.

Monitor them for confirmation.

------------------------------------------------------------------------

## 4. Breakout Diagnostics

Example:

``` text
SONACOMS   Trend:Y MA:Y ADX:Y Vol:Y RS:Y
```

Each flag represents one rule.

### Trend

Y = Close above the 55-day Donchian breakout.

### MA

Y =

-   Close above 200 DMA
-   50 DMA above 200 DMA

### ADX

Y = ADX above the configured threshold.

### Volume

Y = Volume above the configured multiplier.

### RS

Y = Relative Strength rank inside the configured threshold.

### Close

Latest closing price.

### Don

55-day breakout level.

------------------------------------------------------------------------

# Trading Workflow

Every evening:

1.  Update market data.
2.  Run RitsTrend.
3.  Review **Top Opportunities**.
4.  Check **High-Quality Near Breakouts** to prepare tomorrow's
    watchlist.
5.  Use **Breakout Diagnostics** to understand why a stock qualified or
    failed.

------------------------------------------------------------------------

# Interpreting Score vs RS

## Score

Measures the overall quality of the setup.

Higher is better.

## RS Rank

Measures leadership versus the rest of the market.

Lower is better.

When selecting between two BUY candidates:

1.  Prefer the higher Score.
2.  If scores are similar, prefer the lower RS Rank.

------------------------------------------------------------------------

# Future Vision

The planned dashboard will also include a true Market Health section
based on market breadth:

-   Percentage above 200 DMA
-   Percentage above 50 DMA
-   ADX participation
-   Breakout participation
-   Market verdict

# Rits Terminal commands
1.

cd ~/rits_rust_donchian_screener/target/release && ./donchian_engine --tickers-file nse500.txt


2.

cd ~/Documents/ritstrend && source .venv/bin/activate
python update_data.py --tickers-file tickers/nse500.txt --output-dir data --period 5y
RUSTFLAGS="-Awarnings" cargo run -- --tickers-file tickers/nse500.txt --data-dir data --report-dir reports

cd ~/Documents/ritstrend && source .venv/bin/activate
python update_data.py --tickers-file tickers/midcap150.txt --output-dir data --period 5y
RUSTFLAGS="-Awarnings" cargo run -- --tickers-file tickers/midcap150.txt --data-dir data --report-dir reports

cd ~/Documents/ritstrend && source .venv/bin/activate
python update_data.py --tickers-file tickers/smallcap500.txt --output-dir data --period 5y
RUSTFLAGS="-Awarnings" cargo run -- --tickers-file tickers/smallcap500.txt --data-dir data --report-dir reports

cd ~/Documents/ritstrend && source .venv/bin/activate
RUSTFLAGS="-Awarnings" cargo run -- --tickers-file tickers/nse500.txt --data-dir data --report-dir reports

cd ~/Documents/ritstrend && source .venv/bin/activate
RUSTFLAGS="-Awarnings" cargo run -- --tickers-file tickers/midcap150.txt --data-dir data --report-dir reports

cd ~/Documents/ritstrend && source .venv/bin/activate
RUSTFLAGS="-Awarnings" cargo run -- --tickers-file tickers/smallcap500.txt --data-dir data --report-dir reports



cd Documents/ritstrend
source .venv/bin/activate

python update_data.py \
    --tickers-file tickers/nse500.txt \
    --output-dir data \
    --period 5y


cargo run -- \
    --tickers-file tickers/nse500.txt \
    --data-dir data \
    --report-dir reports

