# Build, Release & Development Roadmap

> VEGA Planning Set v2.1 · Hybrid UI/core implementation sequence

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [Rust rules](rust-development.md) · [Engineering quality](engineering-quality.md) · [Security](security-privacy.md)

## Delivery philosophy

Build a functioning, testable application in small vertical slices. Don't implement the whole feature list in one coding-agent prompt. Every phase ends with an app or service that builds, tests, and demonstrates real behavior.

## Phase 0 — validate Avalonia/.NET 11 + Rust bridge

**Goal:** prove the selected desktop UI and native-core boundary are practical before implementing the full editor.

Deliver:

- A minimal C#/.NET 11 + Avalonia/XAML desktop application that opens on Windows 11.
- A short reusable design system: typography, spacing/color tokens, reusable controls, theme switching, focus states, and error presentation.
- A dedicated image-canvas control with a test bitmap, zoom/pan, correct DPI conversion, and overlays.
- A real docking feasibility decision: validate a maintained compatible docking package or implement the planned stable resizable panels/doc tabs without overpromising float/dock behavior.
- A Rust `cdylib` loaded from C# through the versioned C ABI; test version reporting, handle lifecycle, status/error propagation, one basic image/viewport request, and safe disposal.
- XAML build, `dotnet build`, `dotnet test`, analyzers/format check, `cargo fmt`, `cargo check`, `cargo test`, Clippy, and Windows GitHub Actions CI.
- A dependency/license inventory, Windows packaging prototype, crash-on-native-load behavior, and visible fallback/error UI.
- A note on current .NET 11 release state, selected SDK, selected Avalonia package, exact tested docking dependency/license, and upgrade path to .NET 11 GA.

**Exit criteria:** clean checkout builds on a Windows runner; the app has an owner-approved visual style; canvas input and zoom/pan work at multiple Windows scale factors; keyboard navigation/text entry and accessibility names are viable; Rust library load/version/errors/disposal pass integration tests; long operations don't block UI; and versions are pinned only after the prototype passes.

**Do not yet build:** PSD, cloud accounts, public uploads, executable plugins, complex GPU interop, AI, Branchbook, or every Photoshop tool.

## Phase 1 — first real image-editor vertical slice

- Open PNG and JPEG using bounded, validated decoding.
- Show image on a checkerboard/neutral canvas, with pan, zoom, fit-to-window and 100% view.
- Export unchanged image and prove that decode/display/export semantics are correct with fixtures.
- Add a document tab, title/dirty state, recent files, and basic open/save error handling.
- Implement an initial CPU reference color/composite path.

**Exit criteria:** test fixtures survive an open/export cycle without unintended pixel changes within the defined format/color tolerance; UI remains responsive; malformed and oversized inputs fail safely.

## Phase 2 — document model, editing commands, and native project

- Stable node IDs; raster layer and group; visibility, opacity, transform and ordering.
- One command dispatcher with undo/redo records; transactional failure behavior.
- Save/open of versioned `.vega` projects, schema validation, atomic save, autosave, recovery, and explicit migration tests.
- Render cache invalidation and dirty regions.
- Basic pixel/geometry fixtures and property tests for graph invariants.

**Exit criteria:** edits persist after close/reopen, undo/redo restores correct state, invalid projects are rejected without crashing, and autosave recovery is predictable.

## Phase 3 — usable core tools

Add one complete workflow at a time: crop/resize, move/transform, brush/eraser, rectangular/elliptical selection, masks, common adjustments, text, shapes, color picker, gradients, layer groups, blend-mode subset, export options. Each tool must have contextual properties, shortcut/menu command, undo integration, and a test.

**Exit criteria:** beginners can complete basic crop/color/text/export tasks; advanced values are available where implemented; no tool appears enabled if its operation is missing.

## Phase 4 — PSD import beta

- Read the public format specifications and create a documented supported-feature matrix.
- Build safe parser layers with checked arithmetic, explicit file/decompression limits, recursion limits, and fixture tests.
- Support the highest-value layer structures first; show visible compatibility warnings for unsupported effects, smart objects, text, masks, profiles, and adjustment types.
- Build golden render and semantic import fixtures from lawful sources.
- Add fuzz/property tests for parser boundaries and malformed files.

**Exit criteria:** claims are restricted to tested capabilities; unsupported content is never silently reported as preserved; malformed files are handled safely. PSD export/PSB support only follows a written subset definition.

## Phase 5 — engineering hardening and Windows beta

- Large-document memory-budget tests and benchmarks.
- Tablet/pen pressure, display scaling, accessibility/keyboard paths, and configurable shortcuts.
- Crash reporting UX without automatic upload, recovery checks, diagnostics export, installer/uninstaller, signed binaries and signed update metadata.
- License and software-bill-of-materials review.
- Dependency advisory and static-analysis checks in CI.

**Exit criteria:** release candidate built from a clean tagged commit; critical tests pass; signed artifacts verified; backup/recovery, privacy, and support policies are documented.

## Phase 6 — basic cloud backend

Maintain a separate TypeScript Worker project. Deliver Firebase Auth sign-up/login/verification/reset, ID-token verification at the Worker, account profile, private R2 uploads/downloads, D1 metadata, quota reservation/finalization, and the 250,000,000-byte per-account private quota. Keep local editing independent of this backend.

**Exit criteria:** integration tests prove unauthorized users cannot read or overwrite private objects; concurrent uploads cannot exceed quota; failed uploads release reservations; secrets are never present in client builds; staging and production are isolated.

## Phase 7 — public shares and community library

Add public/unlisted share records, controlled download URLs, previews produced by a safe processing path, uploader ownership, reporting/takedown workflow, moderation controls, rate limiting, abuse/cost alerts, and audit logging. Public sharing has no per-user cumulative storage quota per owner policy, but operations remain subject to content policy and service protection.

**Exit criteria:** unauthorized private access tests pass; report/takedown process works; content type does not dictate unsafe rendering; object keys are not authorization; operational limits and cost alerts are tested.

## Phase 8 — safe resources and plugins

Start with declarative resource packs (presets, brushes, gradients, shapes and templates). Then design a versioned plugin API with capabilities and a Wasm sandbox. Do not load user native libraries or arbitrary executable binaries into VEGA's main process. Implement plugin install/remove/disable, package signature/hash verification, permission prompts, version constraints, resource budgets, logs, and revocation before allowing third-party code.

## Phase 9 — professional tools and signature innovations

Expand feature families based on dependency order and validated demand: selection refinement, advanced brush dynamics, retouching, color science, RAW, typography, paths, adjustment layers, advanced compositing, animation exploration and automation.

Only then integrate signature systems over stable core abstractions:

1. Change Lens and Output Lab (comparison/preflight foundation).
2. Recipe Studio (command serialization and repeatable operations).
3. Design Contracts and Variant Matrix (semantic document constraints and relationships).
4. Branchbook and Edit Atlas (history/branch snapshots and explainability).
5. Source Ledger, Semantic Canvas, and Learn by Replay.

This is an implementation sequence, not a promise that all these features ship together.

## Phase 10 — macOS

Build with appropriate GitHub-hosted macOS runners, package for the chosen distribution channel, verify signing/notarization requirements, test Apple Silicon and relevant OS versions, and validate dialogs, clipboard, fonts, display scaling, pen/tablet behavior, file associations and GPU fallback. Do not mark macOS complete just because it compiles in CI.

## Phase 11 — Android feasibility and later release

Build a Kotlin + Jetpack Compose application with a touch-first layout. Integrate the shared Rust core through a small UniFFI Kotlin facade or JNI/C ABI binding selected by a tested spike. Verify Android 11+ lifecycle, content URIs/scoped storage, permissions, keyboard/text input, gestures, memory/thermal limits, and physical-device behavior. Ship a selected mobile feature set, not a desktop UI squeezed into a phone screen.

## Standard task loop for AI-assisted implementation

1. Read `plan.md`, `rust-development.md`, and the relevant domain file.
2. Inspect the repository before writing code.
3. State scope, constraints, and acceptance criteria in the task.
4. Implement a single coherent vertical slice.
5. Run formatting, check, targeted tests, full relevant tests, and Clippy.
6. Inspect diff, handle errors, and remove debug/placeholder code.
7. Report exact validation results and any skipped validation.
8. Update docs only where behavior, API, architecture, or acceptance criteria changed.

## Release gates

A release is blocked if any of the following holds without an explicit documented risk acceptance by the owner: untested critical save/recovery behavior, a private-cloud authorization bypass, unsigned/tampered update acceptance, a known critical parser vulnerability, hidden upload of artwork, unbounded quota bypass, or a plugin path that can execute untrusted native code in-process.

## Official technical references

- Avalonia platforms: https://docs.avaloniaui.net/docs/supported-platforms
- Avalonia .NET 11 support tracker: https://github.com/AvaloniaUI/Avalonia/issues/22011
- .NET support policy: https://dotnet.microsoft.com/en-us/platform/support/policy/dotnet-core
- .NET LibraryImport: https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke-source-generation
- UniFFI: https://github.com/mozilla/uniffi-rs
- Jetpack Compose: https://developer.android.com/develop/ui/compose
- wgpu: https://wgpu.rs/
- Cargo workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Clippy: https://doc.rust-lang.org/clippy/
- Cloudflare Workers TypeScript: https://developers.cloudflare.com/workers/languages/typescript/
- Firebase Authentication: https://firebase.google.com/docs/auth
