# Kotlin / Android Development Rules

> VEGA Planning Set v2.1 · Deferred until the Windows desktop foundation is stable.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [C#/.NET desktop rules](dotnet-development.md) · [Rust core rules](rust-development.md)

## 1. Purpose and release order

Android 11+ is a planned follow-on platform, not part of the initial Windows release. Use Kotlin and Jetpack Compose for a native, touch-first interface. Reuse the Rust document/editing core where it fits and has been validated. Do not build Android now at the cost of delaying the Windows editor.

## 2. Platform responsibilities

Kotlin/Android owns Compose UI, activity and process lifecycle, permissions, Android intents, content URIs, the Storage Access Framework, notifications if ever needed, input methods, gestures, system bars, haptics, accessibility semantics, and mobile-specific navigation. Rust owns shared edit semantics and pixel processing. Do not expose Avalonia controls or C# view-model structures to Android.

## 3. Native core integration

Prototype a small binding layer before choosing a permanent route:

- Evaluate UniFFI's Kotlin bindings for typed API generation, resource lifecycle, and async/job behavior.
- Compare against a JNI/C ABI facade only if it solves a demonstrated limitation or gives a materially simpler build/debug cycle.
- Keep one set of core command semantics. Android may have a facade tailored to Kotlin idioms, but operations must not drift from desktop behavior.
- Keep pixel tiles and large buffers in native-owned memory where possible. Use explicitly scoped direct buffers or viewport-sized render results with documented ownership; avoid copying the full image on every gesture/frame.
- Version the binding API, test startup/version mismatch, error mapping, cancellation, create/dispose cycles, buffer release, and malformed input.
- Never allow native panics to cross the boundary or let native crashes be misreported as recoverable UI exceptions.

Official starting reference: [Mozilla UniFFI](https://github.com/mozilla/uniffi-rs).

## 4. Touch-first editing UX

- Use gesture-aware canvas navigation: pinch zoom, two-finger pan, optional rotation only when the document/tool model supports it, and clear handoff between canvas gestures and UI scrolling.
- Tool controls must be reachable with fingers and stylus. Provide configurable pressure/tilt support where the device and selected API expose it.
- Use contextual bottom/side sheets only for mobile workflows; retain the single-document model and shared command semantics.
- Never shrink the desktop dock layout to fit a phone. Select an intentional subset of tools per release and disclose unavailable tools clearly.
- Support keyboard and mouse on tablets/Chromebooks where feasible, without making them the only usable input.
- Preserve the same native `.vega` semantics when a feature is supported on both platforms; report unsupported features rather than flattening them silently.

## 5. Lifecycle, storage and memory

- Support Android 11+ as the stated minimum and test current Android versions as well.
- Handle process death, low-memory callbacks, backgrounding/resume, interrupted saves, permission denial, provider-backed URIs, insufficient storage, and document access revocation.
- Use the Storage Access Framework for user-selected files; do not assume arbitrary filesystem paths.
- Autosave/recovery checkpoints must be bounded and atomic. A lifecycle callback cannot synchronously block the UI while serializing a large project.
- Do not keep an entire copy of every large image in Kotlin memory. Define budgets and use Rust/tile-store ownership where practical.
- Cloud sync remains optional; local open/edit/save/export must work without signing in.

## 6. Build and test gate

When Android implementation begins:

- Add a versioned Gradle project only then; pin Kotlin, Gradle, Android Gradle Plugin, compile SDK and Rust targets after verifying compatible versions.
- Build debug and release variants on CI with the NDK/native Rust artifacts for each supported ABI.
- Test on a physical Android 11+ device and a current Android device; emulator-only coverage is insufficient for memory, stylus, lifecycle, and input behavior.
- Include Kotlin unit tests, Compose UI tests for core journeys, native binding contract tests, malformed-document tests, and lifecycle/recovery tests.
- Test install/update/uninstall behavior, Android scoped storage, app-private cache cleanup, and signing-key handling.
- Keep Android CI separate from Windows desktop CI to make platform failures attributable.

## 7. Definition of done

An Android task is done only when it passes the relevant Kotlin/native tests, uses the same core operation semantics as desktop where supported, handles lifecycle and permission failure, remains touch-usable on a real device, does not copy high-resolution pixels unnecessarily, and states any desktop feature that is not supported yet.
