# RitsTrend Development Guide

## Project Overview

RitsTrend is a Rust CLI application for end-of-day trend-following stock screening.

The project is actively developed on feature branches and uses Git for version control.

---

## Development Principles

- Preserve the existing architecture.
- Modify only the files required for the requested task.
- Do not rename modules or files unless explicitly requested.
- Do not introduce unnecessary abstractions.
- Keep code idiomatic Rust.
- Keep functions concise and readable.
- Prefer extending existing code over creating new modules.
- Reuse existing calculations whenever possible.

---

## Trading System

### Entry

- Close above 55-day Donchian High

### Exit

- Close below 20-day Donchian Low

### Initial Stop

- Entry − ATR(15) × atr_stop_multiplier

### Filters

- Close > 200 SMA
- 50 SMA > 200 SMA
- ADX above configured threshold
- Relative Strength Rank
- Volume confirmation

---

## Configuration

Always use values from `config.toml`.

Never hardcode configurable values.

Examples include:

- atr_stop_multiplier
- volume_factor
- ADX threshold
- top_percent

---

## Before Modifying Code

Before writing code:

1. Read the affected modules.
2. Understand how they interact.
3. Modify only the required files.
4. Preserve backward compatibility whenever possible.

---

## Before Finishing Any Task

Always:

1. Run `cargo fmt`
2. Run `cargo check`
3. Fix all compilation errors.
4. Summarize the modified files.

Never finish a task with compilation errors.

---

## Git

Never commit automatically.

Never push automatically.

Wait for user confirmation before creating commits.

---

## Communication Style

When implementing a feature:

- Explain the approach briefly.
- List modified files.
- Mention any assumptions.
- Ask before making architectural changes.

Do not redesign the project unless explicitly requested.

## Before finishing

Always execute:

git diff

Show the complete diff.

Never finish silently.

Modify the minimum number of files required.

If additional files are needed, explain why before changing them.

## Terminal Output Philosophy

RitsTrend is a terminal-first application.

Do not generate:

- CSV
- HTML
- JSON
- Reports

unless explicitly requested.

The console output is the product.

Optimize for making trading decisions within 30 seconds after market close.