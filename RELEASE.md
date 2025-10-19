# Release Guide for Arlon CLI

This project is configured with `cargo-dist` for automated releases. Here's how to create and manage releases:

## Setup Complete

✅ cargo-dist has been configured with:
- **Platforms**: macOS (Intel/Apple Silicon), Linux (x86_64/ARM64), Windows (x86_64)
- **Installers**: Shell script installer, Homebrew formula
- **CI**: GitHub Actions workflow for automated builds
- **Homebrew Tap**: Eagle-Konbu/arlon

## Creating a Release

### 1. Update Version Numbers

Update the version in both `cli/Cargo.toml` and `core/Cargo.toml`:

```toml
[package]
version = "0.2.0"  # Update this
```

### 2. Create and Push a Git Tag

```bash
# Create a tag for the new version
git tag v0.2.0

# Push the tag to trigger the release workflow
git push origin v0.2.0
```

### 3. Automatic Process

When you push a tag, GitHub Actions will automatically:
- Build binaries for all supported platforms
- Create installation packages (tar.xz, zip)
- Generate checksums
- Create a shell installer script
- Create a Homebrew formula
- Create a GitHub Release with all artifacts

## Installation Methods

After release, users can install arlon using:

### Shell Installer (Recommended)
```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/Eagle-Konbu/arlon/releases/latest/download/arlon-cli-installer.sh | sh
```

### Homebrew (macOS/Linux)
```bash
# First time setup
brew install Eagle-Konbu/arlon/arlon-cli

# Updates
brew upgrade arlon-cli
```

### Manual Download
Download platform-specific archives from the [GitHub Releases page](https://github.com/Eagle-Konbu/arlon/releases).

## Homebrew Token Setup

For Homebrew publishing to work, you need to:

1. Create a Personal Access Token on GitHub with `public_repo` permissions
2. Add it as a repository secret named `HOMEBREW_TAP_TOKEN`
3. Go to: Repository Settings → Secrets and variables → Actions → New repository secret

## Testing a Release

To test the release process without actually publishing:

```bash
# Test the build locally
cargo dist build

# See what would be released
cargo dist plan
```

## Versioning Strategy

Use semantic versioning (SemVer):
- `v1.0.0` - Major release
- `v1.1.0` - Minor release (new features)
- `v1.0.1` - Patch release (bug fixes)
- `v1.0.0-beta.1` - Pre-release (will be marked as pre-release on GitHub)

## File Structure

```
arlon/
├── .github/workflows/release.yml  # GitHub Actions workflow
├── dist-workspace.toml            # cargo-dist configuration
├── Cargo.toml                     # Workspace config with [profile.dist]
├── cli/Cargo.toml                 # CLI package metadata
└── core/Cargo.toml                # Core library metadata
```

## Troubleshooting

- **Build fails**: Check that all dependencies compile on all target platforms
- **Homebrew fails**: Ensure `HOMEBREW_TAP_TOKEN` secret is set correctly
- **Release doesn't trigger**: Verify tag format matches `v*.*.*` pattern