# VEGA Signature Innovations

> VEGA Planning Set v2.1 · Category document
> VEGA-specific product improvements and the ten signature systems, with acceptance criteria.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [72. VEGA improvements that go beyond parity](#72-vega-improvements-that-go-beyond-parity)
- [84. VEGA's product identity: original capabilities, not cosmetic imitation](#84-vegas-product-identity-original-capabilities-not-cosmetic-imitation)
- [85. VEGA signature system 1 — Branchbook: real creative branches, not just undo history](#85-vega-signature-system-1-branchbook-real-creative-branches-not-just-undo-history)
- [86. VEGA signature system 2 — Edit Atlas: explain what changed, where it changed, and why](#86-vega-signature-system-2-edit-atlas-explain-what-changed-where-it-changed-and-why)
- [87. VEGA signature system 3 — Design Contracts: tell the document what must remain true](#87-vega-signature-system-3-design-contracts-tell-the-document-what-must-remain-true)
- [88. VEGA signature system 4 — Variant Matrix: one creative, many deliverables](#88-vega-signature-system-4-variant-matrix-one-creative-many-deliverables)
- [89. VEGA signature system 5 — Change Lens: inspect the real impact of every edit](#89-vega-signature-system-5-change-lens-inspect-the-real-impact-of-every-edit)
- [90. VEGA signature system 6 — Source Ledger and Rights Passport](#90-vega-signature-system-6-source-ledger-and-rights-passport)
- [91. VEGA signature system 7 — Recipe Studio: inspectable, testable, reusable editing workflows](#91-vega-signature-system-7-recipe-studio-inspectable-testable-reusable-editing-workflows)
- [92. VEGA signature system 8 — Output Lab: predict the delivery, not just export a file](#92-vega-signature-system-8-output-lab-predict-the-delivery-not-just-export-a-file)
- [93. VEGA signature system 9 — Semantic Canvas and universal creative search](#93-vega-signature-system-9-semantic-canvas-and-universal-creative-search)
- [94. VEGA signature system 10 — Learn by Replay and shareable project explanations](#94-vega-signature-system-10-learn-by-replay-and-shareable-project-explanations)
- [95. Innovation integration, data model, and phased delivery](#95-innovation-integration-data-model-and-phased-delivery)
- [96. Differentiation review and launch proof](#96-differentiation-review-and-launch-proof)

---

## 72. VEGA improvements that go beyond parity

The following ideas are differentiators to validate through prototypes and user testing. They should be implemented as additions to the core document/command model rather than isolated gimmicks.

### 72.1 Unified intent-to-command system

A command search accepts tool names, commands, natural-language descriptions, and synonyms. It must be usable with conventional deterministic matching offline; optional AI may interpret more complex phrasing but is not required. Results should show what the command will affect and whether it is destructive, non-destructive, local, or cloud-based. For example, “make the sky darker” should offer a deterministic selection/adjustment workflow and separately mark any AI-based interpretation.

### 72.2 Editable operation recipes

Users can save a repeatable recipe consisting of normal VEGA commands and parameters. A recipe can be inspected, reordered, parameter-edited, shared, and applied to another document. On incompatible documents, VEGA offers a dry run and identifies operations that cannot transfer. Recipes should be importable/exportable as signed or hashed declarative packages, not arbitrary scripts by default.

### 72.3 Document health and explainability

A single Document Health view identifies missing linked files, fonts, unsupported imported layers, hidden objects outside canvas, unusual color profiles, enormous source assets, filters with failed dependencies, unsaved edits, possible export clipping, and project version issues. Each warning includes a concise explanation, impact, and safe action. Warnings are deduplicated and severity-coded; not every minor issue becomes a blocking modal dialog.

### 72.4 History that users can understand

History should group operations into understandable workflows, show command parameters, provide snapshots, and support an optional named checkpoint. Where architecture allows it, users can compare before/after or return to a prior checkpoint without erasing subsequent work. A full branched history is a later advanced feature and must be built only after memory accounting and command replay have been proven. Do not promise infinite undo; expose memory/storage policy and preserve named checkpoints safely.

### 72.5 Visual compare and change auditing

Include split view, side-by-side, wipe compare, difference overlay, blink compare, and zoom-locked comparison for selected layers, filters, and full document versions. Difference view must account for alpha and color profiles. A project can optionally retain a human-readable edit log, but sharing/exporting this history is opt-in because it may disclose sensitive workflow information.

### 72.6 Contextual tool explanations

Every complex tool should have a short explanation, a mini interactive example where sensible, reset-to-default, and links to deeper reference. Explain parameter interactions at point of need. Tutorials must not block the workspace or change the actual document unless the user chooses “try on a copy”.

### 72.7 Smarter presets and safe bulk editing

Preset previews should show representative thumbnails using the current or sample image, indicate required modules/fonts/assets, and compare parameter changes. Batch operations use a preview list, include skip/error behavior, never overwrite original assets by default, and generate a manifest of results and failures.

### 72.8 Pro workflow modes without separate studio modes

VEGA must not create separate Quick/Pro studios. It may offer customizable **workspace layouts** such as Photo Retouch, Composite, Illustration, Typography, Print Prep, and Animation. These are panel arrangements and tool favorites only; they never change available features, hide the existence of the full editor, convert a document, or activate a separate rendering pipeline. Users can reset to a common default.

### 72.9 Reusable component/asset system

Develop linked reusable assets for logos, product shots, icons, patterns, and text treatments. A document may link to an external file or embed it. Link status, expected content hash, update preview, conflict handling, and fallback rendering must be explicit. Updating a source must support previewing affected instances before applying.

### 72.10 Practical project handoff

Offer a handoff report for production files listing color profile/bit depth, fonts, linked sources, missing resources, plugin dependencies, unsupported imports, and export presets. An optional package mode can collect legally redistributable project dependencies into a folder without embedding secrets or unrelated recent-file history.

### 72.11 Quality-aware output assistant

An export preflight can detect likely issues: low effective resolution, accidental transparency, clipping beyond canvas, missing fonts, incorrect color profile, excessive JPEG compression, alpha loss, unembedded profile, hidden layers, and unsupported layers. It must explain each result and let the user proceed intentionally. It cannot guarantee that an image is free of all production errors.

### 72.12 Extensible, pluggable rendering pipeline

The operation graph should have stable contracts for inputs/outputs, color/alpha semantics, capability declarations, versioning, cancellation, determinism class, tile locality, memory estimation, and cache invalidation. This enables future filters and plugins without hardcoding every new operation in the UI. The first native renderer remains the trusted reference implementation for core math.

### 72.13 Optional local privacy controls

Allow users to disable recent-file names, telemetry/diagnostics beyond strictly necessary operational signals, cloud integration, online resource browsing, and any model inference. Diagnostic bundles must preview included paths/logs before sharing. Do not collect document pixel contents, layer names, filenames, prompts, or thumbnails by default. Any opt-in crash reproduction that contains artwork must warn and require explicit consent.

---

## 84. VEGA's product identity: original capabilities, not cosmetic imitation

VEGA's differentiation must not depend on copying another application's exact screen, names, artwork, or branding. It should come from how the product models a creative project, makes change understandable, protects user intent, and moves one piece of work across many outcomes. The goal is to cover the professional capability families users expect while developing coherent workflows that are recognizably VEGA.

### 84.1 Competitive honesty policy

No feature in this specification may be described in marketing as something a competitor definitively lacks until the current shipping products have been checked against a dated, versioned comparison matrix. Professional image editors change frequently. For example, Adobe's current 2026 product materials describe generative editing, contextual assistance, generated variations, adjustment layers, layer compositions, cloud search, local/online processing options, and more. These are existing competitive capabilities, not differentiators VEGA should claim to have invented. See the current official Photoshop feature and release pages: https://helpx.adobe.com/photoshop/desktop/whats-new/whats-new-in-adobe-photoshop-on-desktop.html and https://helpx.adobe.com/photoshop/desktop/generative-ai/generative-ai-features-overview.html.

VEGA should instead use three labels in product planning:

- **Expected capability:** a familiar capability needed for credible professional workflows.
- **VEGA signature:** a designed system whose combination, defaults, persistence, integration, and UX are intended to distinguish VEGA. This label is a product hypothesis, not proof that no other tool has a similar feature.
- **Research required:** a capability for which a competitive audit, legal review, user research, or technical prototype is needed before public differentiation claims.

Before a public claim such as “only in VEGA” or “Photoshop cannot do this,” a product owner must verify the competitor's current desktop, web, beta, and companion workflows, record the checked version/date, and retain evidence. Prefer claims about what VEGA guarantees—such as transparent change history or offline availability—over unsupported exclusivity claims.

---

## 85. VEGA signature system 1 — Branchbook: real creative branches, not just undo history

**Product promise:** explore materially different directions without duplicating entire projects, losing work, or turning a single undo stack into a maze.

### 85.1 User experience

Every project can have named directions such as “warm campaign,” “night version,” “minimal product shot,” or “client-safe.” A compact Branchbook panel displays a visual tree of named branches with previews, creation points, labels, timestamps, and the main differences from their parent. Users can:

- create a branch from the current state or from a saved checkpoint;
- switch branches without overwriting another branch;
- compare two branches in a synchronized split, wipe, flicker, and difference view;
- promote a selected branch as the primary version;
- copy selected, compatible edit operations from one branch to another;
- see conflicts when both branches changed the same underlying layer, asset, mask, or parameter;
- archive or delete a branch with a recovery window;
- package only a chosen branch for export or sharing.

### 85.2 Correctness model

Branchbook is not simply a layer-comp snapshot and must not be implemented as a full duplicate of every pixel buffer. It uses immutable checkpoints where practical, stable object IDs, copy-on-write pixel tiles, operation records, and explicit parent/child relationships. Each branch has a base checkpoint plus its own change sequence. The renderer resolves the active branch into the ordinary VEGA document composite.

A “merge” means applying selected operations from one branch to another only when preconditions and target identities can be validated. Conflicts must be shown instead of silently choosing a winner. If an operation depends on a layer that was deleted, renamed beyond stable-ID reach, flattened, or structurally changed, it should be marked as not safely portable. Do not offer a misleading universal merge button.

### 85.3 Persistence and interchange

The `.vega` format will reserve a versioned branch metadata section. PSD export exports one selected branch at a time unless a future interchange specification provides a safe representation. Export dialogs must state that branch history and branch metadata will not be carried into ordinary PSD, PNG, or JPEG files. Optional branch previews should not expose unpublished work when a project is shared.

### 85.4 Acceptance criteria

- Creating and switching branches never modifies another branch's content.
- Branching a large document reuses unchanged pixel tiles where implementation permits and does not naïvely duplicate all full-resolution assets.
- Branch comparisons register to the same canvas coordinates and can highlight changed regions.
- A conflicting operation is visible and cannot silently overwrite a changed target.
- Save, close, reopen, crash recovery, and format migration preserve the branch structure.
- Users can flatten/export a chosen branch without accidentally uploading the branch tree or other private variants.

---

## 86. VEGA signature system 2 — Edit Atlas: explain what changed, where it changed, and why

**Product promise:** a project is not a mysterious pile of layers. VEGA can trace a visible result to the layers, masks, adjustments, sources, and operations that contributed to it.

### 86.1 Atlas view

Edit Atlas is a browsable provenance graph associated with the document. A user can select a visible region or a layer and see a graph of relevant contributors: source asset, transform, mask, adjustment, composite, retouch operation, and export stage. It may answer questions like:

- Which layer or operation changed the color in this area?
- Which source file produced this placed image?
- Which adjustments affect this subject?
- Why does this layer render differently in this output?
- What would be affected if I changed or removed this node?

Atlas links graph nodes to normal layer rows and property controls. It is not a second editor and does not introduce an alternative document model.

### 86.2 Implementation limits and trustworthy explanations

The first version should trace explicit document graph relationships, masks, blend/adjustment dependencies, and operation metadata. It must not claim exact per-pixel causality for operations where the renderer has not recorded enough information. At the pixel level, show “contributing input/operation” or “estimated affected region” where exact attribution cannot be proven. Future dependency maps can use tile invalidation records and operation-declared input/output bounds.

For AI-produced content, Atlas records the generating operation, model/provider identifier when disclosed, settings that may safely be retained, seed/variation identifier where supported, source references the user provided, and whether the result was subsequently edited. Never record hidden provider secrets or credentials. AI-generated content remains an editable result layer when technically possible.

### 86.3 Acceptance criteria

- Selecting a layer can navigate from its row to its Atlas node and back.
- The graph clearly distinguishes exact relationships from estimates.
- Atlas does not perform edits merely because the user selects a node.
- Renaming or moving a layer preserves stable identity and Atlas links.
- Removing or flattening a source updates the graph and records any lost provenance honestly.
- Atlas data is stored locally in `.vega`; public sharing excludes private notes and unpublished branches by default.

---

## 87. VEGA signature system 3 — Design Contracts: tell the document what must remain true

**Product promise:** users can set explicit constraints for a composition, and VEGA checks those constraints as edits are made instead of relying on the user to discover problems at export time.

### 87.1 Examples of a contract

A contract is a set of opt-in rules attached to a document, artboard, layer, output preset, or selection. Examples:

- keep a logo inside a safe margin;
- do not alter a designated logo or legal-text layer during AI or batch edits;
- preserve a subject within a crop-safe region across target aspect ratios;
- keep headline text above a chosen minimum size for a defined output;
- maintain text/background contrast above a chosen threshold;
- ensure a print document uses a required color profile, dimensions, and resolution;
- preserve exact brand colors on layers tagged as brand assets;
- warn if a required linked asset or font is missing;
- prevent accidental clipping of an approved product silhouette.

Rules can be advisory, warning-level, or blocking. The user who owns the document can override a rule after a clear explanation unless the rule is imposed by an organization policy explicitly configured by that organization in a future enterprise edition. Initial VEGA should have only user-controlled contracts; do not introduce hidden remote lockouts.

### 87.2 Constraint engine

Constraints should run through a testable validation service, not be hardcoded into individual controls. Each contract has a stable ID, scope, version, severity, validation function, user-facing reason, and possible repair suggestions. Some rules are exact (pixel dimensions, profile, bounds); others are heuristic (legibility, subject position, visual contrast in complex imagery). Heuristic checks must be labeled as suggestions, not guarantees.

When an operation violates a contract, preview the predicted violation before committing when possible. Support “apply anyway,” “adjust,” and “cancel.” For a batch operation, show affected files/variants before running the batch.

### 87.3 Acceptance criteria

- Contract evaluation is deterministic for exact rules and versioned for changing heuristics.
- The UI tells the user what rule was violated and where.
- Users can temporarily disable or override user-defined rules.
- Contract metadata is persisted and migrated with the project.
- The export preflight reruns applicable checks against the actual output settings.
- Contracts never silently rewrite artwork without user approval.

---

## 88. VEGA signature system 4 — Variant Matrix: one creative, many deliverables

**Product promise:** build a family of related deliverables from one structured creative source without manually maintaining dozens of nearly identical files.

### 88.1 Core behaviors

Variant Matrix defines multiple outputs and controlled variables inside one VEGA project. A campaign might include a square post, portrait story, landscape banner, print poster, alternate headline, localized text, or a product color variation. Each variant may define:

- canvas size, aspect ratio, and export profile;
- named text slots and replacement text;
- image/source slots and approved asset choices;
- visibility or parameter overrides for selected layers;
- layout constraints and safe areas;
- output filename pattern, format, and quality settings;
- rules that inherit from the base composition or override it intentionally.

A shared edit to a source layer or common adjustment propagates to variants that inherit it. A variant-specific change remains local and is shown as an override. The interface highlights inherited vs. overridden settings so users do not have to guess why one output differs.

### 88.2 Layout adaptation

Aspect-ratio conversion must not merely scale everything down. The layout engine should respect anchors, safe areas, locked/relative sizes, text fitting, subject protection, and user-defined priorities. It can propose a crop or reflow but should show a preview and allow manual correction. Semantic detection for people or objects may be optional and model-dependent.

This system is not intended to replace a vector design program or full page-layout system. Its initial scope is reliable families of raster-first social, advertising, product, and photo deliverables.

### 88.3 Acceptance criteria

- One source edit updates all inheriting variants and leaves local overrides intact.
- Exporting all variants produces deterministic names, dimensions, profiles, and expected files.
- Preview can display a matrix of variants and mark contract violations.
- Users can detach a variant when it must evolve independently.
- Missing fonts/assets and text overflow are reported before batch export.
- A native project reopens with its variant relationships intact.

---

## 89. VEGA signature system 5 — Change Lens: inspect the real impact of every edit

**Product promise:** users can understand an edit before accepting it and verify what it actually changed afterward.

Change Lens is a standardized preview/verification system usable by brushes, filters, retouch tools, transforms, batch recipes, AI results, and plugins that declare preview support. It combines four views:

1. **Before/after alignment:** synchronized zoom/pan and a wipe/flicker comparison.
2. **Difference map:** highlights regions where output differs from source, with adjustable sensitivity and a legend explaining that color-managed image differences may be tiny or expected.
3. **Affected-region overlay:** shows the declared or measured region a command intends to change.
4. **Change summary:** names affected layers/nodes and surfaces relevant warnings, such as edits outside a selection or changes to protected content.

### Integration contract

A command may advertise preview modes: exact preview, approximate draft, or no preview. The interface must label draft output and never imply it is final quality. Long previews run asynchronously, can be cancelled, and must not mutate the document until committed. Commands declare input dependencies, proposed outputs, invalidated caches, affected bounds when known, and whether they are reversible.

For operations that legitimately change the entire image—such as global curves or color-space conversion—the difference map must not treat a full-canvas change as inherently suspicious. The summary explains the expected scope.

### Acceptance criteria

- A preview is discardable without altering document pixels, parameters, history, or dirty state.
- Committing a preview creates one coherent history transaction.
- Cancelling a preview frees temporary buffers and stops cancellable work.
- Comparison does not change the active branch, selection, or zoom state unexpectedly.
- The difference map is computed against a documented color-space/metric policy and can be recomputed after output color conversion.

---

## 90. VEGA signature system 6 — Source Ledger and Rights Passport

**Product promise:** the user can determine where a placed asset came from and what rights/attribution information they have, then package a handoff that does not forget important sources.

### 90.1 Source ledger

Each placed image, font reference, brush, pattern, LUT, plugin-provided asset, linked file, or generated output can optionally carry a source record with fields such as original filename, user-entered author/source, source URL, acquisition date, license/permission note, license text or license identifier, attribution requirement, modifications made, and an optional cryptographic hash. Metadata should be added only when known or explicitly supplied. A missing record means “unknown,” not “unlicensed” and not “safe to use.”

For generated content, record only accurate available information: local/provider model identifier, generation date, prompt if the user elects to keep it, reference asset IDs, and modification history. Give users control over whether prompts or private source URLs are embedded in a share package.

### 90.2 Rights Passport export

An optional project handoff can include a source manifest, required attribution text, linked asset list, font status, license notes, and file hashes. Before publishing to the community, a preflight warns about missing rights information, private notes, personal metadata, unpublished source URLs, and embedded assets that should not be shared. It must not offer legal advice or imply that a hash proves ownership.

### 90.3 Anti-theft boundaries

A Source Ledger improves traceability but cannot prevent screenshotting, copying, re-encoding, or removing metadata from a public image. Optional watermarking can deter casual reuse and help identify a source, but it is not cryptographic proof by itself. Do not add invasive fingerprinting, hidden tracking pixels, or covert telemetry.

### 90.4 Acceptance criteria

- Source records distinguish known, unknown, user-asserted, and tool-recorded fields.
- Export/handoff reports missing linked dependencies and required attribution where supplied.
- Public uploads exclude private notes and secrets unless the user explicitly includes approved fields.
- Importing a project preserves records where the source format supports them and reports metadata that cannot be represented.
- Removing a source record requires explicit confirmation and does not delete artwork content.

---

## 91. VEGA signature system 7 — Recipe Studio: inspectable, testable, reusable editing workflows

**Product promise:** turn a successful sequence into a reusable recipe that anyone can inspect, adjust, apply to another image, or share—without requiring AI or scripting knowledge.

Recipe Studio evolves beyond a simple action recorder. A recipe is a typed, versioned sequence/graph of VEGA commands and parameters. It can contain user input steps, masks, conditional checks, output targets, and optional human review gates. Each step has a description, required inputs, defaults, compatibility tags, and a defined failure policy.

Examples include product-photo preparation, portrait cleanup, thumbnail export, batch resize/rename, preparing a print file, or creating a family of campaign variants. A recipe can be run on one file or a batch. The runner shows a preview and a summary before batch execution, records per-file results, and skips/flags a failed item according to policy rather than silently producing partial output.

### Determinism and safety

- A recipe version pins command identifiers, schema versions, and parameters.
- Upgrades use migrations and show changed behavior before rerunning old recipes.
- Recipes run in a bounded environment; plugin/script steps require explicit permission and cannot quietly access arbitrary paths or networks.
- AI steps, when used, are clearly labeled as potentially non-deterministic and record the selected model/provider and available parameters. A recipe without AI remains runnable offline.
- Batch operations use temporary outputs or checkpoints to avoid corrupting source files.
- The UI differentiates “preview,” “apply to active document,” “apply to copies,” and “overwrite originals.” Overwriting original files is never the default.

A recipe can be generated from selected history entries, but VEGA must check whether those entries can be replayed. Brush strokes dependent on undisclosed transient input or plugin operations unavailable on another computer should be marked as non-portable rather than falsely exported as complete.

### Acceptance criteria

- Each recipe can be validated without running it on source files.
- A batch run creates a manifest of inputs, outputs, skipped items, failures, and warnings.
- Cancelled or failed runs preserve completed safe outputs and clearly report their status.
- Recipes are versioned, importable/exportable, and can be previewed before trust is granted.
- A recipe cannot bypass plugin permission boundaries or cloud storage policy.

---

## 92. VEGA signature system 8 — Output Lab: predict the delivery, not just export a file

**Product promise:** see how a design is likely to survive its destination before committing an export.

Output Lab combines export settings, delivery simulation, target requirements, and preflight in one view. Possible proof profiles include social-media resizing/compression, web transparency behavior, print dimensions/profile/resolution, sRGB browser delivery, thumbnail legibility, and chosen file-size ceilings. These are simulations based on documented rules, not a promise that every third-party platform will preserve the exact file after its own processing.

### Behaviors

- Preview the expected downsample/compression and inspect likely artifacts at native and target display sizes.
- Toggle target backgrounds for transparent assets and inspect premultiplied-alpha edge behavior.
- Check target dimensions, profile, bit depth, alpha support, metadata policy, and expected size.
- Show whether text or important objects may be cropped at a configured safe area.
- Provide separate “export this version” and “export all variants” flows.
- Offer a manifest of generated files and their settings.
- Detect when converting color profiles or bit depth will change appearance or discard data.

### Acceptance criteria

- The simulated export uses the same codec, profile conversion, resampling, and parameters as the actual export whenever possible.
- Any remaining simulation approximation is stated in the UI.
- A proof profile never mutates the source document.
- Export presets are versioned and can be inspected or reset.
- The output manifest records exact filenames, dimensions, formats, profiles, and successful/failed outputs.

---

## 93. VEGA signature system 9 — Semantic Canvas and universal creative search

**Product promise:** users can work with the meaning of objects and design elements, not only their layer names or pixel coordinates.

The Semantic Canvas is a lightweight, user-correctable index for roles explicitly marked by the user or inferred by optional local analysis. Examples include subject, background, product, logo, headline, legal text, shadow, frame, mask, or reference image. Semantic tags do not replace stable IDs or pixel geometry; they are a layer of metadata that enables powerful navigation and validation.

Universal Creative Search can search commands, settings, layers, groups, assets, presets, recipes, variants, source records, and help. Example queries might be “layers marked logo,” “all hidden text,” “objects outside this artboard,” “recipes that export transparent PNG,” or “why is this font missing.” Natural-language parsing can be a convenience layer, but all destructive results must resolve to visible, inspectable targets and explicit commands.

### Privacy and accuracy

- Do not upload document contents for search by default. Local indexing remains local.
- An inferred semantic tag is shown as a suggestion with confidence/uncertainty and can be corrected or removed.
- Do not promise that the system recognizes every object or text element.
- Search results must distinguish a found command from a proposed command and from a result that needs model analysis.
- Searching never means editing. The user explicitly chooses an action after reviewing matches.

### Acceptance criteria

- Every search result navigates to an existing command, object, or help topic.
- Semantic tags survive save/reopen and can be disabled or removed.
- User-corrected labels take precedence over model suggestions.
- No document text, layer thumbnails, prompts, or source assets are sent remotely without an explicit online action and privacy notice.

---

## 94. VEGA signature system 10 — Learn by Replay and shareable project explanations

**Product promise:** a creator can turn a project into an understandable walkthrough, and another user can learn how the result was made without needing to guess from a flattened image.

Learn by Replay can display a selected sequence of edit operations as a guided timeline. It can show the relevant layer, parameter delta, short user note, before/after comparison, and the resulting visual state at each step. Users may create a tutorial from replayable history, reorder or remove explanation steps, add captions, and export a compact tutorial package or rendered walkthrough.

This is not a promise to reconstruct every mouse movement or record the screen continuously. It relies on recorded document operations and user-selected explanation points. The source project, personal notes, prompts, and private branches remain excluded unless explicitly selected.

### Acceptance criteria

- A replay step identifies the underlying operation, not just a screenshot.
- A tutorial can be opened without granting permission to execute scripts or plugins.
- Non-replayable or unavailable-plugin steps are visibly marked.
- Export supports redaction of filenames, user identifiers, prompts, and source URLs.
- A learner can inspect steps and parameters without modifying the original project.

---

## 95. Innovation integration, data model, and phased delivery

The features above should feel like one coherent VEGA system, not ten unrelated experimental panels. Their integration relies on the stable document model, command system, history, tile cache, asset registry, export pipeline, and permissions model already defined elsewhere in this specification.

### 95.1 Required shared primitives

- Stable document, layer, asset, operation, branch, variant, and contract IDs.
- Versioned typed command schemas with preconditions, reversible behavior when possible, invalidation scope, preview capabilities, and failure/cancellation semantics.
- Explicit distinction between document content, document metadata, user-private metadata, public-share metadata, and device/account service data.
- Copy-on-write or equivalent storage for shared immutable content, with memory limits and garbage collection.
- A validated project migration framework and corruption-safe save/recovery paths.
- A permission-aware asset/recipe/plugin subsystem.
- A rendering/output proof API that can compute actual output settings without changing the source project.

### 95.2 Suggested priority

| Priority | Feature | Rationale |
|---|---|---|
| P0 foundation | Change Lens core hooks and stable command metadata | These primitives improve debugging and preview safety across many tools. |
| P1 | Output Lab and richer project diagnostics | Immediate value for export reliability and beginner confidence. |
| P1/P2 | Recipe Studio for core commands | Makes repeatable workflows and batch operations coherent. |
| P2 | Design Contracts and Semantic Canvas tags | Improves multi-output design correctness and search. |
| P2/P3 | Variant Matrix | High value for creator/design workflows; depends on stable layers, text, constraints, and export. |
| P3 | Branchbook | Powerful but requires stable IDs, immutable checkpoints, diffs, migration, and conflict behavior. |
| P3 | Source Ledger and Rights Passport | Build basic asset metadata early; formal handoff manifests after formats and community sharing stabilize. |
| P3 | Edit Atlas | Start with graph navigation and explicit dependencies; refine pixel attribution only where measurable. |
| P3 | Learn by Replay | Depends on reliable operation history and recipe portability. |

Do not start all signature systems at once. Implement a small complete end-to-end slice, measure user benefit, and evolve the data model before making the schema permanent.

### 95.3 Data-model migration rules

Every stored innovation object has a `schemaVersion`, stable ID, creation/update metadata, and defined fallback behavior. Older documents with none of the new metadata must open normally. Unknown optional innovation fields should be preserved when safe to do so. If a future application cannot safely preserve a new field, it must warn before saving in a way that drops it. Branch graphs and shared asset references require integrity validation.

Do not store the same authoritative relationship independently in the UI and renderer. The Rust `vega-core` crate owns the document graph and command semantics; `vega-render` owns pixel/tile processing and rendering primitives. Innovation systems consume typed Rust module APIs and operation contracts rather than mutating renderer memory directly.

---

## 96. Differentiation review and launch proof

A VEGA signature feature is complete only when its value is demonstrated with a test or research artifact. It is not enough to have a panel named Branchbook or an “AI” button.

### Product proof required

For each signature system, maintain:

- problem statement and target user;
- competitive audit with exact product/version/date and source references;
- a prototype and usability tasks;
- measurable success criteria;
- document-format and migration impact;
- performance/memory budget;
- privacy/security threat review;
- manual and automated test fixtures;
- accessibility review;
- feature flag/rollback plan;
- final disposition: ship, revise, defer, or reject.

### Suggested success measures

- **Branchbook:** users can compare directions and recover prior ideas without creating repeated full-file copies; measure task time and accidental overwrite rate.
- **Edit Atlas:** users can explain a region's contributing editable sources with fewer layer-hunting actions; measure correct source identification.
- **Design Contracts:** fewer avoidable export failures and fewer accidental violations of user-defined constraints.
- **Variant Matrix:** less manual duplicated work across output sizes and variants, with a low rate of unintended overrides.
- **Change Lens:** users detect unwanted edits before committing more reliably than without the preview.
- **Source Ledger:** users can produce source/attribution manifests without manually reconstructing asset origins.
- **Recipe Studio:** repeat runs produce predictable results and report partial failures clearly.
- **Output Lab:** exported assets more often match intended size, profile, crop, alpha, and quality targets.
- **Semantic Canvas:** search reduces time to locate meaningful layer/object groups without uploading artwork.
- **Learn by Replay:** creators can generate useful tutorials from real operations without exposing private project information.

### Final differentiation rule

VEGA is not “unique” merely because it has a list of features that other editors also have. The intended identity is their integrated behavior: **traceable edits, branch-safe experimentation, explicit design contracts, reusable recipes, multi-output inheritance, rights-aware asset handling, and delivery proofs—all sharing one editable document model and one coherent workspace.** Whether any individual feature is exclusive must be researched and proven, not assumed.

### References for maintaining competitive accuracy

- Adobe, current Photoshop desktop release notes: https://helpx.adobe.com/photoshop/desktop/whats-new/whats-new-in-adobe-photoshop-on-desktop.html
- Adobe, generative AI feature overview: https://helpx.adobe.com/photoshop/desktop/generative-ai/generative-ai-features-overview.html
- Adobe, non-destructive editing techniques: https://helpx.adobe.com/photoshop/desktop/create-manage-layers/create-layer-compositions/non-destructive-editing-techniques.html
- Adobe, layer compositions: https://helpx.adobe.com/photoshop/desktop/create-manage-layers/create-layer-compositions/create-and-manage-layer-comps.html

**Master specification v1.5 historical note:** Sections 1–96 define the product and innovation scope carried into planning set v2.0 and v2.1. The v2.1 files preserve that feature vision while using C#/.NET 11 + Avalonia for desktop UI, Rust for the shared editing core, Kotlin/Compose for Android, and TypeScript for the backend. Sections 84–96 add an original product-differentiation strategy without asserting unverified competitor exclusivity. Every planned capability remains subject to implementation, validation, security review, and acceptance tests before it is considered shipped.
