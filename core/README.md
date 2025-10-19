# Arlon Core

Core library for Git repository comparison and analysis. This library provides the fundamental building blocks for comparing branches and files in Git repositories.

## Features

- **Domain-Driven Design**: Clean architecture with separated concerns
- **Git Repository Operations**: Compare commits and files between branches
- **Multiple Output Formats**: JSON and simple text formatting
- **Testable**: Comprehensive test coverage with mock support

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
arlon-core = "0.1.0"
```

## Example

```rust
use arlon_core::{GitRepositoryImpl, CompareCommitsUseCase, CompareFilesUseCase};

// Compare commits
let git_repo = GitRepositoryImpl::new(".")?;
let use_case = CompareCommitsUseCase::new(&git_repo);
let commits = use_case.execute("main".to_string())?;

// Compare files
let use_case = CompareFilesUseCase::new(&git_repo);
let files = use_case.execute("main".to_string())?;
```

## Architecture

The library follows clean architecture principles:

- **Domain Layer**: Core business logic and entities
- **Application Layer**: Use cases and DTOs
- **Infrastructure Layer**: Git operations and output formatting

## License

MIT