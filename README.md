# TD3 — Rust Log Analyzer (CLI)

A production-grade log analyzer written in Rust.

## How To Run

```bash
cargo run -- sample.log
```
Verbose:
```bash
cargo run -- sample.log --verbose
```

Errors only:
```bash
cargo run -- sample.log --errors-only
```

JSON:
```bash
cargo run -- sample.log --format json
```

CSV:
```bash
cargo run -- sample.log --format csv
```