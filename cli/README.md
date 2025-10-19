# Arlon CLI

Command-line interface for Git repository comparison and analysis, built on top of `arlon-core`.

## Installation

### From Source

```bash
git clone https://github.com/Eagle-Konbu/arlon.git
cd arlon
cargo build --release --bin arlon
```

The binary will be available at `target/release/arlon`.

## Usage

### Compare Commits

Show commits in HEAD that are not in the specified branch:

```bash
arlon commits main
arlon commits main --format json
```

### Compare Files

Show files that differ between the current branch and the specified branch:

```bash
arlon files main
arlon files main --format json
```

## Output Formats

- **Simple** (default): One-line format with essential information
- **JSON**: Structured format with full details

## License

MIT