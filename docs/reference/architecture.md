# Software Architecture — C# Desktop UI + Rust Core

> VEGA Planning Set v2.1 · Hybrid UI/core architecture
> Windows-first Avalonia desktop app; Kotlin-native Android later; TypeScript Cloudflare Worker remains the separate online backend.

[← Back to `plan.md`](plan.md) · [C#/.NET rules](dotnet-development.md) · [Rust core rules](rust-development.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [8. Technical architecture](#8-technical-architecture)
- [9. Proposed repository layout](#9-proposed-repository-layout)
- [29. Expanded repository layout](#29-expanded-repository-layout)
- [44. Data, command, render, and history contracts](#44-data-command-render-and-history-contracts)
- [60. Document model and image-science requirements](#60-document-model-and-image-science-requirements)
- [76. Architecture changes required for full feature scope](#76-architecture-changes-required-for-full-feature-scope)

---

## 8. Technical architecture

### 8.1 Platform and language plan

VEGA uses a deliberately small hybrid architecture: a high-quality C# desktop UI, a shared Rust editing core, a native Kotlin Android UI, and a separate TypeScript cloud backend. Rust is selected for the image/document engine—not as a mandate to build every platform UI in Rust.

**Windows 11 desktop — C# + .NET 11 + Avalonia UI.** Use Avalonia with XAML styles/templates and MVVM for a customizable professional desktop interface. The UI owns menus, panels, keyboard/focus behavior, command routing, window lifecycle, dialogs, preferences, cloud screens, and view models. Avalonia is the starting cross-platform desktop framework because the same C#/XAML UI can target Windows and macOS. Official docs: https://docs.avaloniaui.net/docs/supported-platforms.

**.NET 11 status is a planning risk, not an assumption.** As of 2026-10-12, Microsoft's .NET 11 documentation reports Release Candidate 1, with general availability expected in November 2026. The official support table lists .NET 11 RC1 end-of-support on 2026-10-13. Use the installed preview only for prototypes, keep the SDK selection explicit, re-evaluate as Microsoft publishes newer release candidates, and do not ship the final release against a prerelease runtime. Official refs: https://learn.microsoft.com/en-us/dotnet/whats-new/ and https://dotnet.microsoft.com/en-us/platform/support/policy/dotnet-core.

**Avalonia + .NET 11 compatibility gate.** Avalonia's current platform docs identify .NET 8+ for desktop, but the Avalonia project's .NET 11 support issue is tracked to milestone 12.2 and calls for testing. Therefore Phase 0 must build and run the exact installed .NET 11 SDK plus selected Avalonia version; check XAML compilation, native packaging, custom canvas behavior, docking package, accessibility, and designer/build tooling before locking versions. If the combination is blocked, do not patch framework internals or ignore errors: use a temporary isolated prototype on a supported target or wait for the compatible Avalonia release, then record the result in an ADR. Re-check: https://docs.avaloniaui.net/docs/supported-platforms and https://github.com/AvaloniaUI/Avalonia/issues/22011.

**Shared editing core — Rust.** Rust owns the canonical document graph, edit commands, history, native `.vega` serialization core, image tiles, compositing, masks, filters, brush operations, format parsing, CPU reference rendering, and background job primitives. Use safe Rust by default. Build native library artifacts for Windows and macOS, and Android ABIs later. Keep pixel operations out of C# and Kotlin except for explicitly UI-only previews or platform-specific glue.

**C#↔Rust boundary.** Use a narrow versioned C ABI exported by a Rust `cdylib`, consumed from .NET using source-generated `LibraryImport`/P/Invoke declarations where compatible. Microsoft documents the source generator here: https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke-source-generation. Use opaque engine/document handles and flat, explicitly versioned request/response types; define UTF-8 string rules, buffer lifetimes, return/error codes, cancellation, and ownership/free functions. Do not expose Rust struct layouts as a permanent ABI, pass GUI objects across the boundary, or make one native call per pixel.

**Android 11+ — Kotlin + Jetpack Compose.** Build a touch-first native Android UI, with Rust still implementing shared editing semantics. Keep Android lifecycle, storage access framework, intents, permissions, system UI, and touch presentation in Kotlin. Expose an intentionally small facade over Rust; evaluate UniFFI's Kotlin bindings for this target (official supported languages include Kotlin) while retaining the documented C ABI for desktop .NET. References: https://github.com/mozilla/uniffi-rs and https://developer.android.com/develop/ui/compose. Do not promise Android feature parity until tested on physical Android 11+ devices and current Android versions.

**macOS.** Reuse the Avalonia desktop app and Rust core once Windows behavior is stable. Build on macOS CI, then validate signing/notarization, Apple Silicon, OS dialogs, fonts, display scaling and graphics on real hardware. Do not treat a successful cross-compile as release readiness.

**Cloud backend exception.** Keep the independently deployed TypeScript Cloudflare Worker, Cloudflare D1/R2, and Firebase Authentication selected by the owner. It is not part of the desktop native bridge.

### 8.2 Desktop UI framework and prototype gate

Selected desktop starting stack:

- **Avalonia UI + XAML:** primary Windows/macOS view and styling system.
- **C# MVVM:** view models and presentation state; keep view code-behind limited to platform/control integration that cannot reasonably be expressed as commands/bindings.
- **Docking strategy:** evaluate a maintained Avalonia-compatible docking library with verified .NET 11 compatibility and license. Avalonia's basic `DockPanel` only lays out controls at edges; do not assume it supplies full Photoshop/IDE-style drag-to-dock, floating windows, persistent layouts, and tab groups. If no suitable dependency passes review, begin with fixed/resizable side panels and document tabs, then add richer docking as a separate feature.
- **Custom canvas control:** own zoom/pan, checkerboard, overlays, selection handles, guides and input routing in a dedicated control. View models do not own pixels.
- **Rust native library:** loaded through the small bridge; API availability/errors must be surfaced at startup rather than causing a crash.
- **Rendering first pass:** Rust returns viewport-sized render/tile results. Avalonia presents them using a compatible bitmap/render surface. Never transfer full-resolution pixel buffers every UI frame. Keep CPU output as the correctness reference.
- **GPU later:** `wgpu` remains a candidate for Rust rendering/compute. Direct sharing between `wgpu` resources and Avalonia/Skia GPU resources is a separate optimization project and must not be assumed easy or mandatory.

Phase 0 must demonstrate a polished Windows 11 window with menu, toolbar, tabs, central image canvas, contextual properties, layers panel, layout persistence, zoom/pan, file dialogs, keyboard and IME text input, accessibility names/focus, high-DPI scaling, error states, and responsiveness with a large test image. Evaluate docking, custom controls, DPI, pointer/stylus capture, fonts, dark/light theme tokens, and .NET/Avalonia build support. Let the owner visually inspect the prototype. If this UI stack does not meet quality needs, compare one concrete alternative by ADR before broad development.

### 8.3 Layered architecture

```text
VEGA Desktop (C# / .NET 11 / Avalonia)
  ├─ XAML views and reusable VEGA controls
  ├─ MVVM view models + presentation-only state
  ├─ Commands, menus, keyboard mapping, workspace layout
  ├─ App services: files, preferences, recent docs, cloud API, job UI
  └─ Versioned native bridge (C ABI / LibraryImport)
       └─ Rust native library
            ├─ Canonical document model + stable IDs
            ├─ Commands, transactions, undo/redo, snapshots
            ├─ Tile store, CPU compositor, viewport render results
            ├─ Brush, selection, masks, transforms, filters, color
            ├─ Native `.vega` serializer and migration core
            ├─ PSD/PSB and other format adapters
            ├─ Background jobs, cancellation, progress and diagnostics
            └─ Resource/plugin host (later, capability-limited)

VEGA Android (Kotlin / Jetpack Compose)
  ├─ Touch-first mobile UI, OS lifecycle and permissions
  └─ Generated/facade bindings to shared Rust editing core

Separate deployment:
  TypeScript Cloudflare Worker + D1 + R2 + Firebase Authentication
```

The Rust document model is authoritative. C# and Kotlin keep read-only or presentation-oriented projections and request document changes through commands. Do not maintain separate mutable layer trees that can drift. All entry points use the same command semantics and undo/history rules.

### 8.4 Solution and workspace structure: start small

Use a small .NET solution plus Cargo workspace. Do not create dozens of empty modules before they have real ownership boundaries.

```text
VEGA/
  global.json                       # explicitly selects available .NET 11 SDK; updated after GA
  Directory.Build.props             # nullable, analyzers, common settings
  Directory.Packages.props           # central package versions only after dependencies stabilize
  VEGA.sln
  Cargo.toml
  Cargo.lock
  rust-toolchain.toml
  src/
    desktop/
      Vega.Desktop/                  # Avalonia executable, App, MainWindow, theme
      Vega.Application/              # workflows, command routing, service interfaces
      Vega.Presentation/             # view models, UI-facing state, validation
      Vega.NativeInterop/            # safe C# wrapper for the Rust ABI
      Vega.Tests/                    # UI/application tests as appropriate
    shared-contracts/                # versioned DTO/schema docs, not pixel buffers
  crates/
    vega-core/                       # canonical document and editing semantics
    vega-ffi/                        # small C ABI cdylib for .NET; add when needed
    vega-kotlin-ffi/                 # UniFFI facade when Android phase begins
    vega-render/                     # split only if render ownership warrants it
    vega-formats/                    # split only when format-module size warrants it
  android/                           # add Kotlin/Compose project in Android phase
  backend/worker/                    # separate TypeScript Cloudflare deployment
  tests/
    fixtures/
    golden-images/
  docs/
    plan.md
    ARCHITECTURE.md
    DOTNET_DEVELOPMENT.md
    RUST_DEVELOPMENT.md
    FORMATS/VEGA_FORMAT.md
    FORMATS/PSD_COMPATIBILITY.md
    BACKEND_ARCHITECTURE.md
    PLUGIN_API.md
    ADR/
  .github/workflows/
    dotnet-ci-windows.yml
    rust-ci-windows.yml
    native-bridge-ci.yml
    dotnet-ci-macos.yml             # later
    android-ci.yml                  # later
    worker-ci.yml                   # when backend phase begins
```

The exact project count is not a goal: fold `Vega.Application` and `Vega.Presentation` together at first if separate projects create overhead. Split when build/test isolation or ownership justifies it. Keep `Cargo.lock` committed. Pin a reviewed Rust stable toolchain. For .NET, pin the selected SDK in `global.json`; never commit an expired .NET RC as the permanent release toolchain.

### 8.5 Ownership and concurrency rules

- Rust `vega-core` owns the authoritative document graph and operation semantics. Do not keep a second independently mutable layer graph in the UI or renderer.
- UI state owns presentation choices only: selected layer, active tool, viewport transform, visible panels, transient handles, and modal state.
- Source image/tile storage is authoritative in CPU memory or a controlled on-disk tile store. GPU textures and previews are disposable caches, never the only copy of user pixels.
- Keep expensive import/export/filter/render tasks off the UI frame. Use a bounded worker/job system with explicit cancellation, progress, completion/error messages, and document-generation IDs so stale results cannot overwrite newer edits.
- Begin with standard threads/channels and well-defined job interfaces. Introduce an async runtime, `rayon`, or other scheduler only when a concrete task and benchmark justify it.
- Avoid holding broad locks while doing disk IO or rendering. Prefer ownership transfer, immutable snapshots, small lock scope, and tile-level scheduling where appropriate.
- Closing a document cancels or safely drains its jobs before its resources are released.
- Use checked arithmetic and explicit limits for image dimensions, strides, tile counts, decompression, memory budgets, and file offsets.

### 8.6 Rust API and error conventions

- Define stable, typed module APIs. Avoid public APIs that return opaque stringly typed maps when normal enums/structs represent the domain.
- Use a structured application error enum in core modules. Crates may use `thiserror` if it simplifies error definitions; `anyhow` may be used at the executable boundary to add context, not as the domain API type.
- Distinguish expected user errors (unsupported PSD feature, invalid parameter, permission denied, out-of-memory guard, canceled operation) from programmer defects.
- Never `unwrap()` or `expect()` on user-controlled file input, network data, plugin data, or normal platform failures. Use them only for true invariants with an explanation, and prefer no panic in normal user workflows.
- Avoid `unsafe` in product crates by default. Put necessary unsafe code in the smallest wrapper module, document its invariants, and test boundary cases. CI must inventory unsafe sites.
- No background task may mutate document state without going through the document command/transaction contract.
- Use feature flags sparingly; do not create separate product builds that silently exclude core editing capabilities.

### 8.7 Rendering architecture

- Implement a deterministic CPU reference renderer before GPU acceleration.
- Use tile-based storage to avoid requiring one contiguous allocation for every large surface. Begin performance tests at 256×256 tiles; choose final size by benchmark rather than assumption.
- Represent format, channel order, alpha convention (straight/premultiplied), bit depth, color profile, and row stride explicitly.
- Separate source content from composited viewport images, thumbnails, and filter caches.
- Track dirty tiles/regions; invalidate only affected composites and previews when correctness permits.
- Bound every cache by memory and provide invalidation on edit, profile changes, display changes, and resource replacement.
- Run the CPU path in tests for known blending, transform, color, mask, and resampling cases. GPU output must meet documented tolerance and never become the only implementation for critical operations at first.
- Use wgpu only when target hardware supports it. Handle adapter/device loss and backend initialization failure by falling back to CPU or a simpler viewport path, with a clear diagnostic.
- Use GPU acceleration first for viewport display and selected operations, not every filter simultaneously.

### 8.8 UI and accessibility rules

- Keep UI functions presentational. User actions dispatch typed commands to application logic; they do not directly manipulate pixel buffers.
- Keep tool options contextual without removing access to advanced controls.
- Every long-running operation shows progress, permits cancellation when safe, and reports failure without losing the previous valid document state.
- Preserve standard keyboard navigation, shortcut remapping, text input/IME, scaling, contrast, and non-color-only status indication.
- Panel layout must be serializable, versioned, resettable, and recoverable if corrupted. Persist window/layout settings independently from project files.
- Have a safe mode that starts VEGA without third-party plugins/resources if a broken extension prevents normal startup.

### 8.9 Android and platform-specific APIs later

- Use Kotlin + Jetpack Compose for the Android UI; the screen composition, gestures, file pickers, storage permissions, lifecycle, and touch ergonomics are platform-native responsibilities.
- Reuse Rust document/editing semantics through a narrow generated/facade binding layer. Prototype UniFFI Kotlin bindings or a JNI/C ABI wrapper and choose based on clean-build reliability, call overhead, generated API clarity, cancellation support, and debuggability.
- Do not expose desktop-only APIs or C# view-model concepts to Rust core.
- Keep large pixel/tile data within Rust-owned memory where possible. Transfer viewport-sized results or direct buffers with explicitly defined lifetime; do not create repeated full-document copies through JNI.
- Handle Android process death, suspend/resume, low memory, scoped storage, content URIs, document providers, software keyboard, stylus/touch gestures, and thermal throttling.
- Use a mobile-specific UI and selected feature scope; never squeeze desktop docks and toolbars onto a phone. Android feature parity is an evidence-based goal, not an assumption.
- Validate on physical Android 11+ devices as well as emulator/CI. Do not claim tested support from compilation alone.

## 9. Proposed repository layout

```text
VEGA/
  global.json
  Directory.Build.props
  VEGA.sln
  Cargo.toml
  Cargo.lock
  rust-toolchain.toml
  src/desktop/
    Vega.Desktop/              # Avalonia UI and application startup
    Vega.Application/          # high-level workflows/services
    Vega.Presentation/         # view models and UI state
    Vega.NativeInterop/        # typed, safe LibraryImport wrapper
    Vega.Tests/
  crates/
    vega-core/                 # authoritative document/edit model
    vega-ffi/                  # stable C ABI for .NET
  android/                     # Kotlin/Compose, later phase
  backend/worker/              # TypeScript, separately deployed
  tests/fixtures/
  tests/golden-images/
  docs/
  .github/workflows/
    dotnet-ci-windows.yml
    rust-ci-windows.yml
    native-bridge-ci.yml
```

Start with the fewest C# projects and Rust crates that retain clear responsibilities. Do not create `vega-render`, `vega-formats`, `vega-kotlin-ffi`, or separate application projects on day one unless the prototype proves their boundaries helpful. Add them incrementally; the architecture diagram describes logical responsibilities rather than a demand for many packages.

`global.json` should select the currently approved .NET 11 SDK. During the release-candidate window it may select a newer available RC for experimentation, but the team must update it to .NET 11 GA before any public release. The Rust toolchain must be pinned to a reviewed stable release. `Cargo.lock` is committed for the shipping application. Keep all package versions and release tooling reproducible.

## 29. Expanded repository layout

Add folders as their phase begins. Do not create inactive scaffolding for every future feature.

```text
VEGA/
  src/desktop/                       # C# / Avalonia
  crates/                            # Rust core / bridge / optional modules
  android/                           # Kotlin + Jetpack Compose, later
  backend/
    worker/
      src/routes/
      src/middleware/
      src/auth/
      src/storage/
      src/shares/
      src/plugins/
      src/moderation/
      migrations/
      tests/
      wrangler.toml
      package.json
      tsconfig.json
    contracts/
      openapi.yaml
      schemas/
  docs/
    FORMATS/VEGA_FORMAT.md
    FORMATS/PSD_COMPATIBILITY.md
    BACKEND_SECURITY.md
    CLOUD_STORAGE_POLICY.md
    COMMUNITY_CONTENT_POLICY.md
    PLUGIN_API.md
    DOTNET_DEVELOPMENT.md
    RUST_DEVELOPMENT.md
    ADR/
  plugins/
    examples/                         # safe example resource bundles
```

Use separate local/staging/production Worker environments, separate Firebase projects where practical, isolated D1 databases and R2 buckets, and unique secrets. Untrusted pull requests must never receive production credentials or deploy production. Generate Worker binding types from the selected Wrangler configuration.

## 44. Data, command, render, and history contracts

### 44.1 Document graph

The core document is a typed graph with stable IDs. Node types include raster, text, shape, group, adjustment, embedded asset, mask, and later specialty nodes. Every node has explicit visibility, opacity, blend behavior, transform, color/alpha interpretation where applicable, and a versioned property payload. Graph edits are transactional: reject invalid graph structure rather than leaving partially mutated state.

### 44.2 Command contract

Each document-changing operation declares:

- validated input parameters and required resources;
- the target node/region and relevant preconditions;
- whether it is destructive, reversible, or non-destructive;
- the expected document changes and invalidated render regions;
- estimated resource impact and cancellation safety;
- undo/redo representation or checkpoint policy;
- a stable command ID/type for testing and optional recipe serialization.

Execute commands through one dispatcher/transaction API. For failure, preserve the last valid document state. Do not let UI widgets directly edit domain fields in arbitrary callbacks.

### 44.3 History and recovery

Undo records should store compact inverse operations, changed-tile snapshots, or explicit checkpoints rather than cloning the entire document for every action. The implementation must report or budget history memory. Large operations may checkpoint to disk. Autosave/recovery is separate from normal user save and must have a deterministic restore policy. Named checkpoints and Branchbook are later stages built on a proven history core.

### 44.4 Render request contract

Render jobs identify document ID, revision/generation, viewport, requested quality, dirty regions, color/display intent, cancellation token, and output target. Completion messages must carry the revision used to render so the UI can discard stale work. GPU/CPU results share pixel format and tolerance rules. Do not retain unbounded render requests after newer revisions supersede them.

### 44.5 Module boundaries

- `vega-core`: documents, IDs, invariants, commands, history types, semantic validation.
- `vega-render`: tile types, compositing, filters, viewport output, GPU caches.
- `vega-formats`: serialization and format adapters; it may depend on `vega-core`, but `vega-core` must not depend on PSD parser implementation details.
- `vega-app`: orchestration and UI. It can depend on the above; lower-level crates must not import UI code.
- `backend/worker`: network authority for online identity, ownership, quota, and content visibility. The local app cannot grant itself server permissions.

Avoid circular crate dependencies. Move shared types down into the narrowest crate that owns their invariant; do not solve every dependency problem by creating a generic `common` crate.

---

## 60. Document model and image-science requirements

- Color space/profile metadata must be explicit. Avoid treating all decoded bytes as sRGB without a documented conversion policy.
- Distinguish straight and premultiplied alpha at API boundaries.
- Track bit depth and channel model; do not assume all workflows can permanently reduce to 8-bit RGBA.
- The CPU renderer acts as the correctness reference until GPU behavior is proven against fixtures.
- Pixel arithmetic and resampling need fixtures for edges, transparency, color profiles, and high-contrast boundaries.
- Keep operations non-destructive when practical. A filter/adjustment node retains parameters; a destructive result must be explicit and undoable.
- Decode external files under memory and dimension limits before allocating large buffers.
- Preserve embedded source pixels, metadata, masks, and profiles when supported; compatibility reporting identifies information that cannot be represented.
- Test visual output with both exact comparisons for deterministic integer operations and bounded numerical tolerances for floating-point/GPU paths.

---

## 76. Architecture changes required for full feature scope

The full ambition includes advanced layers, selection refinement, brushes, typography, raw development, filters, compositing, PSD/PSB import, animation/video exploration, automation, and VEGA's original systems. This does not justify implementing each module before the core exists.

Required design properties:

1. **Layered but not over-modularized.** Begin with a handful of crates; split at stable ownership boundaries.
2. **Same document model everywhere.** Quick commands, advanced tools, recipes, and signature systems dispatch through the same operation/history model.
3. **CPU truth, GPU acceleration.** GPU caches can be discarded and rebuilt; unsupported GPU paths retain a correct fallback.
4. **Versioned persistence.** Native project migrations are explicit and tested; do not serialize Rust struct layouts as a long-term binary ABI.
5. **Plugin isolation.** Never load arbitrary native code into the main process. Declarative assets first; WebAssembly sandbox later with explicit host capabilities, resource budgets, and revocation.
6. **Backend separation.** The TypeScript Worker owns online authorization and quota; desktop Rust remains a client, not the authority.
7. **Format adapters.** PSD/PSB-specific parsing stays outside the core graph; unsupported features map to explicit compatibility diagnostics.
8. **Operational observability without surveillance.** Local logs help diagnose crashes without collecting image content or leaking sensitive paths. Telemetry remains opt-in if ever introduced.
9. **Platform validation.** Shared source does not guarantee equal support. Windows, macOS, and Android need target-specific builds and real-device validation.
10. **Change the plan only with evidence.** UI framework replacement, new runtimes, and more dependencies require an ADR based on prototypes, tests, license review, or measured limitations.

## Official technical references

- Avalonia supported platforms: https://docs.avaloniaui.net/docs/supported-platforms
- Avalonia .NET 11 support tracker: https://github.com/AvaloniaUI/Avalonia/issues/22011
- .NET releases/support: https://dotnet.microsoft.com/en-us/platform/support/policy/dotnet-core
- .NET `LibraryImport` source generation: https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke-source-generation
- UniFFI Rust bindings: https://github.com/mozilla/uniffi-rs
- Jetpack Compose: https://developer.android.com/develop/ui/compose
- wgpu (candidate for a later measured graphics path): https://wgpu.rs/
- Cargo workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Clippy: https://doc.rust-lang.org/clippy/
- Cloudflare Workers TypeScript: https://developers.cloudflare.com/workers/languages/typescript/
- Cloudflare Workers Rust (reference only; not the selected backend): https://developers.cloudflare.com/workers/languages/rust/
