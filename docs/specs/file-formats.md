# File Formats & Interoperability

> VEGA Planning Set v2.1 · Category document
> The native VEGA document, PSD/PSB behavior, other formats, and fidelity reporting.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [7. File formats and data safety](#7-file-formats-and-data-safety)
- [70. File format and asset interoperability expansion](#70-file-format-and-asset-interoperability-expansion)
- [75. Interoperability and format-fidelity scorecards](#75-interoperability-and-format-fidelity-scorecards)

---

## 7. File formats and data safety

### 7.1 Native working format

**Proposed extension:** `.vega`

VEGA's native format is the authoritative editable project format. It must be versioned, resilient to interrupted saves, and capable of representing VEGA features even when those features cannot be written to PSD.

A versioned container is recommended, provisionally ZIP-based, but its physical layout must be tested for large documents before finalizing. A possible logical layout is:

```text
project.vega
  manifest.json
  document.json
  layers/
    <stable-layer-id>/metadata.json
    <stable-layer-id>/pixels-or-tiles/...
  assets/
  previews/
    thumbnail.png
    composite.png
  color/
    profiles/...
```

The actual format may store tiles in larger binary blobs or a database-like chunk store; do not assume that one tiny file per tile is optimal.

If a project is uploaded to private account storage, the saved container and any separately retained assets/previews count against the account’s 250,000,000-byte online quota. Local copies remain unrestricted by this cloud quota. Uploading/publishing a community copy is a separate opt-in operation and does not count against the private quota.

Required format properties:

- Explicit format/schema versions and migration code.
- Stable IDs for all layers, groups, masks, and assets.
- Document dimensions, resolution, color settings, layer order, visibility, opacity, transforms, blending, masks, text, effects, and asset references as features are implemented.
- Composite preview and thumbnail.
- Atomic save: write to a temporary location, flush and validate, then replace the prior file safely.
- Integrity checks and user-safe recovery for truncated/corrupted projects.
- Safe path handling and limits on decompressed size, entries, dimensions, and allocations when opening untrusted files.
- No executable project macros or automatic code execution.
- Round-trip tests from a clean process: save, close, reopen, compare document model and render.
- Clear behavior for linked files that are missing or moved.

### 7.2 PSD/PSB compatibility

PSD is an interoperability format, not VEGA's internal source of truth. The published PSD/PSB format specification describes file structures; it does not guarantee that a different editor can recreate every Photoshop-specific operation or effect. Therefore compatibility is a tested capability matrix, not a binary “supports PSD” checkbox.

**Import behavior tiers:**

1. **Preserved and editable:** mapped into VEGA-native editable structures, with tested visual behavior.
2. **Imported with approximation:** imported, but a report clearly describes differences.
3. **Composite fallback:** rendered preview/composite is available if full structure cannot safely be represented; label it as flattened/non-editable fallback.
4. **Rejected safely:** malformed, unsupported, or resource-exhausting files fail with a clear message rather than crashing or allocating without bounds.

Import pipeline:

1. Validate magic bytes, headers, lengths, offsets, section boundaries, dimensions, and compression types before allocation.
2. Decode merged composite/preview when available so the user can quickly see the source and the importer can compare results.
3. Parse layer records and image data into a normalized interchange model.
4. Map supported raster layers, groups, visibility, opacity, modes, masks, and other supported structures.
5. Preserve supported text metadata where possible but report font, effect, or rendering differences.
6. Record every unsupported or approximated feature in an import report available after import.
7. Compare VEGA output to known source composites using documented tolerances and known color-management assumptions.

Export policy:

- PSD export is a staged capability with a declared supported subset.
- Before writing, identify unsupported VEGA features and clearly say if they will be omitted, approximated, or flattened.
- Keep the original `.vega` project intact.
- Never claim that a resulting PSD preserves editability beyond tested features.
- PSB and very-large-document support should follow a robust PSD pipeline and a separate resource-limit plan.

Test corpus:

- Single-layer and multi-layer raster files, nested groups, transparency, visibility, opacity, supported blend modes, masks, text, color profiles, large dimensions, supported compression methods, and malformed/truncated files.
- Fixtures must have clear testing and distribution rights. Preserve regression fixtures for every discovered parser bug when permitted.
- Test both visual equivalence and the exact list of retained/unsupported structures.

### 7.3 Other formats

- PNG/JPEG should be the first reliable raster import/export formats.
- Add WebP, TIFF, BMP, and others after codec selection, licensing review, color handling, metadata rules, and tests.
- OpenRaster can be considered as a supplementary layer-interchange format; it does not replace `.vega` or provide the full advanced native model.
- RAW, PSB, PSD write support, animation, and print-specialty formats are explicit roadmap projects, not launch assumptions.

---

## 70. File format and asset interoperability expansion

### 70.1 Import design

Each importer should have a manifest describing supported content, known limitations, resources it may reference, whether it can contain active/executable content, and its resource limits. Importers should produce a structured report that can be saved alongside diagnostics but doesn't leak private content by default.

Image orientation, ICC profiles, transparency, metadata, resolution, alpha, bit depth, and animation must be tested independently; “file opened successfully” is not sufficient evidence that the file was interpreted correctly.

### 70.2 PSD/PSB compatibility contract

PSD/PSB work should be tracked across these distinct surfaces:

1. Header and document metadata parsing.
2. Composite preview reading.
3. Pixel layers, visibility, order, names, opacity, and supported blend modes.
4. Groups, masks, clipping, channels, and supported layer types.
5. Text/shape/vector representations where feasible.
6. Adjustment layers and layer effects by explicit supported subset.
7. Smart/embedded/linked content and external-resource relinking where understood.
8. Color modes, profiles, bit depth, and alpha/channel behavior.
9. Frame/timeline/video-related structures, if supported by the format variant and VEGA's planned module.
10. Exporting VEGA's supported subset with versioned tests and a loss report for unsupported VEGA content.

The importer should preserve unknown non-executable blocks if format rules allow it and preservation can be validated. Unknown structures must never be interpreted as code. When full preservation is unsafe, VEGA may retain the source file in a protected embedded object or offer a composite-only import, with a clear explanation that individual objects are not editable.

Maintain real fixture files created lawfully for the project, including synthetic test projects generated by VEGA and licensed/publicly shareable interop fixtures. Track origin, rights, expected visual output, and known layer contents. Avoid using confidential user projects as regression fixtures without explicit permission.

### 70.3 Export and handoff

Export dialogs should be organized into common preset choices and advanced settings, with the two views changing disclosure rather than feature availability. Offer format-specific preview, estimated file size, compression/quality, dimensions, transparency, profile conversion/embedding, metadata policy, resampling method, and naming rules where relevant.

For web delivery, plan assets/slices/export regions and batch exports by layer/composition/artboard, plus SVG where content remains vector-compatible. For print, include document dimensions, effective resolution, destination profile, soft-proof state, bleed/crop marks if required by the print workflow, and warnings for out-of-gamut colors or insufficient resolution.

---

## 75. Interoperability and format-fidelity scorecards

### 75.1 No single pass/fail for file compatibility

Each format must have a capability-specific scorecard:

- Opens valid files within supported resource limits.
- Correct dimensions, orientation, frame count, layer order, and visibility.
- Pixel/color/alpha fidelity within the declared reference tolerance.
- Correct handling of profiles, metadata, and bit depth where supported.
- Preserves editable structure and supported effect parameters.
- Clearly reports unsupported features and missing resources.
- Handles malformed, truncated, oversized, or adversarial input without memory corruption or unsafe execution.
- Exports valid files that independent readers can open.
- Round-trips supported features without cumulative drift beyond defined tolerances.

### 75.2 Compatibility report schema

For each opened project, keep a structured internal report with file hash, format variant, parsed feature inventory, converted features, preserved opaque data, unsupported/lost features, resource references, profile conversions, and warnings. The user-facing view should be concise and actionable. The raw report is exportable for troubleshooting but must not automatically upload any source content.

### 75.3 Safe conversion strategy

Use a layered conversion pipeline: validate container/header; enforce dimensions and resource budgets; parse known structures; decode content into internal objects; retain safe unknown payloads where supported; build a preview; generate compatibility report; then commit as a new document or explicit copy. Parsing is a read-only stage. If import fails, no half-open document or partially committed asset should replace the user's existing state.

Export follows a preflight stage: inspect unsupported constructs; show loss warnings; allow cancel or export a copy; encode into a temporary file; verify output; atomically replace destination only when the user explicitly agreed and the operation succeeded. Test disk-full, permission denied, cancellation, and process interruption.
