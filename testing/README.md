# Testing

This directory contains all test code, fixtures, and testing infrastructure.

## Structure

- **unit/** - Unit tests for individual components
  - `desktop/` - C# desktop application unit tests
  - `core/` - Rust core library unit tests
  - `render/` - Rendering pipeline unit tests
  - `formats/` - File format parser unit tests
  - `interop/` - Native bridge unit tests

- **integration/** - Integration tests
  - `desktop-core/` - Desktop app + Rust core integration
  - `native-bridge/` - C# ↔ Rust bridge integration
  - `cloud/` - Backend integration tests

- **ui/** - UI tests
  - `automation/` - Automated UI tests
  - `manual/` - Manual test procedures

- **e2e/** - End-to-end test suites

- **performance/** - Performance benchmarks and regression tests

- **security/** - Security and fuzzing tests

- **fuzzing/** - Fuzzing tests for parsers and codecs

## Fixtures

Test fixtures are located in `../shared/fixtures/`:
- Images (various sizes, formats, color spaces)
- PSD files (compatibility testing)
- VEGA projects (version testing)
- Golden images (rendering reference)

## Running Tests

### Desktop (C#)
```bash
cd application/desktop
dotnet test
```

### Core (Rust)
```bash
cd core
cargo test --workspace
```

### Backend (TypeScript)
```bash
cd backend/worker
npm test
```

### Android (Kotlin)
```bash
cd application/android
./gradlew test
```

## Test Coverage

See `../docs/reference/engineering-quality.md` for testing strategy and coverage requirements.
