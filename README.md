# MULENETX

**A Terminal-Based Market Intelligence Dashboard**

MULENETX is a Rust-based terminal user interface (TUI) for exploring market data through a compact, multi-panel dashboard. It combines market charts and a watchlist in a keyboard-oriented interface designed for developers who prefer working directly in the terminal.

## Features

- **Multi-panel market dashboard** — view multiple market symbols simultaneously.
- **Price charts** — separate chart panels for AAPL, MSFT, NVDA, and BTC-USD.
- **Watchlist** — keep market symbols visible alongside the charts.
- **Terminal-native interface** — a focused, keyboard-driven experience built with Rust.
- **Release builds** — compile an optimized executable locally using Cargo.

## Preview

The dashboard currently displays four chart panels:

| Panel | Symbol |
|---|---|
| Top-left | AAPL |
| Top-right | MSFT |
| Bottom-left | NVDA |
| Bottom-right | BTC-USD |

A watchlist is displayed beneath the charts.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) and Cargo
- Git
- A terminal that supports the interface's layout and rendering

## Installation

Clone the repository:

```bash
git clone https://github.com/seekumuran/mulenetx.git
cd mulenetx
```

Build the project in release mode:

```bash
cargo build --release
```

Run the application:

```bash
cargo run --release
```

On Windows, the compiled executable is normally located at:

```text
target/release/mulenetx.exe
```

On Linux and macOS, the executable is normally located at:

```text
target/release/mulenetx
```

## Development

Check the project:

```bash
cargo check
```

Build a debug version:

```bash
cargo build
```

Build an optimized release:

```bash
cargo build --release
```

## Project Structure

```text
mulenetx/
├── Cargo.toml
├── Cargo.lock
├── README.md
└── src/
    ├── app.rs
    └── ui/
        ├── chart.rs
        ├── mod.rs
        ├── split.rs
        └── omega.rs
```

This is a simplified overview; the repository contains additional source files and modules.

## Technology Stack

- **Rust** — application implementation
- **Ratatui** — terminal user interface
- **Cargo** — dependency management and builds
- **Reqwest** — HTTP client functionality

## Attribution

MULENETX is derived from the open-source **AlphaAI-TUI** project.

Original upstream repository:  
https://github.com/makeev/alphai-tui

The original project's license and applicable attribution requirements are retained. See the repository's license file for the applicable terms.

## License

See [`LICENSE`](LICENSE) for licensing terms. MULENETX remains subject to the original project's license obligations and any applicable third-party dependency licenses.

---

**MULENETX** — market data, charts, and terminal workflows.
