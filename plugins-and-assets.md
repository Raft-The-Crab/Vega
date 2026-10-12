# Plugins, Extensions & Creative Assets

> VEGA Planning Set v2.1 · Category document
> Plugin packages, sandboxing, resource packs, fonts, presets, assets, and catalogs.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [23. Plugin system and plugin sharing](#23-plugin-system-and-plugin-sharing)
- [45. Asset, preset, font, and resource management](#45-asset-preset-font-and-resource-management)
- [74. Plugin and extension model — feature coverage expansion](#74-plugin-and-extension-model-feature-coverage-expansion)

---

## 23. Plugin system and plugin sharing

The plugin system must support a community without granting every downloaded plugin unrestricted access to the user's computer.

### 23.1 Package format and manifest

Use a versioned `.vegaplug` package, provisionally a ZIP container with a validated manifest, one or more WebAssembly (`.wasm`) modules, icons, local resources, translations, and optional declarative presets. The exact schema is versioned and documented before a public plugin SDK is released.

The manifest should include:

- Stable plugin ID, display name, version, author/publisher ID, description, plugin API version, minimum VEGA version, supported platforms, entry module, checksum, and license.
- Declared capabilities/permissions: read selected document content, create/edit document objects, register a tool or panel, access user-selected files, or make network requests, if and when these APIs are provided.
- Resource limits and supported operations where relevant.

Do not identify a plugin only by display name. Use an immutable ID and semantic versions. Plugin updates must declare compatibility and allow rollback to a prior known-good version.

### 23.2 Runtime security

- First public plugin runtime should be sandboxed WebAssembly with a host-owned capability API, not arbitrary `.NET` assemblies or native DLL loading.
- Deny filesystem, process creation, environment access, arbitrary networking, and arbitrary OS APIs by default.
- Grant only declared, supported capabilities, with user consent for sensitive permissions. A plugin must never gain more access than the approved host capability it receives.
- Bound execution time/fuel, memory, recursion, output sizes, and job count; support cancellation and crash isolation.
- Plugin calls that alter a document must use the same command/undo infrastructure as built-in tools and be schema-validated at the host boundary.
- Pin plugin versions per document/session or user preference where repeatability matters. A malicious/new update must not silently replace code in the middle of an editing session.
- Show publisher, version, permissions, signature/verification state, compatibility, and warnings before install/update. Provide disable, uninstall, report, and rollback controls.
- Verify package hashes and publisher signatures if/when a signing scheme exists. A hash alone proves byte identity, not that the author is trustworthy.
- Never auto-run newly downloaded plugins. User initiates installation and explicitly enables execution.

### 23.3 Marketplace behavior

Publisher uploads and plugin catalog metadata are stored separately from private project storage. Public plugin packages have no per-user storage quota, but all uploads remain subject to technical size, request-rate, content-policy, and security controls. The catalog can expose pending/approved/hidden/removed state, release notes, supported VEGA versions, and a way to report unsafe content. At launch, moderation may be a review queue/manual process rather than automated malware analysis.

### 23.4 Implementation staging

1. Define the plugin manifest and extension points on paper.
2. Ship built-in tools through internal interfaces first so those boundaries are proven.
3. Implement declarative packs/presets before executable plugins.
4. Add the WASM runtime and limited read-only/resource APIs.
5. Add opt-in document-modifying APIs that integrate with undo/redo.
6. Add publishing, download, version management, permissions, rollback, and reporting.

Do not release executable community plugins before the sandbox, permission model, resource caps, package validation, and incident response all have tests.

---

## 45. Asset, preset, font, and resource management

VEGA needs a common asset/resource system used by local workflows and the community catalog.

### 45.1 Asset categories

- Imported images and embedded/linked media.
- Fonts and font metadata (subject to font license and installation availability).
- Brushes and brush dynamics.
- Gradients, patterns, LUTs/color looks, and adjustment presets.
- Shapes, paths, palettes, and other declarative resources.
- Templates/projects and preview thumbnails.
- Plugins, with a clearly separate trust and permission lifecycle.

### 45.2 Local resource library

- Users can tag, favorite, search, preview, install, update, disable, and delete resources.
- Resource identifiers are stable and versioned; duplicate names do not imply duplicate identity.
- A document referencing a missing resource shows a recoverable placeholder and lets the user relink or substitute it.
- Template opening creates a copy. Presets apply commands to the current document; original preset files remain unchanged.
- Installed resources can work offline. Online catalog browse/download is optional.

### 45.3 Resource packs versus executable plugins

Ship declarative resource packs before executable plugins. A resource pack contains data interpreted by built-in VEGA features and has no arbitrary code. Executable extensions must use the sandbox API, with explicit permissions and compatibility rules. The application must not treat a PNG/JPEG preview as trusted simply because it is stored inside a valid package.

### 45.4 Font behavior

- Do not redistribute fonts unless their licenses permit it.
- Record font family/style metadata and fallback behavior for missing fonts.
- A missing-font warning should list affected text objects and allow substitution without silently rewriting source text.
- Font parsing happens through OS APIs or a maintained parser with bounded inputs; untrusted uploaded fonts should be quarantined and never automatically installed system-wide.

---

## 74. Plugin and extension model — feature coverage expansion

VEGA's plugin system has two compatibility tiers:

1. **Resource bundles:** brushes, gradients, patterns, swatches, shapes, presets, actions/recipes, templates, LUTs, and workspace layouts. These should be the first extensibility release because they can be declarative and narrowly validated.
2. **Executable extensions:** tools, filters, importers/exporters, panels, automation commands, and other extensions running under a sandboxed host. These arrive only after stable API boundaries, permissions, review/disable controls, and adversarial testing exist.

### 74.1 Manifest requirements

Every executable package declares ID, version, publisher, VEGA API range, entry point, hash, signature if applicable, requested capabilities, resource limits, optional network domains/endpoints, supported platforms, dependencies, update channel, license, and description. The declared permissions are not self-enforcing; the host must validate and enforce them at every privileged call.

### 74.2 Runtime isolation

Use a restricted WebAssembly/WASI-style runtime or another audited sandbox with a narrow host-call API. Do not load arbitrary community DLLs or .NET assemblies into the main UI process. Plugins receive opaque document references and validated operations, not unfiltered native pointers or arbitrary filesystem handles. They cannot read account tokens, private application configuration, unrelated documents, or process memory.

Network, filesystem, clipboard, selected-pixel access, document mutation, and external process operations require separate permissions. Deny by default. Apply CPU, wall-clock, memory, output-size, recursion, and cancellation limits; crashed or over-budget plugins must be disabled without bringing down the editor. The host must validate all plugin-produced parameters and data structures.

### 74.3 Compatibility and trust labels

Show independent labels for signature verification, publisher identity, review status, version compatibility, required permissions, and revocation status. A store listing or positive rating is not proof of security. Users need disable, uninstall, roll back, report, and inspect-permissions actions. A revoked plugin must be disabled promptly according to the offline revocation policy, while allowing users to open projects with missing-plugin warnings instead of deleting effects.

### 74.4 Plugin test suite

Cover corrupt archives, archive path traversal, duplicate IDs, signature mismatch, malformed manifest, denied capabilities, oversized output, infinite loops, memory abuse, cancellation, forged result types, API version mismatch, dependency conflicts, failed update, rollback, and project-open behavior after revocation. Plugins that produce document operations must be tested with undo/redo and save/reopen.
