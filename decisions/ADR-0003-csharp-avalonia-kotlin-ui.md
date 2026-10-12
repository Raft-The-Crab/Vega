# ADR-0003: C# / Avalonia desktop UI, Kotlin Android UI, shared Rust core

- **Status:** Accepted for Phase 0 prototype; release stack approval gated by validation.
- **Date:** 2026-10-12
- **Supersedes:** ADR-0001's decision to use egui/eframe for the desktop UI. The TypeScript backend decision in ADR-0002 remains unchanged.

## Context

The v2.0 experiment proposed egui/eframe as VEGA's only application UI. The product owner prefers to use C#/.NET 11 for the Windows-first/macOS-later desktop because they want a richer, easier-to-style desktop UI than the current egui direction provides. The Android target needs its own touch-first design and native lifecycle handling. Rust remains attractive for a high-performance shared editing/document core, and TypeScript was already selected for Cloudflare Workers.

## Decision

1. **Desktop UI:** C# / .NET 11 + Avalonia UI / XAML for Windows 11 first and macOS later. Use MVVM and reusable controls/theme tokens. Keep view code-behind small and purposeful.
2. **Shared core:** Rust owns the authoritative editable document, command/history semantics, core image operations, tile/pixel processing, format parsing and CPU reference rendering.
3. **Desktop boundary:** expose a narrow versioned C ABI from Rust. Call it from a safe C# wrapper using source-generated `LibraryImport`/P/Invoke where supported. Use opaque handles, documented ownership and stable error/status contracts.
4. **Android UI:** Kotlin + Jetpack Compose, with a deliberately touch-first workflow. Reuse Rust core logic through UniFFI Kotlin bindings or a tested JNI/C ABI facade. Select the exact binding path after a small, measured prototype.
5. **Backend:** preserve the separate TypeScript Cloudflare Worker, D1, R2 and Firebase Authentication architecture.
6. **Rendering:** start with a correct CPU rendering path and viewport-sized outputs. Avoid premature direct GPU resource sharing between Rust/wgpu and Avalonia/Skia. Add GPU processing only after profiling and a focused integration prototype.
7. **Scope control:** preserve the product vision and feature list. This ADR changes implementation technology and boundaries, not the requested end-state capabilities.

## .NET 11 / Avalonia risk

On the decision date, official Microsoft material reports .NET 11 at Release Candidate 1, with GA expected in November 2026; the support policy gives RC1 an end-of-support date of 2026-10-13. The Avalonia project has an open .NET 11 support issue tracked to milestone 12.2. Therefore the .NET 11 + exact Avalonia version must pass Phase 0; no public release may depend on an expired prerelease runtime. Recheck official status when initializing and packaging the project.

References:

- .NET what's new/release status: https://learn.microsoft.com/en-us/dotnet/whats-new/
- .NET support policy: https://dotnet.microsoft.com/en-us/platform/support/policy/dotnet-core
- Avalonia platform targets: https://docs.avaloniaui.net/docs/supported-platforms
- Avalonia .NET 11 tracking issue: https://github.com/AvaloniaUI/Avalonia/issues/22011
- .NET P/Invoke source generation: https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke-source-generation
- UniFFI: https://github.com/mozilla/uniffi-rs

## Consequences

### Benefits

- A C#/XAML desktop UI gives VEGA a practical foundation for a rich, custom-styled workspace, keyboard shortcuts, accessible controls and desktop-centric layout.
- Kotlin Compose provides native Android interaction instead of trying to force desktop UI patterns onto phones.
- The Rust core remains reusable across desktop and Android and owns image-editing semantics in one place.
- The TypeScript cloud backend remains separately deployable and does not complicate the local editor.

### Costs and risks

- The native boundary requires versioning, tests, explicit memory/buffer ownership, and careful threading behavior.
- .NET 11/Avalonia compatibility must be verified during its release-candidate window.
- Maintaining C#, Rust, Kotlin and TypeScript requires clear ownership and consistent contracts; small vertical slices are mandatory.
- Direct GPU-resource interop between rendering systems may be difficult; it is not a Phase 0 requirement.
- Mac and Android support require their own CI, packaging, and real-device tests.

## Phase 0 acceptance gate

The team must demonstrate:

- Avalonia/XAML desktop app running on Windows 11 with the selected .NET 11 SDK/version.
- Approved visual system and working canvas zoom/pan, menus, tabs, contextual properties and layer panel prototype.
- Verified docking approach, high-DPI behavior, keyboard/IME behavior, accessibility names/focus, and large-image UI responsiveness.
- Rust library load, ABI version negotiation, basic request/result, safe buffer release, error mapping, and repeated create/dispose tests.
- CI builds and tests the C# app and Rust core independently and runs an integration test on a matching Windows runner.
- Written record of exact tested SDK/framework versions, dependency licenses, limitations, and upgrade path to .NET 11 GA.

The owner reviews the running prototype before broad feature development begins.
