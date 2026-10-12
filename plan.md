# VEGA — Master Planning Index

**Planning set:** v2.1 · **Architecture:** C# desktop UI + Rust editing core + Kotlin Android UI · **Status:** product vision locked; implementation staged

VEGA is a free, proprietary image editor targeting Windows 11 first, macOS later, and Android 11+ with a native touch-oriented interface. It aims to cover professional image-editing workflows in one unified workspace while making common tasks easier. It includes a native `.vega` project format, PSD interoperability, optional AI, online sharing, and a carefully sandboxed plugin/resource ecosystem.

This folder is the canonical planning set. Read `plan.md` first, then load the relevant topic files for a task. v2.1 supersedes the v2.0 decision to use egui/eframe for the UI. The new direction separates the user interface from the shared, performance-sensitive Rust core. The online backend remains an intentional TypeScript exception.

## Start here

1. [Product Vision & Decisions](product-vision.md) — product promise, policy, and scope boundaries.
2. [Software Architecture](architecture.md) — language split, layers, core bridge, rendering, and repository layout.
3. [C# / .NET Development Rules](dotnet-development.md) — .NET 11, Avalonia, MVVM, UI implementation, P/Invoke, and coding-agent workflow.
4. [Rust Core Development Rules](rust-development.md) — Rust core, API boundaries, tests, dependencies, and safe-code rules.
5. [Kotlin / Android Development Rules](android-development.md) — deferred Android platform and Rust binding requirements.
6. [UI & User Experience](ui-ux.md) — one workspace, panels, controls, visual system, and workflows.
7. [Build, Release & Development Roadmap](build-release-roadmap.md) — phase gates and CI.
8. [Security, Privacy & Anti-Theft](security-privacy.md) — must-read before cloud, uploads, plugins, or release work.

## Category documents

| Area | File | Contents |
|---|---|---|
| Product vision & decisions | [`product-vision.md`](product-vision.md) | Goals, audience, pricing, platform and locked policies. |
| Unified workspace & UI/UX | [`ui-ux.md`](ui-ux.md) | Avalonia desktop interface, layout, command search, keyboard, accessibility, and user journeys. |
| Editing capabilities | [`editing-capabilities.md`](editing-capabilities.md) | Professional feature inventory and acceptance criteria. |
| AI & automation | [`ai-automation.md`](ai-automation.md) | Optional AI, recipes, scripts, and batch operations. |
| Hybrid architecture | [`architecture.md`](architecture.md) | C# desktop, Rust core, Kotlin Android, bridge contracts, rendering, and project layout. |
| C# / .NET development | [`dotnet-development.md`](dotnet-development.md) | .NET 11, Avalonia, MVVM, generated P/Invoke, UI tests, and vibe-coding rules. |
| Rust core development | [`rust-development.md`](rust-development.md) | Rust core APIs, Cargo, safety, error handling, interop, tests. |
| Kotlin / Android development | [`android-development.md`](android-development.md) | Android 11+, Compose UI, lifecycle, Rust bindings, touch-first behavior. |
| File formats | [`file-formats.md`](file-formats.md) | Native `.vega`, PSD/PSB, import/export, compatibility reporting. |
| Backend & cloud | [`backend.md`](backend.md) | Firebase Auth, TypeScript Workers, D1/R2, quotas, shares, API contracts. |
| Plugins & assets | [`plugins-and-assets.md`](plugins-and-assets.md) | Resource packs, plugin packages, sandboxing, fonts, presets, and assets. |
| Security & privacy | [`security-privacy.md`](security-privacy.md) | Account security, software integrity, privacy, moderation, and abuse handling. |
| Engineering quality | [`engineering-quality.md`](engineering-quality.md) | Tests, benchmarks, performance, native-boundary tests, and release gates. |
| Build & roadmap | [`build-release-roadmap.md`](build-release-roadmap.md) | Prototype, GitHub Actions, packaging, phases, and implementation tasks. |
| VEGA innovations | [`innovations.md`](innovations.md) | Branchbook, Edit Atlas, Design Contracts, Variant Matrix, Change Lens, Source Ledger, Recipe Studio, Output Lab, Semantic Canvas, Learn by Replay. |
| Decision records | [`decisions/`](decisions/) | Rationale for hybrid UI/core architecture and TypeScript backend. |
| Section manifest | [`SECTION_MANIFEST.md`](SECTION_MANIFEST.md) | Traceability to the earlier v1.5 product scope. |

## Locked product decisions

- **One workspace:** no separate Quick Studio and Pro Studio modes. Contextual properties, quick actions, command search, and optional panel layouts all operate on the same document.
- **Free and proprietary:** no subscription, paid tier, advertisements, or paywalled core editing. First-party VEGA code remains closed-source. All third-party code is reviewed and attributed as its license requires.
- **C# desktop UI:** C# on .NET 11 with Avalonia UI/XAML is the selected desktop direction for Windows and later macOS. Use MVVM for presentation and explicit interfaces for app services. Avoid a giant code-behind window or treating UI view models as the document source of truth.
- **Rust editing core:** Rust owns the authoritative document model, operations, history semantics, pixel/tile processing, CPU reference rendering, format parsers, and shared platform-neutral editing logic. Do not add C++ unless a proven platform/library need requires it.
- **Native bridge:** .NET calls a small versioned C ABI using source-generated `LibraryImport` where practical. Android uses a Kotlin-native interface with a UniFFI facade or a documented JNI/C ABI bridge after a prototype. Never pass unclear-lifetime pointers, GUI objects, or massive pixel buffers for every UI frame.
- **Kotlin Android:** Android 11+ gets a Kotlin/Jetpack Compose touch-first UI; reuse Rust editing operations where proven. It is not a resized desktop interface.
- **.NET 11 release policy:** current framework preview/RC status and Avalonia compatibility must be rechecked at implementation time. Prototype with the available .NET 11 SDK, track Avalonia support, and do not ship a final build on a .NET prerelease. Update to the .NET 11 GA SDK and an explicitly tested Avalonia version when available.
- **Desktop UI quality gate:** the Phase 0 Avalonia prototype must prove a canvas, working input, docking strategy, accessibility, high DPI, and responsiveness before full UI implementation. Avalonia's basic `DockPanel` is not by itself assumed to provide full IDE-style docking; evaluate a maintained compatible library or scope the first layout to fixed/rearrangeable docks.
- **Graphics:** start with a deterministic CPU/reference render path in Rust and render visible viewport regions into an Avalonia-friendly surface. Add GPU compute or direct GPU interop only after profiling and a focused proof of concept.
- **Keep architecture small:** begin with a few Rust crates and a few C# projects. Add boundaries when ownership, build, or test reasons are demonstrated.
- **Backend exception:** TypeScript Cloudflare Workers, D1, R2, and Firebase Authentication remain the chosen online stack. Backend contracts stay language-neutral.
- **Offline-first:** local open/edit/save/undo/export never requires login, cloud, or AI.
- **File formats:** versioned editable `.vega`; PSD import has honest feature reporting; PSB and advanced interchange/export are phased and tested.
- **Optional AI:** never required for core editing. Prefer local inference; no artwork upload without explicit, contextual consent.
- **Private cloud quota:** 250,000,000 bytes per account, shared across devices. Public/unlisted shares are a separate pool subject to safety and operational protections.
- **Device policy:** target three registered account identities per VEGA installation, without claiming tamper-proof physical-device enforcement unless proven.
- **Plugins:** declarative resources first; executable plugins later, sandboxed, permission-declared, versioned, reviewed, and revocable.
- **Security:** protect identity, privacy, project data and release integrity without covert surveillance or hostile anti-piracy behavior.
- **Scope freeze:** preserve the existing Photoshop-family goals and the ten VEGA signature systems. Do not add new major feature families during foundation work.

## Recommended implementation order

1. **Phase 0 — UI + bridge feasibility:** prove a polished Avalonia/.NET 11 desktop shell, Rust library build/load, file-open/render path, basic dock strategy, C ABI, CI, and the Kotlin-binding approach with tiny test programs. Confirm framework versions before general feature work.
2. **First vertical slice:** open PNG/JPEG, render accurately, zoom/pan, export an unchanged image, and test pixel correctness.
3. **Document foundation:** stable IDs, a basic layer model, command dispatch, undo/redo, `.vega` save/open, autosave and crash recovery.
4. **Usable editor:** crop, transform, brush/eraser, selections, masks, basic adjustments, text/shapes, and export preflight.
5. **PSD beta:** supported-feature matrix, safe parsing, compatibility reports, fixtures and fuzz tests.
6. **Cloud basics:** Firebase identity, Worker token verification, private cloud quota, then sharing/moderation.
7. **Security hardening:** signed releases/updates, parser hardening, plugin/resource validation, account/session controls, incident response.
8. **Professional breadth:** extend Photoshop-family workflows in dependency order; add acceleration based on measured bottlenecks.
9. **VEGA innovations:** integrate Branchbook, Change Lens, Design Contracts, Variant Matrix and other signature systems only after the document/history model supports them.
10. **Platforms & ecosystem:** macOS desktop; Kotlin/Compose Android with shared Rust core; safe resource packs first, sandboxed executable plugins later.

## How AI coding agents must work in this repository

- Read `plan.md`, the relevant domain file, and the C# or Rust coding rules before changing code.
- Inspect the repository and existing behavior before editing; do not blindly regenerate files.
- Implement one small vertical slice at a time. Keep the solution buildable and avoid placeholders.
- For changes crossing C#↔Rust or Kotlin↔Rust, update the bridge contract, generated bindings, tests, and versioning together.
- Run applicable .NET format/build/test/analyzer checks and Rust `cargo fmt`, `cargo check`, `cargo test`, and Clippy. Run Worker checks only when backend code changes.
- Report commands actually executed and their real results; disclose any unavailable platform or unrun tests.
- Do not add dependencies, formats, permissions, telemetry, or architecture layers without a concrete need and review.

## Version history

- **v1.5:** broad product vision, professional feature coverage, security and VEGA signature systems.
- **v2.0:** Rust/egui desktop experiment.
- **v2.1:** C#/.NET 11 + Avalonia desktop UI, shared Rust editing core, Kotlin/Compose Android, bounded native bridge, .NET 11/Avalonia compatibility gates, and platform-specific coding rules.
