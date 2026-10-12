# Editing Capabilities & Feature Inventory

> VEGA Planning Set v2.1 · Category document
> The target editing scope, professional tool families, and capability coverage.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [4. Product feature map](#4-product-feature-map)
- [58. Capability-parity charter: broad scope, independent implementation](#58-capability-parity-charter-broad-scope-independent-implementation)
- [59. Full capability coverage matrix](#59-full-capability-coverage-matrix)
- [61. Layers, objects, channels, and compositing — expanded requirements](#61-layers-objects-channels-and-compositing-expanded-requirements)
- [62. Selection, masks, and edge quality](#62-selection-masks-and-edge-quality)
- [63. Brushes, painting, and tablet workflow](#63-brushes-painting-and-tablet-workflow)
- [64. Retouching, healing, and photo restoration](#64-retouching-healing-and-photo-restoration)
- [65. Photo development, RAW workflows, and computational photography](#65-photo-development-raw-workflows-and-computational-photography)
- [66. Adjustment, filters, color grading, and image effects](#66-adjustment-filters-color-grading-and-image-effects)
- [67. Typography, vector content, graphic layout, and multi-artboard design](#67-typography-vector-content-graphic-layout-and-multi-artboard-design)
- [69. Animation, timeline, and video capability plan](#69-animation-timeline-and-video-capability-plan)
- [81. Additional current feature-reference set](#81-additional-current-feature-reference-set)
- [82. Detailed tool, panel, and command-family inventory](#82-detailed-tool-panel-and-command-family-inventory)
- [83. Final expanded product quality principles](#83-final-expanded-product-quality-principles)

---

## 4. Product feature map

The items below are the long-term roadmap. The initial release must implement a coherent subset rather than show inactive controls for future features.

### 4.1 Documents and canvas

**Foundation:** create/open/save/save-as, new-document dimensions, zoom, pan, fit-to-window, 100% view, checkerboard transparency, tabs, document dirty state, rulers/guides later, image placement, and separate image-resize versus canvas-resize operations.

**Advanced roadmap:** multiple artboards if needed, perspective grids, custom guides, measurement overlays, canvas rotation, reference images, and document metadata.

### 4.2 Layer and object system

- Raster/pixel layers.
- Layer groups with nesting.
- Visibility, lock, rename, duplicate, delete, reorder, opacity, and supported blend modes.
- Layer transform, alignment, distribute, and snapping.
- Layer masks, alpha/transparency handling, clipping masks, and adjustment layers.
- Editable text layers and vector shape layers.
- Embedded/linked assets analogous to smart objects, introduced only after references, caching, missing-asset behavior, and save semantics are designed.
- Layer effects and non-destructive filter stacks in later phases.
- Stable internal IDs; displayed layer names must never be used as identity keys.

### 4.3 Selection and geometry tools

- Rectangular and elliptical marquee.
- Freehand and polygonal lasso.
- Color-range selection and tolerance-based selection.
- Invert, feather, expand, contract, fill, transform, and save/reload supported selections.
- Selection-to-mask and mask-to-selection operations.
- Pen/path and Bézier editing in a later milestone.
- Subject/object selection and edge refinement when reliable models are available.

A selection is document/editor state with well-defined coordinates and persistence semantics; it must not be represented solely as a transient UI outline.

### 4.4 Painting and retouching

- Brush and eraser with size, hardness, opacity, spacing, color, and presets.
- Pressure-sensitive size/opacity where hardware and OS APIs support it.
- Fill, gradient, eyedropper, clone stamp, healing, dodge/burn, smudge, blur/sharpen, and noise tools as staged features.
- Stable stroke sampling, interpolation, predictable edge behavior, and cancellation.
- Brush strokes are one coherent undo operation by default, not hundreds of individual undo steps.
- Retouching should not unexpectedly smear pixels outside the expected target region.

### 4.5 Image adjustments and filters

- Exposure/brightness/contrast, levels, curves, hue/saturation, color balance, black-and-white, white balance, and invert.
- Blur, sharpen, denoise, and other filters after the processing contracts and performance model are in place.
- Presets must expose/edit their underlying parameters rather than become unexplained baked results by default.
- Adjustments should use a shared color/precision pipeline and should be testable independently of the UI.
- Non-destructive adjustment layers and editable filter stacks are the target model; destructive application may still be offered as an explicit command.

### 4.6 Typography and graphic design

- Editable text, text boxes, font family and fallback, size, weight, tracking, leading, alignment, fill, stroke, and simple text effects.
- Shapes with fill, stroke, radius where appropriate, transform, and snapping.
- Text and shapes stay editable until explicitly rasterized.
- Missing fonts must be handled predictably and reported where they affect appearance.
- Export presets for common dimensions can be introduced after the core export system is stable.

### 4.7 Photography and professional color

- EXIF/metadata reading with privacy-conscious behavior.
- Color profiles and consistent preview/export conversion.
- RAW development, lens correction, advanced retouching, higher bit depth, HDR workflows, and CMYK workflows are advanced roadmap items—not assumptions for the first beta.
- Decide bit-depth/color-mode scope explicitly before implementation. Do not claim pro print or HDR suitability until end-to-end color management is tested.

### 4.8 Export and batch workflows

- PNG and JPEG first; WebP/TIFF/BMP and other formats after codec/license review and adequate tests.
- Export current composite, supported layers where format supports them, and later batch export.
- Export presets must state file type, dimensions, quality/compression, color profile behavior, transparency behavior, and metadata behavior.
- Never silently discard alpha, convert color profiles unexpectedly, or overwrite an existing file without clear UX.
- Batch actions and macro/automation features are later milestones after normal commands are deterministic and testable.

### 4.9 Optional accounts and community services (later backend phase)

- Local editing remains available while signed out.
- Optional Firebase-backed account supports private cloud storage, a public/unlisted community library, and plugin discovery.
- Cloud save is explicit; no file syncs online automatically. Private storage is 250,000,000 bytes per account.
- Public sharing has no per-user total-storage quota or item-count cap, separate from private cloud storage.
- Community uploads require safe format validation, moderation/reporting, and platform abuse controls.
- No comments, messages, teams, shared cursors, live co-editing, or other collaboration features in the first backend.

---

## 58. Capability-parity charter: broad scope, independent implementation

VEGA's long-term product scope is to cover the major workflows and tool families associated with Photoshop: raster editing, compositing, selection and masking, retouching, painting, non-destructive image adjustments, typography and design, color-managed output, Camera Raw-style development, automation, extensibility, frame animation, and timeline-based motion work. It should also improve the learning curve, reliability, observability, and repeatability of those workflows.

This is a **target capability catalog**, not a claim that every listed capability already exists or that every Photoshop file can be perfectly interpreted. Adobe continues to add and change functionality, including adjustment presets, newer selection and masking workflows, generative tools, scripting/actions, and timeline features. VEGA should re-check the linked official documentation during each major planning cycle rather than freeze the comparison against a single feature snapshot.

Reference material used to establish the broad coverage map includes Adobe's official [Photoshop feature overview](https://www.adobe.com/products/photoshop/features.html), [selection tools overview](https://helpx.adobe.com/photoshop/desktop/make-selections/get-started-selections/selection-tools-overview.html), [retouch tools overview](https://helpx.adobe.com/photoshop/desktop/repair-retouch/remove-objects-fill-space/retouch-tools-overview.html), [adjustment layers guidance](https://helpx.adobe.com/photoshop/desktop/create-manage-layers/color-adjustment-layers/work-with-adjustment-and-fill-layers.html), [color-management guidance](https://helpx.adobe.com/photoshop/desktop/adjust-color/color-profiles/color-management-in-photoshop.html), [Camera Raw masking](https://helpx.adobe.com/camera-raw/desktop/edit-and-enhance-images/masking-and-local-adjustments/masking.html), [generative AI overview](https://helpx.adobe.com/photoshop/desktop/generative-ai/generative-ai-features-overview.html), [timeline animation workflow](https://helpx.adobe.com/photoshop/desktop/add-video-and-animation/use-keyframes/create-timeline-animation-workflow.html), and [automation with scripts](https://helpx.adobe.com/photoshop/desktop/automate-tasks/automation-settings-and-presets/automate-photoshop-with-scripts.html).

### 58.1 What parity means for VEGA

A feature is not considered equivalent simply because it has a similarly named button. Feature parity must be judged by user outcomes, editability, control, output fidelity, and file round-tripping.

For each feature, define and test:

1. **Behavior:** Does the operation produce the intended result across normal and edge cases?
2. **Control:** Are the relevant parameters exposed and predictable?
3. **Editability:** Can the user revise, disable, reorder, or remove the operation when it is advertised as non-destructive?
4. **Composability:** Does it work with selection masks, layer masks, groups, opacity, blending, history, color profiles, and export as appropriate?
5. **Persistence:** Does save, close, and reopen preserve its parameters and visual result within defined tolerances?
6. **Interoperability:** Does import/export preserve the supported semantics, or does VEGA clearly identify what cannot be preserved?
7. **Performance:** Does the operation meet a documented response-time/memory budget on the declared reference hardware?
8. **Accessibility and learning:** Can it be discovered, operated by keyboard where feasible, and understood without relying on icon recognition alone?
9. **Failure behavior:** Are cancellation, unavailable features, malformed inputs, unsupported modes, and out-of-memory cases handled safely?
10. **Security:** Can untrusted files, plugins, scripts, or remote resources exploit the operation or bypass permission boundaries?

Parity status values should be standardized: **Not planned**, **Designed**, **Prototype**, **Implemented**, **Feature-complete**, **Interoperability-tested**, and **Released**. “Feature-complete” requires passing the feature's acceptance checklist. “Interoperability-tested” additionally requires fixture-based import/export results. Marketing must use these states honestly.

### 58.2 Product and intellectual-property boundary

VEGA can implement familiar categories of professional editing functionality and standard interaction conventions. It must not copy Photoshop's protected application assets, exact proprietary artwork, code, private implementation details, or branded visual identity. Reverse engineering an unsupported proprietary algorithm is not a substitute for independent design and lawful use of public specifications. File-format parsing and interoperability work must receive a licensing review and maintain a clear separation between published format behavior and assumptions.

### 58.3 Scope is broad; release order is deliberate

The features in Sections 59–70 are part of the long-term capability goal unless explicitly marked as an external integration, validation item, or separate major module. Priority labels decide sequence rather than deleting the product goal:

- **P0 — Engine correctness:** needed to establish document integrity, editing consistency, history, color/pixel semantics, and crash-safe persistence.
- **P1 — First credible desktop release:** essential end-to-end capabilities required for real users to finish meaningful work.
- **P2 — Professional coverage:** advanced controls and workflow depth needed for demanding recurring production work.
- **P3 — Extended parity and VEGA differentiation:** compute-intensive, specialized, video/animation, extensive compatibility, and new experience improvements that depend on earlier foundations.
- **External or conditional:** features requiring third-party systems, licensed codecs/models, hardware capabilities, or external legal/operational approval. They remain clearly described but are not assumed to be available offline or in every build.

---

## 59. Full capability coverage matrix

This table is the high-level source of truth for scope. Individual items should become epics, feature records, tests, and release notes. Related features may ship as a coherent group rather than in the order shown.

| Capability area | VEGA target | Priority | Exit criterion |
|---|---|---:|---|
| Document creation/open/save | New files, presets, open/import, Save, Save As, Revert, duplicate, close, recovery, linked assets | P0 | Document state and content survive save/reopen; failures never silently overwrite the last good file |
| Canvas navigation | Zoom, pan, rotate view, fit/fill, actual pixels, navigator, multi-view, pixel grid | P0/P1 | Accurate hit-testing at every zoom and rotation; no accidental image mutation from view actions |
| Raster layers | Create, rename, duplicate, reorder, group, merge, flatten, lock, hide, color labels | P1 | Stable layer identity and correct order/visibility through history and persistence |
| Layer masks | Reveal/hide masks, paint, invert, disable, apply, copy, density/feather where supported | P1/P2 | Mask state is editable and deterministic; no destructive application without explicit command |
| Vector masks and paths | Bézier paths, shape paths, convert selection/path, vector masking, path operations | P2 | Curve geometry remains editable; rasterization is explicit and reversible when supported |
| Blend/compositing | Standard and advanced blend modes, opacity/fill opacity, clipping, group compositing | P1/P2 | Reference-based pixel tests for every mode and supported color space |
| Layer effects | Shadow, inner shadow, glow, bevel/emboss, stroke, overlays, contours, effect stacking | P2 | Parameters remain editable; preview and final render use the same model within tolerance |
| Smart/embedded objects | Linked/embedded raster or nested project content; non-destructive transforms; linked-instance updates | P2 | Transforming/scaling repeatedly does not degrade the source; links resolve or report missing assets |
| Adjustment/fill layers | Tonal, color, gradient, solid, pattern, LUT, threshold/posterize and later specialist adjustments | P1/P2 | Original pixels remain unchanged until user explicitly rasterizes/commits an operation |
| Selections | Marquee, lasso, polygonal, magnetic/edge-aware, quick/object/color selection, select subject/sky where capable | P1/P2 | Selection geometry and soft edge are reproducible and can be saved or turned into a mask |
| Selection refinement | Add/subtract/intersect, feather, smooth, expand/contract, border, hair/edge refinement, decontaminate where feasible | P1/P2 | Refinement produces an editable mask and does not irreversibly alter the source by default |
| Painting | Brush, pencil, eraser, airbrush behavior, texture/tip, hardness/spacing, opacity/flow, smoothing, pressure | P1/P2 | Smooth low-latency strokes; deterministic pressure fallback; undo handles whole strokes as expected |
| Advanced brush systems | Brush dynamics, tilt/rotation, scatter, dual brushes, wet-media simulation, symmetry, custom brush import | P2/P3 | Presets are versioned; unsupported parameters are reported on import rather than silently ignored |
| Retouching | Spot repair, healing, clone, patch, red-eye, dodge/burn, sponge, blur/sharpen/smudge | P1/P2 | Sample source and affected target are explicit; tools respect selection/mask and history |
| Transform and geometry | Scale, rotate, skew, distort, perspective, warp, rotate object, flip, align, distribute, numeric transforms | P1/P2 | Repeated transform editing retains source quality when used through non-destructive object models |
| Content-aware operations | Content-aware fill/move/scale, patch synthesis, remove distractions, perspective-aware fills | P2/P3 | Results are reproducible within model scope; result is isolated on a new layer/mask where possible |
| Photo/color adjustments | Exposure, brightness/contrast, levels, curves, hue/saturation, vibrance, color balance, selective color, channel mixer, B/W | P1 | Well-defined channel/math semantics and nondestructive parameter persistence |
| Advanced color/effects adjustments | Dehaze, clarity, texture, grain, photo filter, gradient map, color lookup, inversion, threshold, posterization | P2 | Adjustment parameters, color-space assumptions, and profile behavior are documented and tested |
| Filters | Blur families, sharpen, noise, distort, stylize, render, pixelate, morphology, convolution, custom filter graph | P1/P2 | Filter parameters are saved; destructive and non-destructive modes are clearly differentiated |
| Smart filters | Editable effect stacks on object layers, masks per filter where supported, ordering, blend/opacity | P2 | Filters can be disabled/reordered/edited after reopening a project |
| Blend and compositing effects | Layer compositing, alpha math, channel controls, tonal blend ranges, blend-if equivalents, knockout behavior | P2 | Edge cases with alpha, premultiplied alpha, groups, and masks are covered by pixel fixtures |
| Text | Editable text layers, font family/weight, point/paragraph, size, tracking, leading, kerning, baseline, alignment | P1 | Text remains editable and missing-font substitution is visible and recoverable |
| Advanced typography | OpenType features, variable fonts, vertical text where relevant, text on a path, paragraph flow, text boxes, glyph panel | P2 | Glyph shaping follows platform-independent text layout rules as far as possible; font metadata survives save |
| Text/vector effects | Text warping, outlines, fill/gradient, shadows, styles, clipping, rasterize-on-demand | P2 | Common text effects remain editable; rasterization is explicit and reversible through undo |
| Shape/vector design | Rectangle, ellipse, polygon, line, custom shapes, fill/stroke, gradient, Boolean path operations | P1/P2 | Shapes remain vector-editable at any zoom until explicitly rasterized |
| Typography/layout helpers | Guides, rulers, grids, smart guides, snapping, alignment/distribution, spacing, measurement | P1 | Consistent snapping and alignment at different zoom levels; guides don't alter output unless requested |
| Artboards and multi-layout docs | Multiple canvases/artboards, per-board dimensions, background, duplication, exported board sets | P2 | Artboard-specific export and layer ownership are clear and stable in the native format |
| Frame/layer animation | Frame timeline, frame delay, disposal/loop settings, per-frame visibility, onion-skin preview where applicable | P2/P3 | Frame timing and exported animation match preview under documented playback rules |
| Timeline/keyframe animation | Animated properties, keyframes, easing, work area, layers and media tracks, playback cache | P3 | Deterministic render at a specified timestamp/frame rate; saving preserves keyframe curves |
| Video layers | Import supported video/sequence formats, trim, frame navigation, paint/retouch per frame where feasible | P3/conditional | Codec support is explicit; unsupported codecs fail gracefully; timeline stays separate from still-image P0 requirements |
| Camera Raw-style workflow | RAW decoding, white balance, exposure, profiles, lens correction, tone curve, detail, optics, local masks | P2/P3 | Supported camera fixtures produce stable outputs and expose decoder/profile information |
| Photo merge tools | HDR merge, panorama stitch, focus stacking, exposure blending, image alignment, image-stack statistics | P3 | Source alignment and merge output can be inspected; source images remain intact |
| Image science | Histogram, channels, bit depth, color modes, profiles, soft proof, gamut warning, metadata, EXIF orientation | P0/P2 | Pixel math, profile handling, and mode conversions have reference tests |
| Generative/ML editing | Optional fill, expand, remove, background replacement, generate image, similar variations, harmonize, upscale | P3/external | Always optional; prompts/data flow are disclosed; output becomes a separate editable result and can be rejected |
| Automation | Record/play actions, macros, batch operations, conditions, variables, output naming, presets, queues | P2 | Recorded workflows replay with explicit handling for missing assets and failed steps |
| Scripting/API | Versioned public API, document commands, read-only inspection, safe automation, examples and debugger/logs | P2/P3 | API is versioned; scripts cannot access arbitrary host secrets or bypass user permissions |
| Plug-ins | Sandboxed effects/tools/importers/exporters/resources; manifests, permissions, signing, rollback | P2/P3 | Malformed or over-budget plugin can be disabled without compromising core application state |
| Preset/resource manager | Brushes, gradients, patterns, swatches, LUTs, shapes, styles, actions, workspaces, templates | P1/P2 | Import/export is versioned and previewable; invalid entries can be isolated and removed |
| File interoperability | PSD/PSB defined support, OpenRaster, common raster formats, PDF/SVG ingest as applicable | P1/P2 | Supported subsets documented; lossy conversion has a report and no silent feature disappearance |
| Web/asset export | PNG/JPEG/WebP/TIFF/BMP/GIF and other formats after validation; quality/size preview; batch presets | P1/P2 | Format-specific metadata, alpha, profile, dimension, and compression semantics are tested |
| Print output | CMYK workflow, soft proof, print dimensions, PPI, profile conversion, proof/print checklist | P2/P3 | Print conversions require explicit profile and rendering intent; warnings are actionable |
| Security/provenance | File parser limits, project recovery, optional edit history, exported metadata policy, signatures | P0/P1 | Security tests and user-facing privacy/metadata choices pass release gate |

This catalog should be mirrored into a tracked backlog. Add a row only with an owner, dependencies, a test fixture or test plan, a failure mode, and a measurable exit criterion. If the feature is postponed, retain its row and status so “future parity” cannot disappear from the plan without a decision.

---

## 61. Layers, objects, channels, and compositing — expanded requirements

### 61.1 Layer kinds and relationships

Plan for pixel, background, group, text, vector shape, fill, adjustment, embedded object, linked object, frame/media, and generated-result layers. Layer kinds must advertise supported operations, not merely hide unsupported behavior behind disabled icons. Groups should define compositing semantics and preserve child order. Artboards are a layout/document structure rather than just a normal group unless tests prove the simpler model is sufficient.

Layer operations to cover over time:

- New, duplicate, rename, delete, reorder, group, ungroup, collapse/expand, color label, tag, search/filter.
- Lock position, lock content, lock alpha/transparency, lock all, and preserve background behavior.
- Merge selected, merge visible, stamp/flatten copy, rasterize, convert to object, and flatten document, each with clear warnings where editing structure will be lost.
- Show/hide, solo, isolate, hide all others, toggle selection visibility, and compare one or more layers.
- Create/replace/duplicate/invert/disable/enable/apply/copy/move masks.
- Clip layers to a base layer or base group, define nested clipping behavior, and report ambiguous relationships.
- Change blend mode, opacity, fill opacity, effect stack, advanced channel blending, blend ranges, group blend behavior, and knockout behavior where supported.
- Use compositing previews that show both selected-layer results and the final full-document result.

### 61.2 Object and transform quality

Placed assets and embedded objects should preserve original source data. Transform parameters should remain mathematical values instead of being repeatedly baked into raster pixels. The engine must maintain a clear distinction between object transform, internal content transform, and output/resampling.

Support anchor points, numeric width/height, aspect lock, rotation, skew, perspective, warp grids, alignment, distribution, multi-selection transforms, transform-origin selection, snapping, and pixel-aware constraints. Provide reset-to-source and compare-to-original actions.

For warp and perspective controls, the UI must have visible handles, editable control points, keyboard precision, and a clear confirm/cancel interaction. A cancelled transform should restore the prior document without producing multiple confusing history entries.

### 61.3 Channels and advanced composition

The long-term plan includes individual component channels, alpha channels, saved spot/selection channels where supported, channel isolation and toggling, channel-to-selection and selection-to-channel workflows, channel calculations, and channel-aware adjustment/filter behavior.

Advanced blending must define how each operation interacts with color components and alpha. Do not implement blend mode equations from remembered labels alone: establish independent fixtures against well-defined mathematical references. Blend ranges/“Blend If”-style controls need split sliders or equivalent smooth ramps to avoid abrupt transitions, and their parameters must serialize in the native format.

### 61.4 Layer cleanup and explainable organization

VEGA should improve large-file maintenance with a Layer Health panel that identifies empty layers, unreferenced assets, missing fonts, unresolved links, duplicate names, disabled effects, hidden content outside the canvas, and suspiciously large embedded sources. Suggestions are previewable and reversible. Automated rename/group behavior must never silently rewrite a complex production document.

---

## 62. Selection, masks, and edge quality

Selections and masks determine the quality of nearly every compositing workflow. VEGA should treat them as first-class document data, not transient dotted outlines.

### 62.1 Selection system

Plan for rectangular/elliptical marquee, row/column selection where meaningful, freehand/polygonal/magnetic lasso, quick selection, tolerance/color range selection, object detection selection, foreground/subject selection, sky selection, select by transparency, selection from path, and selection from layer content. More complex semantic selection can depend on optional local models.

Shared selection operations: add, subtract, intersect, invert, grow/shrink, feather, smooth, expand/contract, border/ring, remove holes where appropriate, preserve soft edges, save/load selection, and transform selection independently from pixels. All selection operations must respect document coordinate space and current feather/antialiasing settings.

### 62.2 Refine Mask workspace

Provide a dedicated but integrated refine experience with edge view modes (overlay, on black/white, grayscale mask, on layers), edge radius, smooth, feather, contrast, shift edge, color decontamination where safe, and brush add/subtract. It should offer previews at fit, 100%, and an interactive close-up; show progress and allow cancel on slower operations.

VEGA improvement: show a non-blocking boundary-quality hint that highlights possible halos, missed fine hair, transparent fabric, and color spill without pretending to know the correct artistic intent. Hints should be dismissible, locally processed where possible, and never rewrite a mask automatically without explicit consent.

### 62.3 Mask types and combination

The architecture should plan for raster masks, vector masks, clipping masks, layer opacity, channel-derived masks, and adjustment masks. The UI must make the currently selected target obvious: image pixels, vector path, mask, or filter mask. Show a high-visibility target indicator and allow a user preference for confirmation before painting on a mask when their tool was recently targeting pixels.

Combinations must have defined order and alpha behavior. Saving/reopening must preserve mask relationships. Mask application/rasterization requires an explicit user action and a clear undo record.

---

## 63. Brushes, painting, and tablet workflow

### 63.1 Core brush engine

The brush engine should define brush tip shape, spacing, hardness, diameter, angle, roundness, opacity, flow, blending, smoothing, texture, scatter, dual-tip behavior, color dynamics, wetness/mix for supported media, and stroke stabilization. Parameters must have normalized ranges, serialization versions, predictable behavior across zoom levels, and a documented preset format.

### 63.2 Input and latency

Support mouse and supported pen/tablet APIs, including pressure, tilt, twist, barrel button, eraser end, and optional rotation. Pressure curves should be configurable and previewable. When a device lacks a capability, VEGA should fall back deterministically and disclose the missing input only where it affects behavior.

Record stylus samples without overloading the UI thread. Stroke interpolation should avoid visible spacing at fast or slow movement. The viewport should show immediate feedback, while the engine can commit a stroke to tiles asynchronously when safe. Cancellation and focus loss must not leave a half-committed stroke or orphaned input state.

### 63.3 Painter-focused improvements

- Brush laboratory with side-by-side preset previews and searchable tags.
- Stroke preview strip and before/after tip visualization.
- A pressure/tilt calibration canvas with no need to edit a real document.
- Dynamic brush preset comparison and rollback after importing a collection.
- Brush gesture recorder for reproducing a stroke path during tool development and bug reports, with sensitive artwork excluded by default.
- A performance status indicator that distinguishes brush lag, canvas render lag, and input-device sampling issues.

---

## 64. Retouching, healing, and photo restoration

Plan for spot healing, healing brush, clone stamp, patch-based repair, red-eye, background eraser behavior, color replacement, dodge, burn, sponge, blur, sharpen, smudge, content-aware movement, and advanced object removal. Add restoration workflows for dust/scratches, fading/color cast, tears or scratches where feasible, scanned document cleanup, film grain/noise management, and upscaling when licensed methods are available.

Every sampling tool must clearly expose sample source rules: current layer, current and below, visible layers, selected source layer, alignment behavior, offset, and sample reset. Users need to understand whether a tool samples the displayed composite or raw source layer.

Retouch tools should support non-destructive output where practical: create a repair layer, sampled-source record, or editable patch. Where a direct pixel operation is necessary, use a history command and preserve the source in undo/checkpoint. Avoid claiming that every generative repair is reversible at the model level; preserve its generated output and parameters instead.

VEGA improvement: before/after toggle on the affected region, with a zoom-locked compare view so the user does not lose context while judging small repairs. Optional artifact detection can highlight suspicious repeating texture, seams, halos, and oversharpening, with no automatic correction unless requested.

---

## 65. Photo development, RAW workflows, and computational photography

### 65.1 RAW development

The long-term program includes a dedicated non-destructive RAW development stage for supported sensor formats, white balance, tint, exposure, contrast, highlights, shadows, whites/blacks, tone curve, texture/clarity/dehaze, color mixer, color grading, sharpening, noise reduction, chromatic aberration, lens profile correction, geometry, crop, calibration, and local masks. Exact controls depend on the chosen decoder/processing stack and licensing.

Maintain an explicit compatibility table by camera/decoder capability rather than claiming universal RAW support. Every RAW importer needs test files covering supported camera families, high-bit-depth behavior, orientation, embedded previews, and missing/unsupported metadata. Preserve source RAW content and store development settings separately.

### 65.2 Local adjustment masks

Support manually painted, linear/radial gradient, luminance-range, color-range, subject, sky, people/subject-part, object, and intersected masks as algorithms permit. The user can add/subtract with brush or gradient, inspect the mask overlay, and apply a full set of local photo adjustments. A semantic mask derived from an optional model must identify that dependency and preserve its result so a project remains visually stable without rerunning the model.

### 65.3 Photo merge and stacks

Plan for image alignment and merging workflows: exposure bracketing/HDR, panorama stitching, focus stacking, average/median stacks for noise reduction or removing transient objects, and supported high-resolution composition. The flow must show source selection, alignment preview, ghosting/overlap warnings, crop boundaries, merge parameters, and a non-destructive final output layer or nested merge object.

### 65.4 Camera and lens corrections

Lens correction metadata, distortion, vignetting, chromatic aberration, horizon, perspective, upright geometry, and crop constraints should be editable and tied to known source metadata where available. Profile lookup must be transparent: show which profile was applied and how users can override it or report a wrong match.

---

## 66. Adjustment, filters, color grading, and image effects

### 66.1 Adjustment library

Target an extensible adjustment registry rather than a hard-coded one-off dialog for every operation. Planned adjustment families include:

- Brightness/contrast, exposure, levels, curves, automatic tone, highlights/shadows, and white/black points.
- Hue/saturation, vibrance, color balance, photo filters, channel mixer, selective color, color lookup tables, and color grading wheels.
- Black and white, gradient map, invert, posterize, threshold, channel operations, and monochrome toning.
- Temperature/tint, clarity, texture, dehaze, grain, sharpening, and local contrast.
- Solid-color, gradient, and pattern fills.
- Dedicated print/color proof overlays and gamut checking where color-management support is available.

Each adjustment definition must include supported modes/bit depths, parameter ranges, interpolation, color-space assumptions, masks, and a versioned serializer. Unsupported adjustment classes imported from other file formats must be preserved as opaque data when safe or represented by a composite preview plus a compatibility warning.

### 66.2 Filter library

Plan for Gaussian/lens/motion/radial/box blur, surface and edge-aware blur where supported, sharpening and high-pass, noise/add/remove, median, distort/displace, perspective/warp, liquify-style mesh deformation, lens and lighting effects, pixelate, halftone, morphology, edge detection, emboss, relief, sketch/stylize, render patterns/noise/clouds, and custom convolution/filter graph support.

Not every Photoshop legacy filter needs an identical menu or algorithm. VEGA should organize filters by what users want to achieve (blur, focus, repair, texture, distort, stylize, generate) while preserving direct access to named technical filters in a searchable catalog.

### 66.3 Filter graphs and previews

Every non-trivial effect should declare whether it is destructive, non-destructive, or can use either mode. For non-destructive filtering, store an operation graph with stable IDs and explicit inputs. The graph must detect cycles, unsupported versions, invalid parameter values, and missing models/assets.

Provide preview quality levels (draft/normal/final) and visibly indicate when a draft preview is not full resolution. Preview result must not be mistaken for final output. If render evaluation fails, preserve the last known good preview and expose a clear error without corrupting the document.

### 66.4 Presets and grading consistency

Adjustments and filters should support saved presets, preset groups, per-preset notes, version migration, import/export, before/after comparison, and batch application. Presets must not silently reference assets that have not been installed. Users can select whether importing a preset replaces values or adds a nondestructive layer.

VEGA improvement: create a **Look Recipe** from the stack of visible compatible adjustment operations. A recipe should preserve order, parameters, and mask behavior where representable. Applying a recipe to an incompatible document reports the omitted or changed components rather than silently producing a materially different result.

---

## 67. Typography, vector content, graphic layout, and multi-artboard design

### 67.1 Text editing

Text layers must support live editing, point and paragraph text, boxes, overflow indicators, resize/reflow, multiple styles in a text run, font selection, family/weight/style, font size, leading, tracking, kerning, baseline shift, alignment, tabs and paragraph spacing where supported, OpenType features where the shaping stack supports them, variable font axes where available, text color/fill/stroke, and font fallback.

Font installation and licensing remain OS/user responsibilities unless a legally approved font/resource distribution workflow is added. Missing fonts must be named in an asset warning, not invisibly substituted. Users should see the difference between a temporary fallback and a committed text-layout conversion.

### 67.2 Vector and shape tools

Plan for editable paths, pen/freeform pen, direct path selection, node add/delete/convert, corner and smooth nodes, fill/stroke, stroke alignment, dashes, caps/joins, path operations, path-to-selection, selection-to-path, text-on-path, custom shapes, and SVG import where supported. Bézier editing should support keyboard navigation and numeric transforms. Vector content is rasterized only when the user requests it or exports to a raster format.

### 67.3 Layout and artboards

Artboards should enable multiple named canvases within one project for related outputs such as social posts, ads, thumbnails, or alternate crops. Shared assets and local overrides must be distinguishable. Provide duplicate-artboard, resize-with-content options, per-artboard export, per-artboard background, layout guides, and batch naming templates.

Provide both absolute-pixel design workflows and print-layout workflows. Unit conversion among pixels, inches, centimeters, millimeters, and PPI must never unexpectedly resample image content. Resizing canvas, changing print dimensions, and resampling pixels are separate commands with separate previews.

### 67.4 Design-oriented quick actions

Contextual actions can include remove background, replace background, isolate subject, relight, add shadow, create outline, crop/resize, add border, make transparent, text fitting, center/align, match color, change color of selected object, generate variations, and export presets. Quick actions are command wrappers over the same document engine. They may create multiple standard layer/adjustment/mask entities, but must not create a special hidden document representation that advanced tools cannot edit.

---

## 69. Animation, timeline, and video capability plan

A broad feature target must explicitly account for Photoshop's frame and timeline capabilities, but VEGA should avoid allowing the video subsystem to delay the first still-image editing release.

### 69.1 Frame animation

Support frames that reference layer visibility/content states, frame delays, loop count, disposal behavior, frame duplication/deletion/reordering, playback preview, onion-skin support if helpful, and export to validated animated formats. Frame state must not be modeled solely as arbitrary snapshots of the entire document if a more compact, reliable frame/layer relationship is practical.

### 69.2 Keyframe timeline

The long-term timeline engine should define a time base, frame rate, timeline tracks, source in/out points, keyframed position/scale/rotation/opacity and supported effect properties, easing curves, work area, looping, caching, audio handling if later included, and deterministic render at a target time. Every animated parameter must have a unit and interpolation rule. Unsupported animation in imported content must be reported clearly.

### 69.3 Video-layer and sequence support

Plan an isolated media module to decode supported footage/image sequences, scrub frames, trim, composite text/shapes/still layers with moving footage, and apply supported per-frame retouching. Codec licensing, hardware decode, Windows build compatibility, and performance need a separate proof-of-concept. Do not claim support for a codec until there are test assets and redistribution rights are reviewed.

### 69.4 Export contract

Timeline export should explicitly specify resolution, frame rate, time range, output codec/format, alpha support, color space/profile, quality, audio policy where relevant, and estimated output size where available. Cancelled or failed jobs must clean temporary files safely and preserve the project. Exported video must be checked for frame count, duration, and frame pacing against the render settings.

---

## 81. Additional current feature-reference set

These official Adobe pages informed the version 1.4 capability map and should be checked again during future planning updates because product features change. They are references for product-category coverage, not a license to copy implementation or branded UI:

- Photoshop feature overview and current feature highlights: https://www.adobe.com/products/photoshop/features.html
- Selection tools overview: https://helpx.adobe.com/photoshop/desktop/make-selections/get-started-selections/selection-tools-overview.html
- Retouch tools overview: https://helpx.adobe.com/photoshop/desktop/repair-retouch/remove-objects-fill-space/retouch-tools-overview.html
- Layer groups: https://helpx.adobe.com/photoshop/desktop/create-manage-layers/create-layer-compositions/create-layers-and-layer-groups.html
- Layer masks: https://helpx.adobe.com/photoshop/desktop/create-masks/layer-masks/add-layer-masks.html
- Clipping masks: https://helpx.adobe.com/photoshop/desktop/create-masks/layer-masks/create-and-manage-clipping-masks.html
- Advanced blending options: https://helpx.adobe.com/photoshop/desktop/create-manage-layers/apply-layer-effects/use-advanced-blending-options.html
- Adjustment layers: https://helpx.adobe.com/photoshop/desktop/create-manage-layers/color-adjustment-layers/work-with-adjustment-and-fill-layers.html
- Color management: https://helpx.adobe.com/photoshop/desktop/adjust-color/color-profiles/color-management-in-photoshop.html
- Smart filters: https://helpx.adobe.com/photoshop/desktop/effects-filters/smart-filters/apply-and-manage-smart-filters.html
- Camera Raw local masks: https://helpx.adobe.com/camera-raw/desktop/edit-and-enhance-images/masking-and-local-adjustments/masking.html
- Camera Raw panorama merge: https://helpx.adobe.com/camera-raw/desktop/hdr-and-advanced-output/create-panoramas.html
- Generative features overview: https://helpx.adobe.com/photoshop/desktop/generative-ai/generative-ai-features-overview.html
- Generative editing with selections: https://helpx.adobe.com/photoshop/desktop/make-selections/refine-modify-selections/use-selections-for-generative-editing.html
- Timeline animation: https://helpx.adobe.com/photoshop/desktop/add-video-and-animation/use-keyframes/create-timeline-animation-workflow.html
- Video layers: https://helpx.adobe.com/photoshop/desktop/add-video-and-animation/create-animation-frames/video-layers-in-photoshop.html
- Automation scripts: https://helpx.adobe.com/photoshop/desktop/automate-tasks/automation-settings-and-presets/automate-photoshop-with-scripts.html
- Conditional actions: https://helpx.adobe.com/photoshop/desktop/automate-tasks/automation-settings-and-presets/add-conditional-actions.html
- Web export: https://helpx.adobe.com/photoshop/desktop/save-and-export/save-files/save-for-web.html

---

**Version 1.4 scope statement:** VEGA now has an explicit long-term coverage map for the major Photoshop capability families, plus a separate set of proposed improvements. This is a product specification and prioritized engineering plan; it is not evidence that any of the capabilities have been implemented, that all PSD/PSB constructs will be supported, or that VEGA has already reached Photoshop parity.

---

## 82. Detailed tool, panel, and command-family inventory

This inventory turns broad feature groups into discoverable product areas. It is intentionally more detailed than the release-one toolbar. Each item should have a backlog entry and status. Items identified as legacy or conditional should be investigated for user demand, format compatibility, and maintenance cost before implementation; the inventory is not a promise that every historical Photoshop command remains current or should be copied without rethinking it.

### 82.1 Navigation, view, and measurement tools

- Hand/pan tool and spacebar temporary pan.
- Zoom in/out, scrubby zoom where appropriate, zoom around cursor, fit on screen, fit width/height, actual pixels, print size where accurate, navigator preview, and zoom presets.
- Rotate canvas view without rotating image pixels; reset view orientation.
- Rulers, unit switching, drag-out guides, guide lock/clear, guide presets, grids, grid subdivisions, snapping toggles, smart guides, pixel snapping, and snap targets.
- Eyedropper with sample size, current layer/current composite/sample-all-layers choice, foreground/background assignment, and profile-aware color readout.
- Color sampler points, info readout, pixel-coordinate readout, dimension measurement, distance/angle measurement, and optional annotation/notes.
- Count/label tools for scientific or cataloging workflows where a stable use case is demonstrated.
- Proof setup/soft-proof view, gamut warning, gamut/proof toggle, proof-profile indicator, and before/after proof comparison.
- Canvas boundary, transparency checkerboard settings, outside-canvas pasteboard, artboard boundaries, bleed/safe-area overlays, and optional presentation/full-screen canvas.

### 82.2 Move, selection, and layout tools

- Move/transform, auto-select configuration, show transform controls, alignment, distribution, smart alignment, and selection of layers through the canvas.
- Rectangular/elliptical marquee, single-row/single-column selection if useful for exact pixel work, lasso, polygonal lasso, magnetic/edge-snapping lasso, quick selection, object selection, magic-wand/tolerance selection, color-range selection, focus-area selection where supported, and semantic select-subject/sky/object/people where models permit.
- Select/deselect, reselect, inverse, grow, similar, modify border, feather, smooth, expand, contract, transform selection, save selection, load selection, save mask/channel, and selection-to-path/path-to-selection.
- Quick-mask-style editable selection overlay. The UI must clearly distinguish a temporary selection overlay from a real layer mask and must not silently switch document targets.
- Crop, straighten, rotate, perspective crop, content-aware crop/expand when supported, crop presets, constraint/aspect ratio, delete/crop pixels option, non-destructive crop, and crop preview.
- Slice/export-region and slice-selection equivalents for web asset workflows, with a clear distinction from layers/artboards and no requirement for legacy slice features in the first release.
- Frame/placeholder tools that allow content to be dropped into a defined shape, with linked-source and clipping behavior clearly exposed.
- Perspective/vanishing-point grid or plane tools as an advanced geometry module, where a practical non-destructive model can be maintained.

### 82.3 Paint, drawing, and erasing tools

- Brush, pencil/hard-edge brush, airbrush behavior, mixer/wet brush, eraser, background-erasing behavior, fill, gradient, pattern fill, color replacement, and symmetry drawing.
- Brush settings: tip shape, dynamics, pressure, tilt, twist, spacing, scatter, angle, roundness, hardness, opacity, flow, smoothing, texture, dual tip, transfer, buildup, wetness/mix where supported, and brush preview.
- Clone stamp, pattern stamp, healing brush, spot healing, patch repair, red-eye correction, dodge, burn, sponge, blur, sharpen, and smudge.
- Foreground/background colors, color picker, swatches, sampler points, color mode, recent colors, color harmonies, gradient editor, and saved patterns.
- Tool-specific reset, preset save, preset import/export, safe preset migration, and the ability to revert to stock preset state without deleting user presets.
- Animation/symmetry or art-painting subfeatures should be modular. Legacy tools with low usage may be implemented as compatible workflows or omitted only through a documented scope decision, not through accidental disappearance.

### 82.4 Path, shape, and typography tools

- Pen, freeform pen, curvature-friendly path entry, anchor/node selection, direct selection, add/delete/convert anchor, handle adjustment, path closure, and path operations.
- Rectangle, rounded rectangle, ellipse, line, triangle, polygon, star, custom shape, freeform path, stroke/fill, dashed strokes, gradients, and object snapping.
- Horizontal and vertical type where supported by the text engine; point text; paragraph text; text in shape; text along path; editable warp; type-mask selection variants if users need pixel selection derived from glyph outlines.
- Character/paragraph control, glyph/character explorer, OpenType options, variable font axes, mixed formatting, language/script shaping, inline style runs, font fallback, missing-font detection, and outline/raster conversion explicitly invoked by the user.
- Text layer effects and reusable type styles. The source text must be retained for a non-destructive style adjustment; rasterization is never an implicit side effect of simple resizing or moving.

### 82.5 Layer, channel, and mask command families

- New layer/group/shape/text/adjustment/fill/object; rename, duplicate, delete, reorder, link, select, select similar, isolate, hide/show, lock, color label, tag, search, filter, and batch rename.
- Layer and group masks; vector masks; clipping; rasterize; convert to embedded/linked object; replace source; relink source; update linked instances; edit contents; reset transform; release clipping; merge down; merge visible; flatten; create merged-copy; artboard operations.
- Channel panel for color components, alpha/saved-mask channels, channel visibility, load channel as selection, save selection to channel, duplicate/delete/rename channel where supported, and channel calculations.
- Compositing modes and advanced blend controls, effect stacks, per-effect visibility, effect copy/paste, masks on effect stacks where the internal model supports them, layer comps/snapshots of selected visibility/position/appearance for alternate versions, and group compositing behavior.
- Layer diagnostics for empty content, duplicate labels, missing linked assets, unsupported effects, off-canvas content, and potentially destructive operations.

### 82.6 Adjustments, filters, and specialized image commands

- Automatic tone/contrast/color suggestions with before/after and explicit accept/reject.
- Levels, curves, exposure, brightness/contrast, highlights/shadows, hue/saturation, vibrance, color balance, black-and-white, channel mixer, selective color, photo filter, color lookup/LUT, gradient map, invert, threshold, posterize, solid color, gradient and pattern fills, plus clarity/texture/dehaze/grain as supported by color-space decisions.
- Gaussian/box/motion/radial/lens/depth-aware blur; field/iris/tilt-shift blur if implemented; sharpen, smart sharpen, high-pass, noise removal/addition, median, dust/scratch, despeckle, edge finding, emboss, stylize, pixelate, halftone, distort/displace, wave/ripple, twirl, pinch/spherize, lighting effects, render patterns, and convolution/custom filters.
- Liquify-like mesh distortion with face-aware options only if a model and user-consent/policy model are approved; puppet-style warp and perspective warp; adaptive wide-angle/lens correction; vanishing-point perspective painting/transform; content-aware scale; content-aware move/fill; sky replacement; and image harmonization.
- Filter Gallery-equivalent grouped preview, stack ordering, shared parameters, and batch application. Large multi-filter previews must remain cancellable and must identify when they are low-quality drafts.
- HDR tone mapping/merge and exposure merge, panorama/photomerge, focus stacking, stack modes/statistics, noise reduction through averaging, and other multi-image workflows as a separate photo-merge module.
- Neural/ML filter categories such as restoration, colorization, portrait enhancement, style transformations, depth/blur effects, super-resolution and denoise are optional modules subject to model licensing and quality/safety checks. Do not ship placeholder panels that imply functionality where no supported model exists.

### 82.7 History, automation, presets, and asset panels

- Undo, redo, history list, snapshots/checkpoints, history state grouping, current-document recovery, and before/after view.
- Brush, gradient, pattern, swatch, shape, style, LUT, action/recipe, workspace, template, and export preset libraries.
- Batch rename, batch resize/convert/export, metadata workflow, image stacks, conditional processing, action recording, script execution and documented application API.
- Assets panel for linked/embedded resources, imported media, recent placements, fonts references, missing dependencies, usage count, relink, and replace/update workflow.
- Library/community panel is optional and never required for local editing. Online content is marked as a network action; users can browse/download without automatically publishing their current work.
- Timeline/animation panel appears only when a document has or requests time-based content or the user opens it. It does not switch the entire app to a second studio.

### 82.8 File, import, export, and print command families

- New from clipboard/template/preset; open/import as document, layer, linked object, embedded object, sequence, or frame stack when supported.
- Place/replace linked or embedded asset; relink; package assets; missing-link repair; reopen/revert; duplicate document; save, save as, save copy, export current composite, export layers/artboards/slices, batch export, and output preflight.
- Format-specific options for lossless/lossy compression, quality, alpha, animation, dimensions, resampling, palette/indexed output, profile conversion/embedding, metadata retention/removal, and estimated output size.
- Print dimension/PPI handling, soft proofing, destination profiles, rendering intent where supported, image positioning, paper/background assumptions, and print-ready export. Direct hardware printing can be delayed if cross-platform support or driver behavior makes it unreliable; a tested print/export path remains the product goal.
- PDF/SVG import or export only where semantics and security limits can be defined. These formats can contain vectors, fonts, embedded raster assets, metadata, or complex constructs; never treat import as harmless text parsing.

### 82.9 Current and legacy capabilities policy

Some tools historically associated with Photoshop may be legacy, deprecated, platform-specific, external-product features, or omitted from current releases. The feature audit must therefore maintain three statuses: **current desktop capability**, **legacy/interoperability capability**, and **external companion workflow**. A legacy capability may still justify opening older projects or providing an import/export bridge, but does not automatically deserve a first-class tool in VEGA.

Classical 3D editing, deprecated filters, retired import/export workflows, and companion-app features must go through a product and maintenance review. If users need a 3D workflow, a safer future design may be 2D compositing with rendered 3D assets, optional model preview/placement, or an external-tool bridge rather than an entire 3D modeling engine inside VEGA. This decision remains open until scope and dependency costs are tested.

---

## 83. Final expanded product quality principles

1. **No fake functionality.** A control is either real, clearly marked as a future concept outside the shipped UI, or omitted from that build. No inert buttons dressed as finished tools.
2. **Editability first.** Preserve source pixels, vector geometry, text, linked assets, masks, adjustment parameters, and operations wherever practical. Make destructive actions intentional.
3. **One workspace, many workflows.** Workspace layouts organize panels; they never hide the product into separate modes or introduce multiple incompatible editing engines.
4. **Honest compatibility.** Every file format and advanced feature has a tested scope, visible limitations, and a clear loss report where required.
5. **Safety is part of UX.** File parsing, project recovery, plugin isolation, cloud permissions, and AI consent must be understandable and tested from the beginning.
6. **Offline independence.** Core image-editing and local project workflows remain usable without a network, account, cloud storage, or AI model.
7. **One command system.** Toolbar, menu, contextual action, recipe, script, and batch execution use the same documented command semantics.
8. **Measured performance.** Improvements are accepted based on realistic fixtures and reference hardware, not subjective claims that a project “feels fast.”
9. **Designed for change.** Schema migrations, plugin APIs, importers, models, render nodes, and cloud endpoints are versioned and observable.
10. **Ship complete slices.** Each implementation task delivers an end-to-end tested feature instead of a large scaffolding PR full of placeholders.
11. **User trust.** No hidden upload, secret telemetry of artwork, covert surveillance, or hostile anti-piracy behavior. Security controls protect users and the service rather than punish legitimate ownership.
12. **Clear ambition without false claims.** The goal is broad parity plus meaningful improvements. Full parity must be demonstrated feature-by-feature, not asserted before implementation.

---

**Master specification v1.4 historical note:** Sections 1–83 formed the consolidated planning source before the v1.5 innovation expansion. The intended long-term scope covers Photoshop's major current desktop capability families, relevant legacy interoperability, and a set of VEGA-specific improvements. The roadmap deliberately sequences that scope behind correctness, safety, and measurable quality; it does not imply that those features already exist in a running application.
