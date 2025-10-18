# Arlon

A CLI tool to compare branches and files in Git repositories.

## Features

- **Commit Comparison**: Show commits in HEAD that are not in the specified branch
- **File Comparison**: Show files that differ between branches
- Multiple output formats:
  - **Simple**: One-line format with essential information
  - **JSON**: Structured format with full details

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

Arlon provides two main commands for comparing branches:

### 1. Commit Comparison

Show commits in HEAD that are not in the specified branch:

```bash
arlon commits <branch-name>
```

### 2. File Comparison

Show files that differ between branches:

```bash
arlon files <branch-name>
```

### Output Formats

#### Simple Format (Default)

**Commits:**
```bash
arlon commits main
```

Output:
```
453d1733970aea8e088d8f57e638900ea3d8da74 2025-10-18 00:08:24 Add CLI tool with git2 and chrono dependencies
```

**Files:**
```bash
arlon files main
```

Output:
```
modified src/cli.rs
added test_file.txt
modified README.md
```

#### JSON Format

**Commits:**
```bash
arlon commits main --format json
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

**Files:**
```bash
arlon files main --format json
```

Output:
```json
[
  {
    "path": "src/cli.rs",
    "status": "modified"
  },
  {
    "path": "test_file.txt",
    "status": "added"
  },
  {
    "path": "README.md",
    "status": "modified"
  }
]
```

### Commands

```
Usage: arlon <COMMAND>

Commands:
  commits  Show commits in HEAD that are not in the specified branch
  files    Show files that differ between branches
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

#### Commit Command Options

```
Usage: arlon commits [OPTIONS] <BRANCH>

Arguments:
  <BRANCH>  Branch name to compare against

Options:
  -f, --format <FORMAT>  Output format [default: simple] [possible values: simple, json]
  -h, --help             Print help
```

#### Files Command Options

```
Usage: arlon files [OPTIONS] <BRANCH>

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
arlon commits main
```

### Check file changes before merging

```bash
# Check what files have changed in the current branch compared to main
arlon files main
```

### Generate release notes

```bash
# Get commits for release notes in JSON format
arlon commits release/v1.0 --format json > release-notes.json
```

### Review branch differences

```bash
# Compare current branch with development branch
arlon commits develop

# Check file differences with development branch
arlon files develop
```

### Pre-merge analysis

```bash
# Get both commit and file differences before merging
arlon commits main --format json > commits.json
arlon files main --format json > files.json
```


## License

MIT License - see [LICENSE](LICENSE) file for details
