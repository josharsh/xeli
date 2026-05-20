# xeli

**Excel for the terminal** — an interactive TUI spreadsheet with natural language queries.

[![release](https://img.shields.io/github/v/release/josharsh/xeli?color=cba6f7)](https://github.com/josharsh/xeli/releases/latest)
[![npm](https://img.shields.io/npm/v/%40josharsh%2Fxeli?color=a6e3a1)](https://www.npmjs.com/package/@josharsh/xeli)
[![license](https://img.shields.io/github/license/josharsh/xeli?color=f9e2af)](LICENSE)
[![CI](https://img.shields.io/github/actions/workflow/status/josharsh/xeli/release.yml?branch=master&label=release)](https://github.com/josharsh/xeli/actions/workflows/release.yml)
[![stars](https://img.shields.io/github/stars/josharsh/xeli?style=flat&color=f5e0dc)](https://github.com/josharsh/xeli/stargazers)

Open CSV, JSON, Parquet, and Excel files in a beautiful TUI spreadsheet. Query data with natural language (AI → SQL), filter / sort / pivot visually, and export results — all from your terminal.

```
┌─ xeli  CSV  employees.csv  │ 30r × 7c ────────────────────────────────────────┐
│ R1/30   C3/7   ● Dracula                                                      │
│        id    name           city            department    salary  start_date  │
│      ▶ 1     Alice Johnson  New York        Engineering   95000   2020-03-15  │
│        2     Bob Smith      San Francisco   Marketing     78000   2019-07-22  │
│        3     Carol Williams Chicago         Engineering   105000  2018-01-10  │
│        ...                                                                     │
│                                                                                │
│  AI ▌ show me top 5 cities by total revenue for active customers_              │
│                                                                                │
│  NORMAL   R1/30  C3/7  Dracula                                                 │
│  Ctrl+K AI · Ctrl+Q SQL · / find · f filter · s sort · g group · ? help · q   │
└────────────────────────────────────────────────────────────────────────────────┘
```

## Try it without installing

→ **[xeli.vercel.app/playground](https://xeli.vercel.app/playground)** — DuckDB-WASM in your browser. Drop a CSV. Ask a question. Nothing leaves the tab.

## Install

```bash
# npm (no install — runs once)
npx @josharsh/xeli examples/employees.csv

# npm (global)
npm install -g @josharsh/xeli

# Homebrew
brew install josharsh/tap/xeli
```

Or grab a [prebuilt binary](https://github.com/josharsh/xeli/releases/latest).

Supported: **macOS** (Apple Silicon & Intel) and **Linux** (arm64 & x86_64). Windows isn't supported yet — see [#help-wanted](CONTRIBUTING.md#open-help-wanted).

## Quick start

```bash
xeli sales.csv             # open a CSV
xeli data.parquet          # or Parquet, JSON, JSONL, Excel
cat events.json | xeli     # pipe anything
psql -c "..." | xeli       # query results, browsable
xeli                       # no args? opens a file picker for the current dir
```

Press `Ctrl+K` and ask: *"top 10 employees by salary"*, *"average revenue by city for active customers"*, *"signups per week last quarter"*. xeli sends your schema + question to OpenAI or Anthropic, runs the returned SQL on DuckDB, and shows the rows.

## Features

| | |
|---|---|
| **Multi-format** | CSV, TSV, JSON, JSONL, Parquet, xlsx |
| **AI queries** | `Ctrl+K` — plain English → SQL via OpenAI or Anthropic |
| **SQL mode** | `Ctrl+Q` — direct DuckDB queries with history |
| **Filter** | `f` — visual builder with 12 operators (`=`, `>`, `contains`, `regex`, …) |
| **Sort** | `s` — cycle current column ASC / DESC / off |
| **Search** | `/` — regex across all columns, `n` / `N` to walk matches |
| **Group-by + pivot** | `g` — multi-column grouping wizard with `COUNT`, `SUM`, `AVG`, `MIN`, `MAX` |
| **Join** | `J` — pick a second file, choose `INNER` / `LEFT` / `RIGHT` / `FULL` |
| **Computed columns** | `c` — derive new columns from expressions (`price * qty`) |
| **Formula bar** | `=` — one-shot expressions (`SUM(amount)`, `MEDIAN(salary)`) |
| **Histograms** | `v` — sparkline distribution for any numeric column |
| **Column stats** | `Ctrl+I` — min / max / mean / median / nulls / unique count |
| **Export** | `e` — write filtered result back out as CSV / JSON / Parquet |
| **Themes** | `t` — Dracula, Nord, Catppuccin, Tokyo Night, Solarized |
| **Command palette** | `Ctrl+P` — fuzzy-search every command |
| **Undo** | `u` — full view-state stack |
| **File picker** | run `xeli` with no args — fuzzy-searchable list of supported files in cwd |
| **Mouse** | click, scroll, drag to resize columns |

Full keybinding list is in the in-app help (`?`) or in [src/handlers/input.rs](src/handlers/input.rs).

## How is this different from VisiData / csvkit / xsv / sc-im?

Honest take — they're all great. Here's how I think about the niche:

| Tool | Best for | What xeli does differently |
|---|---|---|
| **[VisiData](https://www.visidata.org/)** | Mature, scriptable Python TUI | DuckDB engine + AI natural-language query layer |
| **[csvkit](https://csvkit.readthedocs.io/)** | Pipeline transforms | Interactive browsing instead of one-shot scripts |
| **[xsv](https://github.com/BurntSushi/xsv)** | Blazing fast CSV processing | Multi-format (Parquet, JSON, xlsx) + TUI + SQL |
| **[Miller](https://miller.readthedocs.io/)** | Streaming transforms across formats | Interactive table, group-by wizard, joins by point-and-click |
| **[sc-im](https://github.com/andmarti1424/sc-im)** | Classic spreadsheet UX | Less Excel-cell-editing, more "browse + query large data" |
| **[q](https://harelba.github.io/q/)** | SQL over CSVs from the shell | Same idea + TUI + NL query + larger file formats |

If you live in a notebook for ad-hoc data exploration, xeli is the "I just want to look at this CSV without spinning up Python" tool.

## AI setup

xeli works without AI — every feature except `Ctrl+K` runs offline. To enable natural language queries, set an API key:

```bash
# OpenAI
export OPENAI_API_KEY=sk-...
# or: xeli config set-key openai sk-...

# Anthropic (Claude)
export ANTHROPIC_API_KEY=sk-ant-...
# or: xeli config set-key anthropic sk-ant-...
```

If you press `Ctrl+K` with no key set, xeli walks you through it inline — pick a provider, paste your key, hit Enter. No restart needed.

## Examples

There are sample datasets in [`examples/`](examples/):

```bash
xeli examples/employees.csv      # 30 rows × 7 columns
xeli examples/departments.csv    # join target
xeli examples/employees.json     # same data, JSON
```

Try this in the AI bar (`Ctrl+K`):
- `top 3 highest-paid people in engineering`
- `average salary by department, ordered by amount`
- `who has been here the longest in each city?`

Or join two files (`J`):
- Open `examples/employees.csv`
- Press `J`, enter `examples/departments.csv`
- Pick `INNER` join, both on the `department` column

## Building from source

```bash
cargo build --release
./target/release/xeli examples/employees.csv
```

Tests (release mode required on macOS):

```bash
cargo test --release
```

## Tech stack

- **Rust** — single static binary, ~3k LOC, no runtime deps
- **[Ratatui](https://ratatui.rs/)** — terminal UI rendering
- **[DuckDB](https://duckdb.org/)** — embedded vectorized SQL engine
- **[clap](https://docs.rs/clap)** — CLI parsing
- **[tokio](https://tokio.rs/)** — async runtime for the AI request only

## Contributing

PRs welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for build + test loop and the open help-wanted list (Windows, Ollama backend, streaming, themes).

## License

MIT. See [LICENSE](LICENSE).
