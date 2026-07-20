#!/usr/bin/env python3
"""
RitsTrend Market Data Downloader v0.1

Downloads daily OHLCV data from Yahoo Finance for NSE stocks.

Example:
    python update_data.py --tickers-file nse500.txt --output-dir data --period 5y
"""

from __future__ import annotations

import argparse
import concurrent.futures as futures
import logging
from pathlib import Path

import pandas as pd
import yfinance as yf
from tqdm import tqdm


def parse_args():
    p = argparse.ArgumentParser(description="Download NSE EOD data")
    p.add_argument("--tickers-file", required=True)
    p.add_argument("--output-dir", default="data")
    p.add_argument("--period", default="5y",
                   choices=["1y", "2y", "5y", "10y", "max"])
    p.add_argument("--threads", type=int, default=8)
    return p.parse_args()


def load_symbols(path: Path):
    return [
        line.strip().upper()
        for line in path.read_text().splitlines()
        if line.strip() and not line.startswith("#")
    ]


def download(symbol: str, outdir: Path, period: str):
    yf_symbol = f"{symbol}.NS"
    try:
        df = yf.download(
            yf_symbol,
            period=period,
            interval="1d",
            auto_adjust=False,
            progress=False,
            threads=False,
        )

        if df.empty:
            return symbol, False, "No data"

        # Flatten MultiIndex columns returned by newer yfinance versions.
        if isinstance(df.columns, pd.MultiIndex):
            df.columns = df.columns.get_level_values(0)

        df = df.reset_index()

        keep = ["Date", "Open", "High", "Low", "Close", "Volume"]
        df = df[keep]

        df["Date"] = pd.to_datetime(df["Date"]).dt.strftime("%Y-%m-%d")

        outfile = outdir / f"{symbol}.csv"
        df.to_csv(outfile, index=False)

        return symbol, True, ""
    except Exception as ex:
        return symbol, False, str(ex)


def main():
    args = parse_args()

    outdir = Path(args.output_dir)
    outdir.mkdir(parents=True, exist_ok=True)

    Path("logs").mkdir(exist_ok=True)

    logging.basicConfig(
        filename="logs/update.log",
        level=logging.INFO,
        format="%(asctime)s %(levelname)s %(message)s",
    )

    symbols = load_symbols(Path(args.tickers_file))

    ok = 0
    failed = []

    with futures.ThreadPoolExecutor(max_workers=args.threads) as executor:
        jobs = [
            executor.submit(download, s, outdir, args.period)
            for s in symbols
        ]

        for job in tqdm(futures.as_completed(jobs), total=len(jobs),
                        desc="Downloading"):
            symbol, success, message = job.result()

            if success:
                ok += 1
                logging.info("%s OK", symbol)
            else:
                failed.append(symbol)
                logging.error("%s FAILED %s", symbol, message)

    print(f"\nDownloaded : {ok}")
    print(f"Failed     : {len(failed)}")

    if failed:
        failed_file = Path("logs/failed.txt")
        failed_file.write_text("\n".join(failed))
        print(f"See {failed_file}")


if __name__ == "__main__":
    main()
