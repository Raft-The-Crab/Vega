# VEGA

VEGA is a proprietary, free-to-use image-editing application with professional capabilities. It uses a hybrid architecture: C#/.NET 11 + Avalonia for desktop UI, Rust for the shared editing core, Kotlin + Jetpack Compose for Android, and TypeScript for the Cloudflare Worker backend.

## Project Structure

```
VEGA/
├── application/           # Application code
│   ├── desktop/          # C#/.NET 11 + Avalonia (Windows/macOS)
│   └── android/          # Kotlin + Jetpack Compose (Android 11+)
├── core/                 # Shared Rust editing engine
│   └── crates/          # Rust workspace (vega-core, vega-ffi, vega-render, vega-formats)
├── backend/              # TypeScript Cloudflare Worker backend
│   ├── worker/          # Main Worker application
│   └── contracts/       # API contracts and type definitions
├── shared/              # Shared resources and test fixtures
│   ├── assets/          # Branding, UI resources, templates, presets
│   └── fixtures/        # Test data and golden images
├── infrastructure/      # CI/CD, build, scripts, and tools
│   ├── ci/             # GitHub Actions workflows
│   ├── build/          # Build output directories
│   ├── config/         # Configuration files
│   ├── scripts/        # Utility scripts
│   └── tools/          # Development tools
├── testing/            # All test code and infrastructure
│   ├── unit/           # Unit tests
│   ├── integration/    # Integration tests
│   ├── ui/             # UI tests
│   ├── e2e/            # End-to-end tests
│   ├── performance/    # Performance benchmarks
│   ├── security/       # Security tests
│   └── fuzzing/        # Fuzzing tests
├── docs/               # Documentation
│   ├── plan.md         # Master planning document
│   ├── product-vision.md
│   ├── specs/          # Feature specifications
│   ├── guides/         # Development guides
│   └── reference/      # Architecture and API reference
├── decisions/          # Architecture Decision Records (ADRs)
└── README.md           # This file
```

## Quick Start

### Prerequisites

- **Desktop:** .NET 11 SDK, Rust toolchain
- **Android:** Android SDK, Kotlin
- **Backend:** Node.js, Wrangler CLI

### Building

#### Desktop
```bash
cd application/desktop
dotnet build
```

#### Core (Rust)
```bash
cd core
cargo build --workspace
```

#### Backend
```bash
cd backend/worker
npm install
npm run build
```

#### Android
```bash
cd application/android
./gradlew build
```

## Documentation

Start with [docs/plan.md](docs/plan.md) for the complete planning overview.

- **[Product Vision](docs/product-vision.md)** - Goals, audience, and policies
- **[Architecture](docs/reference/architecture.md)** - Technical architecture and design
- **[Development Guides](docs/guides/)** - Platform-specific development rules
  - [Desktop Guide](docs/guides/desktop/dotnet-development.md)
  - [Core Guide](docs/guides/core/rust-development.md)
  - [Android Guide](docs/guides/android/android-development.md)
  - [Backend Guide](docs/guides/backend/backend.md)
- **[Feature Specs](docs/specs/)** - Detailed feature specifications

## License

Copyright (c) 2025 Raft-The-Crab. All rights reserved.

This software is proprietary and confidential. See [LICENSE](LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## Support

See [SUPPORT.md](SUPPORT.md) for support resources.

## Security

See [SECURITY.md](SECURITY.md) for security policy and vulnerability reporting.
