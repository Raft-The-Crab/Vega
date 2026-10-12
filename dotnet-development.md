# C# / .NET 11 Desktop Development & Vibe-Coding Rules

> VEGA Planning Set v2.1 · Applies to desktop UI, view models, native bridge and desktop integration.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Rust core rules](rust-development.md)

## 1. Purpose and boundaries

Use C# on .NET 11 for the Windows-first desktop application and the later macOS desktop build. Use Avalonia UI with XAML, styles, templates and MVVM to create a coherent, polished editor interface. Rust owns the authoritative document and editing engine. C# must not implement a second image engine or maintain a separately mutable layer graph.

Kotlin + Jetpack Compose is the Android UI. TypeScript Cloudflare Workers remains the separate online backend. Do not migrate those responsibilities into the desktop client just because C# can technically do so.

## 2. .NET 11 lifecycle and SDK pinning

.NET 11 is the intended target framework, but its release status must be checked at the start of implementation and release preparation. On 2026-10-12, official Microsoft pages indicate .NET 11 is at Release Candidate 1, with GA expected in November 2026; the release policy lists RC1 end-of-support on October 13, 2026. Therefore:

- Use the installed .NET 11 SDK for an isolated prototype if needed, but don't treat the current RC as the final distribution runtime.
- Pin the exact SDK in `global.json`; update it explicitly as newer RCs/GA are released. Never silently float to a preview SDK in CI.
- Before public shipping, use .NET 11 GA plus the latest servicing patch and verify Avalonia, native packaging and all workloads/dependencies.
- Recheck Avalonia's official .NET 11 support issue/milestone at Phase 0. If compatibility is blocked, don't patch the framework internally or hide compiler errors. Either wait for a compatible Avalonia release or record a temporary prototype-only target as an ADR.
- Target-specific runtime and package version upgrades are focused changes with builds/tests, not random package updates mid-feature.

References: [What's new in .NET 11](https://learn.microsoft.com/en-us/dotnet/whats-new/), [.NET support policy](https://dotnet.microsoft.com/en-us/platform/support/policy/dotnet-core), [Avalonia supported platforms](https://docs.avaloniaui.net/docs/supported-platforms), [Avalonia .NET 11 tracker](https://github.com/AvaloniaUI/Avalonia/issues/22011).

## 3. UI structure and visual quality

- Use XAML views, shared theme resources, design tokens, templated controls and MVVM for the standard screen/panel architecture.
- Keep code-behind small and purposeful. Use it for unavoidable visual-tree, pointer-capture, custom canvas or OS integration tasks; put workflows in commands/application services.
- Do not create one giant `MainWindow`, one global service locator, or a single view model containing the entire app.
- Build the editor as purposeful controls: `VegaCanvasControl`, tool options, layer tree, properties inspector, history, assets, dialogs, status bar, job center, and command search.
- Keep the UI unified. Quick actions, menus, keyboard shortcuts, command search and advanced controls must invoke the same operation APIs.
- Store user workspace layout separately from `.vega` project data. Layout corruption or a missing panel must recover to defaults.
- Define control states (normal, hover, focus, pressed, selected, disabled, loading, error), accessible names, keyboard behavior and automation IDs for important interactive controls.
- Test high DPI, mixed-DPI monitors, resizing, text/IME composition, keyboard navigation, contrast modes, focus traversal, touch/stylus/mouse capture, context menus and window restoration.
- Never imitate a polished UI with nonfunctional buttons, hardcoded mock data presented as real, or status messages that claim an operation succeeded when it did not.

## 4. Avalonia and docking policy

Avalonia is the selected starting framework, not permission to skip validation. Its basic `DockPanel` is an edge-layout panel; it is not assumed to provide arbitrary detachable IDE-style docking. In Phase 0:

1. Check the exact Avalonia release's .NET 11 compatibility and XAML compiler.
2. Evaluate a candidate docking library only after verifying current maintenance, licensing, compatibility, accessibility and sample build behavior.
3. If there is no suitable candidate, start with document tabs and fixed/resizable left/right/bottom panel zones. Design panel interfaces so richer docking can be added later without rewriting tool/document logic.
4. Prototype the image canvas with realistic input routing and a large image; don't decide by looking at a static mockup alone.
5. The owner must inspect the running prototype and approve the UI direction before broad UI work.

Do not add multiple UI frameworks to avoid making the choice. The desktop solution uses Avalonia; Android uses native Compose because its form factor and lifecycle differ significantly.

## 5. MVVM and state boundaries

- **View:** XAML composition, visuals, routed input, focus and automation properties.
- **View model:** presentation state, commands, validation, busy/progress state, and read-only document projections.
- **Application services:** open/save/export workflows, preferences, recent files, cloud calls, native-core session lifetime, background job orchestration.
- **Rust core:** authoritative document graph, layer/object IDs, commands, history, edit operations and pixel content.
- **Native wrapper:** `Vega.NativeInterop` hides raw ABI calls behind safe C# interfaces and typed result objects.

Avoid chatty native calls caused by binding updates. Batch metadata and viewport requests; debounce properties previews where safe; keep pixel buffers out of property notifications; cancel stale previews based on document revision/generation IDs.

## 6. Rust bridge rules for C#

- Use the explicitly versioned C ABI defined in `architecture.md`; import functions through `LibraryImport` source generation where supported by the selected SDK.
- Create one safe wrapper layer. View models do not call raw native exports directly.
- Define API version checks, engine/document handle lifecycle, thread-affinity, error/status mapping, UTF-8 handling, buffer ownership/free methods, cancellation, and progress semantics.
- Never let a Rust panic unwind across the ABI. Convert errors to documented result/status values.
- Do not transfer full-resolution images over the bridge for every frame. Request visible viewport pixels/tiles, use bounded caches, and profile the interop cost.
- Never keep borrowed pointers after a native call unless the contract explicitly provides a retained handle with safe lifetime semantics.
- Fail gracefully when native binary load fails or ABI versions mismatch; show actionable repair diagnostics and do not corrupt document files.
- Every bridge change requires contract tests for success, bad inputs, native errors, memory/buffer release, repeated create/destroy, cancellation and version mismatch.

Official reference: [.NET P/Invoke source generation](https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke-source-generation).

## 7. Async work and UI responsiveness

- No full-size decode/filter/export, PSD parse, native render, cloud operation or plugin scan on the UI thread.
- Represent long operations as jobs with progress, cancellation, document generation, and structured completion/error results.
- Use `CancellationToken` consistently for cancellable .NET workflows and map cancellation to the Rust API contract.
- Do not silently abandon background tasks during document close or app exit. Cancel/drain safely or persist a recoverable job state where appropriate.
- A stale background result must not overwrite newer edits or the preview for a newly selected layer.
- Keep dispatch/synchronization context behavior explicit around Avalonia UI updates.

## 8. C# coding and dependencies

- Enable nullable reference types and analyzers. Prefer explicit types at native/resource boundaries and ordinary idiomatic C# for simple view model code.
- Use structured result/error types; don't swallow exceptions with empty `catch` blocks. Show concise errors in UI and log technical context without leaking access tokens, secrets, or sensitive file contents.
- Avoid `async void` except required UI event handlers; route real logic through awaitable commands/services.
- Avoid reflection-heavy frameworks or source generators added solely for novelty. Prefer a small number of maintainable dependencies.
- Do not suppress analyzer/compiler diagnostics merely to make a generated implementation compile. Explain any narrowly scoped suppression.
- Every third-party package needs license, maintenance, vulnerability, compatibility, and transitive-dependency review. Central Package Management may be introduced after initial package choices settle.

## 9. Build/test baseline

After the solution exists, CI should run the commands supported by the selected SDK/tooling, typically:

```powershell
dotnet --info
dotnet format VEGA.sln --verify-no-changes
dotnet build VEGA.sln -c Release
dotnet test VEGA.sln -c Release
```

Run analyzers as part of build, test the native library on the matching OS/architecture, and run UI automation/manual smoke tests on a real Windows desktop. If Avalonia's UI automation stack requires a separate test project or runner, document it rather than pretending `dotnet test` alone covers visual behavior.

## 10. Vibe-coding task loop

1. Read `plan.md`, this file, `architecture.md` and the relevant UI/domain doc.
2. Inspect the existing solution, branches/diff, build output, tests, dependency versions and native artifacts.
3. Write the smallest task scope and acceptance criteria before editing.
4. Make one coherent change. Don't bundle a redesign with a native ABI redesign and a new editing feature.
5. Build early, run focused tests, then relevant full checks.
6. Inspect the diff for stubs, accidental file deletions, fake success paths, unbounded tasks, secrets and unrelated formatting.
7. Report only commands actually run and outcomes observed. Explicitly state which OS/device/manual UI tests were not run.

## 11. Definition of done

A desktop task is done only when its behavior works, appropriate automated tests pass, UI states/errors/accessibility are defined, relevant Rust bridge behavior is covered if touched, no placeholder behavior remains, and the result is documented with any known limitations.
