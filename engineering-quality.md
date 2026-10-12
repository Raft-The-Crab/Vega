# Testing, Performance & Engineering Quality

> VEGA Planning Set v2.1 · Category document
> Automated verification, benchmarks, performance budgets, and release-quality rules.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [10. Testing and engineering quality](#10-testing-and-engineering-quality)
- [50. Performance budgets and benchmark protocol](#50-performance-budgets-and-benchmark-protocol)
- [51. Quality strategy and release gates](#51-quality-strategy-and-release-gates)
- [77. Performance and stability engineering for expanded scope](#77-performance-and-stability-engineering-for-expanded-scope)
- [78. Expanded verification and acceptance program](#78-expanded-verification-and-acceptance-program)

---

## 10. Testing and engineering quality

A task is not complete because the UI control appears or the code looks plausible. The relevant behavior must be verified.


### Multi-language validation baseline

For Rust core changes, CI runs `cargo fmt --all -- --check`, `cargo check --workspace --all-targets`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`. For desktop changes, run `dotnet format VEGA.sln --verify-no-changes`, `dotnet build VEGA.sln -c Release`, `dotnet test VEGA.sln -c Release`, and the selected analyzers. Keep `Cargo.lock` and approved .NET package versions reproducible. Add advisory/license tools after configuration and false-positive policies are reviewed; a green scanner does not replace dependency review. Native bridge changes require ABI contract tests plus create/dispose/error/buffer-lifetime tests on the target OS.

### Test layers

- **Unit tests:** layer invariants, geometry, transforms, selection behavior, undo commands, document versions, blend math, color operations.
- **Golden-image tests:** known inputs and expected outputs with explicit pixel/color tolerances.
- **Round-trip tests:** write `.vega`, terminate/close, reopen, compare editable structure and rendered output.
- **PSD tests:** supported-feature matrix, unsupported-feature reports, preview comparisons, malformed/truncated fixtures.
- **Module-boundary tests:** Rust crate contracts, C#↔Rust C ABI, Android binding API, buffer dimensions/strides, ownership/lifetime behavior, error paths, cancellation, ABI version mismatch, and repeated create/dispose cycles. Native-boundary bugs receive dedicated tests.
- **UI tests:** open/save/export, layer ordering, dirty state, command search, keyboard navigation, recovery flow.
- **Security/resource tests:** malformed containers, oversized dimensions, decompression limits, invalid offsets/lengths, untrusted paths, memory exhaustion protection.
- **Performance tests:** startup, open/save, fit/100% zoom, panning, brush latency, layer composite, cache eviction, memory under large documents.
- **Regression tests:** each fixed bug that is practical to reproduce gains a permanent test.

### Performance target policy

Do not invent universal speed promises before benchmarking. Select a reference machine and a documented fixture set. Establish targets for UI responsiveness, canvas update latency, startup, save/open time, memory, and cancellation after the first working rendering path exists. Measure p50/p95 where useful, not just one best run.

Suggested initial validation fixtures: a 12-megapixel photo, a multi-layer 4K composition, a deep layer tree, transparent large images, a wide panorama, a document with many masks, and a case with expensive filters. These are validation cases, not a promise that the initial beta can edit every such file without limits.

---

## 50. Performance budgets and benchmark protocol

Exact numeric targets must be calibrated against a stated baseline computer and reference fixture set; initial numbers in this section are hypotheses, not promises.

### 50.1 Baseline test machine and fixtures

Choose a representative Windows 11 computer (CPU model, RAM, GPU/driver, disk, display scale) and keep a second lower-end system to expose memory/performance assumptions. Use legally distributable reference images/projects with defined dimensions, layers, bit depth, masks, and filters. Record cold start separately from warm start.

### 50.2 Performance categories

- Startup and document open.
- Zoom, pan, and canvas input latency.
- Brush stroke feedback and commit duration.
- Selection and transform preview/commit.
- Adjustment preview and final render.
- Save, open, autosave, and recovery.
- PSD parse/import and compatibility reporting.
- Memory growth and undo-history cost.
- Export duration/quality.
- UI responsiveness during background tasks.

### 50.3 Performance engineering rules

- Profile before optimizing; save reproducible traces and fixture IDs.
- No expensive file parsing, thumbnail generation, image processing, or network calls on the UI thread.
- Avoid full-document clones for common edits; use tile deltas, snapshots, or operation-specific history as appropriate.
- Maintain a memory budget and degrade gracefully when approaching it. Do not catch an out-of-memory error and continue in an undefined document state.
- Cache derived renders and invalidate by dependency; never persist cache as the only source of truth.
- GPU acceleration is introduced behind capability detection and validated against CPU reference outputs. Software fallback is required where practical.
- Keep a benchmark history per release so regressions are measurable.

---

## 51. Quality strategy and release gates

### 51.1 Test pyramid

- **Unit tests:** command validation, geometry, compositing math, project schema, token/metadata validators, quota arithmetic.
- **Property/fuzz tests:** file parsers, geometry/path operations, native API boundary, archive parsing, and serialization invariants.
- **Integration tests:** Rust application/core integration, D1 migrations/quota reservations, R2 upload lifecycle, Firebase token verification using test credentials, and account authorization.
- **Golden image tests:** controlled fixtures for rendering, blend modes, masks, color conversions, export, and import.
- **UI automation:** top user journeys, accessibility names, panel reset, keyboard commands, and failure states.
- **End-to-end staging:** account creation, cloud save, quota exceeded, public publish/takedown, plugin install/disable, and account deletion.
- **Manual usability:** beginners complete common tasks; professionals review precision, shortcut behavior, panel flexibility, color accuracy, and file interoperability.

### 51.2 Release gates

- **Prototype:** must build/run and demonstrate only the scoped spike; no claims of completeness.
- **Internal alpha:** core document and save/open tests pass; unsupported features clearly scoped.
- **Private beta:** crash recovery, basic security, privacy notice, file parser threat testing, signed packages, and reproducible builds are ready.
- **Public editor beta:** core editing quality and support process exist; reported feature list matches the app; user artwork isn't silently uploaded.
- **Cloud beta:** token verification, ownership, exact quota, rate limits, deletion, logs, backups/restore, and operational alerts pass.
- **Public uploads/plugins:** moderation and takedown, file validation, cost/abuse alerts, tested kill-switch, plugin isolation, and incident-response drills pass.
- **Production:** signed installer/updater, SBOM/notices, release rollback, data recovery, privacy terms, support contact, and vulnerability disclosure process are ready.

### 51.3 Definition of done across product and backend

A feature is done only when it is implemented end-to-end, all advertised UI paths work, validation/error behavior exists, relevant tests pass, data is correctly saved/reopened or server state is correctly persisted, accessibility is considered, security/privacy implications are reviewed, and the release note states any remaining limitation. “Compiles” is necessary but not sufficient.

---

## 77. Performance and stability engineering for expanded scope

### 77.1 Reference hardware and project corpus

Before claiming performance readiness, select named baseline systems for minimum supported Windows 11 hardware and a recommended creative workstation. Record CPU, RAM, GPU, storage, display resolution/scale, OS build, drivers, and configuration. Use synthetic as well as realistic fixture projects: single photo, layered design, many-layer composite, large panorama, brush-heavy project, high bit-depth project, linked assets, and a large PSD import.

The project corpus must contain licensing/ownership metadata. Never use private customer artwork as an undisclosed benchmark fixture. Every regression test records the exact project version and content hash.

### 77.2 Performance budgets

Set budgets after initial prototypes, but require them to cover:

- Cold startup and time to usable first window.
- Open/import time by pixel count, layer count, and file format.
- Canvas interaction latency and time to refresh after selection/transform.
- Brush input-to-visible-stroke latency and gap behavior.
- Common adjustment preview latency and full-resolution commit time.
- Undo/redo time and memory growth across realistic editing sessions.
- Save/autosave time, recovery time, and worst-case temporary storage usage.
- Large export time and cancellation responsiveness.
- Peak RAM/GPU memory by project profile, including safe behavior at budget boundaries.
- Idle CPU/GPU use, thermal/power behavior, and background indexing/caching.

Report median and tail percentiles, not just a single best-case time. Performance regressions should have a threshold and triage process. A feature can be implemented correctly yet not be releasable if it makes the application freeze or exhaust memory on its declared support target.

### 77.3 Large-document safeguards

Validate width, height, pixel count, number of layers/nodes, recursion depth, compressed expansion ratio, embedded payload sizes, and aggregate linked-asset bytes before full decode/allocation. Prefer 64-bit safe-size arithmetic and checked allocations. Use tile-based allocation and stream where possible. Offer proxy/preview rendering when a full-resolution operation is costly, but never export the proxy as though it were the full-resolution result.

### 77.4 Failure recovery

Crash injection and forced process termination should be routine tests. Validate autosave recovery for open documents, recovery of a save interrupted mid-write, stale temporary cleanup, corrupt autosave quarantine, and recovery when disk space is exhausted. Recovery must never overwrite the original source without explicit user choice. Logs and recovery data are local by default.

---

## 78. Expanded verification and acceptance program

### 78.1 Feature-development template

Every feature task must include:

- User story and a short example of the desired workflow.
- Supported document/layer types and color modes.
- UI entry point plus keyboard/command-search discovery.
- Command/model changes and native-file serialization impact.
- Undo/redo expectations and transaction grouping.
- Render/cache invalidation behavior.
- Error cases, cancellation rules, and resource limits.
- Accessibility/keyboard behavior and localization needs.
- Security implications, especially when parsing user files or executing plugin/automation code.
- Unit/integration/visual fixture tests and a manual verification recipe.
- Performance baseline and expected resource usage.
- Compatibility report behavior where imported/exported formats are affected.
- Documentation/help and release-note needs.

### 78.2 Visual regression tests

Use deterministic reference scenes for compositing, masks, text, transforms, edge rendering, gradients, resampling, and each supported blend mode. Store reference parameters and expected tolerances. Compare output in the intended color domain; do not blindly compare differently profiled encoded bytes. Any intentional visual change requires an updated reference plus a review note explaining why.

### 78.3 Property and fuzz tests

Fuzz file parsers, decompression, project migrations, layer-graph traversal, filter parameter validation, brush preset import, PSD blocks, image metadata, font metadata parsing, plugin package manifests, and cloud upload metadata. Add property-based tests for transform inverses, selection bounds, alpha invariants, command undo/redo round-trip, serialization stability, and migration idempotence where meaningful.

### 78.4 Manual end-to-end suites

Maintain a small must-pass suite:

1. Create a blank document, draw, adjust, save, close, reopen, and export.
2. Import a photograph, crop, apply a masked adjustment, retouch, and export.
3. Build a multi-layer composition with text, shape, clipping, group, opacity, and effects.
4. Create/refine a mask and edit only the targeted subject/background region.
5. Open PSD samples across supported complexity levels and review compatibility reports.
6. Cancel import, filter, batch, cloud upload, and export operations at multiple stages.
7. Recover after forced termination during editing and during save.
8. Run a batch recipe on varied dimensions and formats, including failure cases.
9. Install a valid resource pack and a sandboxed test plugin; deny permissions; revoke and uninstall it.
10. Disable internet access and verify all core local workflows remain functional.

### 78.5 Release confidence rules

No feature is release-ready just because the happy path works once. Critical P0 features require automated tests and manually validated recovery. File parsing/security-sensitive features require adversarial test coverage. Interoperability claims require fixtures and written support limits. A failed critical check blocks the release or requires an explicit, documented risk decision by the product owner.
