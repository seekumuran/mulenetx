# MULENETX-OMEGA

### Graph-Native Financial Crime Intelligence Terminal

MULENETX-OMEGA is a Rust-based terminal workstation for exploring synthetic financial-market data, account risk signals, transaction flows, and connected entities through a dense, Bloomberg-inspired TUI.

The project explores how graph-oriented intelligence and transaction-pattern analysis can be presented in a lightweight, interactive terminal interface.

**Status:** Active development  
**Language:** Rust  
**Interface:** Terminal UI (TUI)

---

## Features

- **Market workspace** — stacked candlestick charts and a multi-symbol watchlist.
- **Risk watch** — synthetic account records with risk scores, signal labels, clusters, and exposure values.
- **Transaction trace** — linked account-to-account transactions with timestamps, pattern labels, and transfer amounts.
- **Ticker-linked intelligence** — inspect account and transaction data associated with a selected market symbol.
- **Keyboard-driven navigation** — switch symbols, navigate workspaces, and inspect tabular data.
- **Deterministic synthetic network** — generated account and transaction data for repeatable demonstrations without requiring a live financial-data backend.

### Supported symbols

`AAPL` · `MSFT` · `NVDA` · `AMZN` · `GOOGL` · `META` · `TSLA` · `AVGO` · `ORCL` · `AMD`

---

## Preview

The terminal workspace combines two market charts on the left, account-risk and transaction-intelligence panels on the right, and a watchlist along the bottom.

> Add a screenshot of the running terminal here.

---

## Getting Started

### Requirements

- Git
- A Rust toolchain installed through [rustup](https://rustup.rs/)
- Cargo, included with Rust
- A terminal emulator that supports interactive terminal applications

### Clone the repository

```bash
git clone https://github.com/seekumuran/mulenetx.git
cd mulenetx
```

### Verify the source

```bash
cargo check
```

### Format the code

```bash
cargo fmt --check
```

To apply standard Rust formatting:

```bash
cargo fmt
```

### Run the application

```bash
cargo run
```

For an optimized build:

```bash
cargo build --release
```

The executable will be generated under `target/release/` for your platform.

**Windows note:** Some managed Windows environments enforce application-control policies that block locally built executables. If compilation succeeds but execution is blocked, consult the relevant Windows security logs or your administrator.

---

## Navigation

The exact key bindings may depend on the active workspace and repository configuration.

| Key | Action |
|---|---|
| `q` | Quit |
| `Tab` / `1–9` | Switch views |
| `↑` / `↓` | Select ticker |
| `/` | Find ticker |
| `t` | Change chart interval |
| `r` | Refresh |
| `s` | Settings |
| `?` | Help |
| `PageUp` / `PageDown` | Scroll data, where supported |

Use the in-application help for the current key-binding list.

---

## Synthetic Data and Interpretation

This repository's intelligence workspace uses generated demonstration data.

- Account identifiers, exposures, risk scores, and transaction records are synthetic.
- Signal labels such as `CIRCULAR`, `VELOCITY`, and `CROSS-CLUSTER` represent demo pattern classifications.
- Market symbols are used to organize the demonstration workspace; displayed intelligence is not evidence of wrongdoing by any real company or individual.
- Synthetic risk scores are not calibrated probabilities of fraud.
- The project is a research and engineering prototype, not a production anti-money-laundering system.

Do not use demonstration outputs to make real financial, compliance, or law-enforcement decisions.

---

## Technology

- **Rust** — application logic and data processing
- **Ratatui** — terminal interface and tabular presentation
- **Crossterm** — terminal interaction
- **Cargo** — dependency management and builds

See `Cargo.toml` for the project's dependency declarations.

---

## Development

Check compilation before submitting changes:

```bash
cargo check
cargo fmt --check
```

Run the application locally:

```bash
cargo run
```

Contributions, reproducible bug reports, and suggestions for improving the terminal interface and synthetic graph-intelligence demonstrations are welcome.

---

## License and Attribution

See the repository's `LICENSE` file for the applicable license and reuse conditions. This project originated from an existing Rust TUI codebase; retain applicable upstream notices and attribution when redistributing or modifying inherited components.

---

**Repository:** [github.com/seekumuran/mulenetx](https://github.com/seekumuran/mulenetx)
