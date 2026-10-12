# Infrastructure

This directory contains all infrastructure, build, CI/CD, and tooling configurations.

## Structure

- **ci/** - GitHub Actions workflows and CI configuration
- **build/** - Build output directories and packaging
- **config/** - Configuration files for various platforms and tools
- **scripts/** - Utility scripts for building, testing, and deployment
- **tools/** - Development tools, linters, formatters, and generators

## CI/CD

GitHub Actions workflows for:
- Windows desktop builds
- macOS desktop builds
- Android builds
- Rust core builds
- Backend deployment
- Security scanning
- Dependency checks

## Build Artifacts

Platform-specific build outputs:
- `build/windows/` - Windows installers, MSIX packages, portable builds
- `build/macos/` - macOS app bundles, DMG, PKG
- `build/android/` - APK, AAB, bundles

## Scripts

Common utility scripts for:
- Building all platforms
- Running tests
- Code formatting
- Linting
- Documentation generation
- Deployment automation

## Tools

Custom development tools for:
- Code generation
- Asset processing
- Format conversion
- Code signing
- Package validation
