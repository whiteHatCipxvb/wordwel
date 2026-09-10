# wordwel

Learning Rust by rewriting coreutils, one tool at a time

## Installation

```bash
cargo build --release
```

## Usage

```bash
./target/release/wordwel src/*.rs
cat README.md | ./target/release/wordwel
```

## What it does

- Counts lines, words and bytes like wc
- Parallel over files with std threads
- Zero dependencies outside std
- Reads stdin or multiple files

## Project structure

```text
├── .github/
│   ├── workflows/
│   │   └── ci.yml
│   └── pull_request_template.md
├── docs/
│   ├── development.md
│   ├── roadmap.md
│   └── usage.md
├── examples/
│   └── quickstart.md
├── src/
│   └── main.rs
├── .gitignore
├── CHANGELOG.md
├── CONTRIBUTING.md
├── Cargo.toml
├── LICENSE
└── SECURITY.md
```

## Development

```bash
cargo build
cargo clippy -- -D warnings
```

## License

MIT licensed, see LICENSE.
