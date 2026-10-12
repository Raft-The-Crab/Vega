# ADR-0001: Rust-first desktop application and editing core

- **Status:** Superseded by ADR-0003 (historical record).
- **Date:** 2026-10-12
- **Supersedes:** C# / Avalonia desktop application + C++ editing engine + stable C ABI.

## Context

VEGA is being developed with AI-assisted coding, and the earlier split required three application languages and a C#↔C++ interop boundary before any feature could ship. The product still needs a highly custom desktop workspace, heavy image processing, Windows-first release, later macOS and Android options, and a proprietary distribution policy.

Rust's ownership/type system and Cargo workspace can keep most first-party client code within one compiler-checked codebase. It does not guarantee faster development automatically: native GUI polish, color science, PSD/PSB support, memory budgeting, and performance still require prototypes and tests.

## Decision

1. Use Rust for the desktop app, document model, command/history system, editing operations, serialization orchestration, rendering, and client services.
2. Prototype `egui` + `eframe` and `egui_dock` for the unified workspace.
3. Use `wgpu` as the potential GPU viewport/compute abstraction, added in stages with a CPU correctness reference.
4. Avoid a first-party C/C++ engine and C ABI. FFI is allowed only for a demonstrated platform/codec need and is isolated behind a small safe wrapper.
5. Start with a few crates (`vega-app`, `vega-core`, `vega-render`, `vega-formats`) and split further only with a clear ownership/test reason.
6. Keep TypeScript Cloudflare Worker deployment outside the Rust client workspace. This is an explicit separate-service decision.
7. Validate the UI decision on Windows before committing to broad implementation. If essential acceptance criteria fail, create an ADR comparing alternatives before a framework switch.

## Consequences

### Positive

- Removes the main C#↔C++ application interop boundary.
- Allows shared types and operation contracts without a generated C ABI.
- One core language simplifies refactors and code-agent context for the client.
- Rust is a plausible fit for tiled pixel processing, checked parsing, and explicit resource ownership.

### Trade-offs and risks

- Rust GUI ecosystem and accessibility/platform conventions need practical validation.
- `egui` is immediate-mode/custom drawn; dock layouts, text input, accessibility, keyboard behavior, and visual polish need explicit work.
- Some codecs/platform services may depend on native libraries and unsafe wrappers.
- Android may need platform-specific glue; shared source does not promise identical UX or behavior.
- The project remains vulnerable to overengineering if too many crates/dependencies are added at once.

## Acceptance gate

Phase 0 must demonstrate a Windows 11 app with a unified window, dockable panels, a central image canvas, zoom/pan, high-DPI correctness, working text input and shortcuts, safe open failure, and successful CI. The prototype also records build time, startup responsiveness, UI responsiveness, dependency licenses, and known accessibility gaps.

## References

- egui/eframe: https://github.com/emilk/egui
- egui_dock: https://docs.rs/egui_dock/latest/egui_dock/
- wgpu: https://wgpu.rs/
- Cargo workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
