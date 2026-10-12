# Rust Core Development & Vibe-Coding Rules

> VEGA Planning Set v2.1. These rules govern the shared Rust document/editing core and its native bindings; the desktop UI follows `dotnet-development.md`.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [C#/.NET rules](dotnet-development.md) · [Engineering quality](engineering-quality.md)

## 1. Rust core goals

Rust is the default language for VEGA's shared, performance-sensitive editing core: canonical document state, operations, history semantics, pixel/tile processing, CPU compositing, image-format parsing, `.vega` serialization, and renderer scheduling. Rust is not the UI language for the Windows/macOS desktop; C#/.NET + Avalonia owns that interface. Kotlin/Compose owns Android presentation. TypeScript owns the separate Cloudflare Worker.

Intentional boundaries:

- **C#/.NET 11:** desktop views, view models, command presentation, window lifecycle, dialogs, preferences, accessibility surface, and cloud client UX.
- **Kotlin:** Android lifecycle, Compose views, permissions, content URIs, touch/keyboard behavior.
- **TypeScript:** independently deployed cloud backend.
- **C ABI / P/Invoke:** desktop bridge into Rust; keep the ABI narrow, versioned, and boring.
- **UniFFI or JNI/C ABI facade:** Android bridge selected by prototype and maintenance review.

Do not add C++ to the core by default. A third-party library may contain native code if there is a real need and license, platform, maintenance, security, and packaging reviews pass.

## 2. Vibe-coding workflow

For every task, the coding agent must:

1. Read `plan.md`, this file, and the relevant domain specification before editing.
2. Inspect the current repository, current branch/status, module APIs, tests, and build scripts. Do not assume paths or recreate files blindly.
3. State a tiny implementation plan in the task/commit description. Identify the expected behavior and acceptance test.
4. Make one coherent change. Avoid mixing a UI redesign, file-format rewrite, new dependency, and feature implementation in a single patch.
5. Compile early, run narrow tests, then the workspace checks. Fix root causes rather than suppressing diagnostics.
6. Inspect the diff for accidental deletions, placeholders, `todo!()`, `unimplemented!()`, debug UI, secrets, and unrelated formatting churn.
7. Report only commands actually executed and outcomes actually observed. Explicitly name skipped tests and known limitations.

A change is not complete because it compiles once. It must satisfy the feature's acceptance criteria, preserve existing behavior, handle errors, and include tests at the appropriate level.

## 3. Cargo workspace and reproducibility

- Commit the root `Cargo.toml`, `Cargo.lock`, and a reviewed `rust-toolchain.toml` for the application.
- Pin the selected stable Rust toolchain after the Phase 0 prototype is verified; avoid stale copied versions from tutorials.
- Use workspace dependency declarations only for shared dependencies that have been approved. Do not centralize every crate setting without a real need.
- Do not run blanket latest-version upgrades during feature work. Upgrade dependencies in focused PRs with changelog review, license scan, vulnerability scan, and regression tests.
- Keep default features lean. Large/optional capabilities such as GPU compute, RAW, video, OCR, or executable plugin runtime must not silently enter the initial build as incidental transitive dependencies.
- Add a new crate only when it has a clear responsibility, owner, boundary, and tests. Keep internal logic in modules until splitting genuinely helps.
- The build must work from a clean checkout with documented prerequisites and no dependency on a developer's local absolute paths.

## 4. Minimum local validation commands

Once a Cargo workspace exists, the default baseline is:

```powershell
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Run relevant focused tests during iteration, then the complete baseline before completion. If a project-specific lint or target cannot run locally, record that clearly and rely on a CI job that actually executes it. Do not claim a Windows/macOS/Android target is validated just because a cross-target `cargo check` passes.

Security/dependency tools are introduced in Phase 0 and locked in CI after their configuration is understood. Candidate checks include `cargo audit` and `cargo deny`; they complement, rather than replace, manual license review, crate maintenance review, and parser fuzzing. Avoid adding tools solely to create a large checklist.

## 5. API, type, and error conventions

- Use domain types and enums for document operations, layers, masks, blend modes, color models, file compatibility outcomes, and errors.
- Avoid stringly typed command dispatch, broad `serde_json::Value` domain payloads, giant `AppState` types, and global mutable singletons.
- Do not pass raw pixel pointers around the app or across a language boundary without a written contract. Represent buffers/tiles with explicit width, height, stride, pixel format, alpha convention, color profile, and ownership. Prefer viewport-sized results/opaque buffer handles to repeated full-image copies.
- Keep UI presentation state distinct from the authoritative document state.
- Use structured errors. Preserve context for logs; show concise actionable messages in the UI; never expose tokens, secrets, or full sensitive paths in user-facing errors by default.
- `unwrap()`/`expect()` must not be used on user-controlled data, file parsing, network responses, plugin inputs, or normal OS errors. Invariant-only use needs a clear explanation.
- Catch and classify expected failures. A corrupt/unsupported file must not crash or panic the whole application.
- Use explicit revisions/generation IDs for background results so stale jobs cannot overwrite newer edits.

## 6. Unsafe Rust policy

- Prefer safe Rust and existing safe APIs.
- Enable workspace warnings for unsafe usage and use `#![forbid(unsafe_code)]` in crates where practical.
- Any necessary unsafe code must be isolated in a small module, document invariants and caller requirements, and provide safety-focused tests.
- Never write a home-grown parser with unchecked offsets or unchecked arithmetic for hostile binary file formats.
- Dependencies that contain unsafe code require an actual need, maintenance review, vulnerability review, license review, and a wrapper that keeps unsafe assumptions out of business logic.
- FFI is not the default engine boundary. Use it only for proven platform APIs or dependencies where no practical safe Rust alternative exists.

## 7. Async, threading, and UI responsiveness

- Never block the UI frame while loading a large image, saving, importing PSD, running a large filter, generating previews, or contacting a cloud endpoint.
- Start with one bounded job abstraction: operation ID, progress updates, cancellation, completion result, and document revision.
- Do not introduce multiple async runtimes or thread pools by default. Choose one approach for a demonstrated use case and document the choice.
- Prevent unbounded queuing of full-resolution jobs; apply backpressure, cancellation, and memory budgets.
- Don't use unsafe shared mutable image buffers to chase early performance. Measure first and use clear ownership or tile scheduling.

## 8. Automated code-generation guardrails

Coding agents must not:

- generate fake implementations that return success without doing work;
- replace full subsystems with stubs to make a build pass;
- swallow errors with empty `catch`-equivalent branches or `let _ = ...` where failures matter;
- add large dependencies without explaining the need and license;
- silently change project format schemas, API contracts, or undo semantics;
- run formatters across unrelated files without a reason;
- delete tests that fail without providing a replacement and explaining the behavior change;
- claim manual visual verification when no screenshot or runtime check occurred;
- store credentials, Firebase service-account material, Worker secrets, signing keys, or private tokens in the repository;
- make the public share quota enforcement a client-only feature.

## 9. Native boundary rules

- Treat the .NET C ABI and Android bindings as versioned product interfaces. Keep the Rust API and each foreign-language facade covered by contract tests.
- Use opaque handles and explicit create/destroy functions for engine/document sessions. Every returned buffer/string has one documented owner and matching release function.
- Define ABI version negotiation, UTF-8 rules, error/status codes, cancellation and progress semantics, and thread-safety guarantees before feature implementation.
- Avoid passing Rust-owned complex structs across the C ABI. Use fixed-width primitives and stable C-compatible structs only where necessary; use versioned serialized DTOs for low-frequency complex commands if that makes the contract clearer.
- Never pass UI objects, callbacks with unclear lifetimes, or full-size image buffers every frame. Use bounded viewport rendering and tile/buffer handles.
- Do not let Rust unwind/panic cross a foreign ABI boundary. Convert expected failures to status/result data and keep panic containment policy explicit.
- C# uses the official .NET source-generated `LibraryImport` pathway where appropriate; do not hand-maintain duplicate marshaling declarations without tests.
- Android may use UniFFI's Kotlin bindings or a JNI/C ABI wrapper. Decide after building the smallest call-and-return prototype; keep the facade API aligned with core command semantics.

## 10. Definition of done

Every task records:

- intended behavior and scope;
- code paths changed;
- acceptance criteria satisfied;
- tests added/changed;
- validation commands and real outcomes;
- dependency/format/API changes, if any;
- known limitations and follow-up work.

## Official references

- Cargo workspaces: https://doc.rust-lang.org/cargo/reference/workspaces.html
- Rust Book: https://doc.rust-lang.org/book/
- Clippy: https://doc.rust-lang.org/clippy/
- Rust security advisories tool: https://github.com/RustSec/rustsec/tree/main/cargo-audit
- cargo-deny: https://embarkstudios.github.io/cargo-deny/
