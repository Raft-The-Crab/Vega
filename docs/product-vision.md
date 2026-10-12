# Product Vision & Decisions

> VEGA Planning Set v2.1 · Category document
> What VEGA is, who it serves, and the policies that define the product.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [1. Executive decision summary](#1-executive-decision-summary)
- [2. Product definition and target users](#2-product-definition-and-target-users)
- [6. Free and proprietary product policy](#6-free-and-proprietary-product-policy)

---

## 1. Executive decision summary

VEGA is a proprietary, free-to-use image-editing application. Its long-term ambition is to cover the major workflows and professional capabilities associated with Photoshop while providing a more approachable user experience.

The ambition is **capability parity over time**, not a promise that the first release will duplicate every Photoshop feature. VEGA must develop and publish a concrete compatibility matrix rather than advertise complete parity before it has been tested.

### Decisions confirmed by the product owner

1. **One unified workspace.** There are no separate Quick Studio and Pro Studio modes. Beginners and professionals use the same editor, document, tools, panels, history, and file formats.
2. **Completely free.** No purchase price, subscription, paid tier, advertisements, or locked core editing tools.
3. **Proprietary product.** VEGA's first-party source code, editing engine, UX, and native project format remain closed-source unless the owner later changes that policy. The application may use reviewed third-party libraries under compatible licenses.
4. **Optional AI.** AI is never required for basic image editing. The initial AI direction is opt-in local processing; online inference is not part of the initial release.
5. **Windows first.** Windows 11 is the implementation and release priority. macOS and Android are planned after the desktop foundation is stable.
6. **Hybrid UI/core architecture.** C#/.NET 11 + Avalonia/XAML is the desktop UI for Windows and later macOS; Rust owns the shared document/editing/rendering core. Kotlin + Jetpack Compose provides a native touch-first Android 11+ UI. Each UI talks to the same Rust semantics through a narrow, versioned native binding layer.
7. **Interoperability.** VEGA opens PSD and maintains its own editable project format, provisionally `.vega`.
8. **Vibe-coded but production-minded.** Each implementation task must leave a buildable application, real functionality, acceptance tests, and honest reports of what was verified.
9. **Optional online backend.** TypeScript on Cloudflare Workers, Cloudflare R2 object storage, Cloudflare D1 metadata, and Firebase Authentication provide basic account/cloud/share/plugin functionality without making local editing require an account. This is an intentional exception to the C# desktop / Rust core / Kotlin Android client stack.
10. **Private cloud quota.** Each account receives 250,000,000 bytes of private online storage. Public and unlisted share uploads use a separate pool and do not count against that allowance.
11. **Public sharing policy.** Public share and community-plugin uploads have no per-user total-storage quota or item-count cap. Safety validation, per-request/platform limits, moderation, rate limits, and service-protection controls remain required.
12. **Device account policy.** Up to three distinct VEGA accounts may be registered to one application installation. Do not claim a tamper-proof physical-device limit on Windows without a proven attestation mechanism.
13. **No collaboration in the initial backend.** Basic authentication, cloud files, public sharing, and plugin publishing/catalog are in scope; real-time collaboration, chat, comments, and team features are not.

### Product non-negotiables

- A quick action and its equivalent advanced tool must operate on the same underlying document.
- Core editing works offline and does not require an account.
- The app never silently pretends an unsupported PSD feature is editable or preserved.
- Saving, reopening, undo/redo, and recovery are product features, not late-stage polish.
- No placeholder tool may look implemented while doing nothing or producing fake results.
- Cloud features are opt-in and cannot discard local work when a request fails.
- Server authorization and quota checks are authoritative; the desktop client is never trusted to enforce access control or storage usage.
- All user artwork stays local unless the user explicitly invokes an online operation such as cloud save, publishing, or downloading a community resource.
- The UI has its own VEGA identity. It can use familiar editing conventions, but must not copy Photoshop's protected branding, artwork, or assets.

---

## 2. Product definition and target users

VEGA is a desktop-first raster image editor with compositing, retouching, typography, graphic-design tools, professional color adjustment, editable project storage, and PSD interoperability.

### User groups

- **Beginners:** crop, resize, remove backgrounds, correct lighting, add text, and export without needing to learn the entire application first.
- **Content creators:** work with social-media dimensions, reusable presets, text, shapes, and multiple exports.
- **Photographers:** retouch, color-correct, apply nondestructive adjustments, and handle higher-quality photo workflows.
- **Designers:** combine raster imagery, typography, shapes, masks, and layered compositions.
- **Professional editors:** use precise controls, advanced selections, complex layers, paths, channels, color management, and dependable format exchange as those features are implemented.

### Product success measures

Use these as proposed acceptance goals, then measure them with real users and test hardware:

- A new user can open an image, crop it, adjust lighting, undo a change, and export without reading a manual.
- Every visible enabled command does what it says and has consistent error behavior.
- A saved VEGA project reopens with the same supported editable structure and visually equivalent composite.
- A PSD import produces a preview and a clear report of preserved, approximated, and unsupported features.
- Common canvas interactions do not block the application UI thread.
- Core editing remains usable without network access or an account.
- The same command behaves consistently whether initiated from a toolbar, menu, property panel, search, or shortcut.

---

## 6. Free and proprietary product policy

VEGA is completely free to use and proprietary. In the initial plan this means:

- No purchase price, subscription, paywall, advertisements, feature unlocks, or mandatory account.
- No paid tier hidden behind the UI.
- The VEGA application and first-party source code remain proprietary/closed-source unless the owner later changes the licensing policy.
- “Free” describes the user price; it does not imply that third-party dependencies, fonts, assets, or AI models have unrestricted licenses.
- Avoid remote services with ongoing per-user operating costs in the initial release.
- Optional donations or sponsorship should not alter feature access and need a separate decision; they are not required or assumed.

### Proposed proprietary-code target

The stated “95% proprietary / 5% libraries” target should be treated as an ownership and dependency-control objective, not as a literal source-line quota. UI frameworks and codecs can contain far more than 5% of total program lines while remaining infrastructure. VEGA should own its core document model, operations, editing behavior, composition policy, native format, PSD adaptation, and user experience.

Dependency governance:

- Maintain an explicit allowlist and record name, version, purpose, license, source, and whether it ships to users.
- Review all transitive dependencies and licenses before shipping; do not assume a package's main license tells the whole story.
- Avoid GPL/AGPL or other copyleft dependencies unless a deliberate legal review approves them.
- Include required notices and an SBOM with releases.
- Pin versions and update dependencies in tested batches.
- Use maintained, reviewed codecs for standard media formats rather than creating a security risk merely to avoid third-party code.
- Do not copy source code from other editors without licensing and provenance review.


## Hybrid UI/core architecture (v2.1)

The product vision and feature scope remain unchanged; this revision changes the implementation split based on the product owner’s UI-quality preference. Desktop UI is C#/.NET 11 + Avalonia; the authoritative editor/document core stays Rust; Android uses Kotlin/Compose; the backend remains TypeScript/Cloudflare. A small C ABI and generated/maintained Kotlin bindings are explicit, versioned contracts, not a reason to share raw pointers or duplicate editing logic. .NET 11/Avalonia compatibility is a Phase 0 gate because .NET 11 is still pre-GA on this planning date and Avalonia’s .NET 11 work is still being tracked. See `architecture.md`, `dotnet-development.md`, and ADR-0003.
