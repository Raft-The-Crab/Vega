# Core

This directory contains the shared Rust core editing engine.

## Structure

- **crates/** - Rust workspace containing all core crates
  - `vega-core/` - Main document model, editing operations, and history
  - `vega-ffi/` - C ABI for interop with desktop and mobile apps
  - `vega-render/` - CPU and GPU rendering pipelines
  - `vega-formats/` - File format parsers and writers (VEGA, PSD, PNG, etc.)

## Core Crates

### vega-core
The authoritative document model, command system, undo/redo, and editing operations.

### vega-ffi
Versioned C ABI exports for interop with C# desktop and Kotlin Android applications.

### vega-render
CPU reference rendering and optional GPU acceleration via wgpu.

### vega-formats
File format support including native `.vega`, PSD/PSB, PNG, JPEG, WebP, TIFF, and BMP.

## Build

```bash
cd core
cargo build --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

## Documentation

See `../docs/` for detailed architecture and development guides.
