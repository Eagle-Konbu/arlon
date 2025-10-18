# Arlon

A CLI tool to show commits in HEAD that are not in the specified branch.

## Features

- Compare current branch (HEAD) with any other branch
- Show commits that exist in HEAD but not in the target branch
- Multiple output formats:
  - **Simple**: One-line format with hash, date, and message
  - **JSON**: Structured format with full commit details

## Installation

### From Source

```bash
git clone https://github.com/Eagle-Konbu/arlon.git
cd arlon
cargo build --release
```

The binary will be available at `./target/release/arlon`.

### Using Cargo

```bash
cargo install --path .
```

## Usage

### Basic Usage

Show commits in HEAD that are not in the specified branch:

```bash
arlon <branch-name>
```

### Output Formats

#### Simple Format (Default)

```bash
arlon main
```

Output:
```
453d1733970aea8e088d8f57e638900ea3d8da74 2025-10-18 00:08:24 Add CLI tool with git2 and chrono dependencies
```

#### JSON Format

```bash
arlon main --format json
```

Output:
```json
[
  {
    "hash": "453d1733970aea8e088d8f57e638900ea3d8da74",
    "author": "Atsuya Uchida",
    "email": "atsuya_eagle@outlook.com",
    "date": "2025-10-18 00:08:24",
    "message": "Add CLI tool with git2 and chrono dependencies"
  }
]
```

### Options

```
Usage: arlon [OPTIONS] <BRANCH>

Arguments:
  <BRANCH>  Branch name to compare against

Options:
  -f, --format <FORMAT>  Output format [default: simple] [possible values: simple, json]
  -h, --help             Print help
```

## Use Cases

### Check commits before merging

```bash
# Check what commits will be merged from feature branch to main
git checkout feature-branch
arlon main
```

### Generate release notes

```bash
# Get commits for release notes in JSON format
arlon release/v1.0 --format json > release-notes.json
```

### Review branch differences

```bash
# Compare current branch with development branch
arlon develop
```


## License

MIT License - see [LICENSE](LICENSE) file for details
