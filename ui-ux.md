# UI & User Experience

> VEGA Planning Set v2.1 · Category document
> The unified workspace, detailed design system, user journeys, and interaction rules.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [C#/.NET rules](dotnet-development.md) · [Rust core rules](rust-development.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [3. The single unified workspace](#3-the-single-unified-workspace)
- [42. Detailed visual design system and UI behavior](#42-detailed-visual-design-system-and-ui-behavior)
- [43. End-to-end user journeys and acceptance behavior](#43-end-to-end-user-journeys-and-acceptance-behavior)
- [73. Unified UI behavior for advanced capability without clutter](#73-unified-ui-behavior-for-advanced-capability-without-clutter)

---

## 3. The single unified workspace

There is **one editor**. There is no mode switch that changes the application into a reduced editor or a professional editor. VEGA stays approachable through contextual UI, progressive disclosure, sensible defaults, command search, and customizable panels.

### Proposed desktop layout

1. **Application/menu bar:** File, Edit, Image, Layer, Select, Filter, View, Window, Help; document tabs; undo/redo; zoom; save state; export.
2. **Left tool rail:** move, selection, crop, brush, eraser, fill/gradient, text, shape, pen/path, eyedropper, hand, zoom, and later retouching tools.
3. **Contextual options strip:** options for the active tool or selected object. It changes context without hiding advanced properties.
4. **Central canvas:** image, transparency checkerboard, selection borders, transform handles, guides, rulers, snapping, and zoom/pan controls.
5. **Right-side dock:** Properties, Layers, Adjustments, Color, History, and Assets. Panels can be resized; later milestones add docking, panel rearrangement, and saved layouts.
6. **Command search:** searchable commands, tools, and settings, with discoverable shortcuts. Commands are invoked through the same application command layer as menus and buttons.
7. **Bottom status area:** zoom, canvas dimensions, active tool feedback, color readout when relevant, background-job progress, and memory warnings when useful.
8. **Start/recent screen:** create a document, open a file, reopen recent projects, or start from a size preset. It is a launch surface, not a second editing mode.

### Progressive disclosure rules

- Display the few most relevant controls for the current selection prominently.
- Keep advanced settings available in Properties, expandable sections, or dedicated panels.
- Do not remove a professional capability merely to make the screen look simpler.
- Quick actions are shortcuts into the normal editable document model. They are not separate destructive filters hidden behind a different workflow.
- Examples: “Remove background” creates or edits a mask; “Improve lighting” creates editable adjustments; “Resize” uses the same document/image-size commands available in menus.
- Use consistent labels, keyboard focus behavior, undo semantics, and error messages across all entry points.
- Make panel layouts restorable to a known default if users rearrange the workspace.

### Interaction requirements

- Keyboard shortcuts for common actions: open, save, save as, undo, redo, zoom, hand/pan, tool switching, deselect, transform, and export.
- Configurable shortcuts can follow after the initial stable shortcut map.
- Support mouse, keyboard, high-DPI displays, Windows scaling, and stylus pressure where the platform/API permits it.
- Every icon-only control needs a tooltip and accessible name.
- Selection and transform handles must remain aligned with the canvas at different zoom levels and display scaling settings.
- Long operations must run outside the UI thread, report progress, support cancellation where safe, and never corrupt the current document on cancellation.

---

## 42. Detailed visual design system and UI behavior

This section turns the unified-workspace direction into implementation requirements. It is a behavioral specification, not a requirement to copy another product's exact interface or assets.


### 42.0 C# / Avalonia UI implementation notes

The desktop UI prototype uses C#/.NET 11 and Avalonia UI/XAML, with MVVM for presentation state and a custom canvas control for artwork interaction. Use Avalonia templates, styles and reusable controls to create a polished VEGA visual identity. This does not remove the responsibility to support keyboard navigation, text composition/IME, focus, screen scaling, accessibility, and clear errors.

- Keep XAML views and code-behind focused on presentation and platform/control integration. Commands and workflow state belong in view models/application services; the authoritative document state remains in Rust.
- Dock state and workspace layout are separate from the project document, versioned, and recoverable. A broken panel layout must never prevent opening a project. Verify an actual docking package or begin with stable fixed/resizable panel zones; Avalonia `DockPanel` alone is not a complete floating docking framework.
- The canvas is a viewport into the document, not the document itself. Keep one tested transform between screen coordinates and document coordinates; tools, selection edges, brush cursors, and transform handles must use it consistently.
- Viewport bitmaps, thumbnails, and textures are cache/display resources, not the source of truth for document pixels. Avoid crossing the native bridge for every pixel or transferring a full-resolution image every frame.
- Expensive work returns through a typed asynchronous job/result interface; no PSD decode, full-size filter, or save operation runs on the UI thread.
- Don't clone or rebuild the authoritative document from a binding refresh. View models expose focused snapshots and commands.
- Phase 0 explicitly tests .NET 11/Avalonia compatibility, XAML compilation, focus and IME, high DPI, docking, pointer/stylus capture, screen-reader/accessibility names, theme behavior and UI responsiveness before framework approval.

### 42.1 Design goals

- **One workspace, many levels of expertise.** Beginners see understandable default controls; specialists can reach exact values and advanced panels without changing modes.
- **Canvas first.** UI chrome should support the artwork, not compete with it. Panel visibility and density are user-controlled.
- **Predictability.** A command invoked from a button, menu, shortcut, context menu, quick action, or command search must call the same domain operation and produce the same document/history behavior.
- **Inspectable state.** Selection, active layer, active mask, transform state, dirty state, color space, zoom, and asynchronous work are visible when relevant.
- **Reversibility.** Destructive actions require clear intent and are undoable when possible. If an operation cannot be reversed, the user is warned before it occurs.
- **No fake affordances.** A disabled feature is visibly disabled with a reason, or omitted from that release. Never ship an active-looking control that returns a canned success message.
- **System integration.** Respect Windows high-contrast settings, scaling, keyboard conventions, file pickers, clipboard, and reduced-motion preferences where applicable.

### 42.2 Main window regions

The initial desktop layout has these regions:

1. **Window and application header:** VEGA logo/wordmark, app menus, window controls, optional document switcher, and a compact cloud/account indicator. The title bar must not consume excessive vertical room.
2. **Menu bar:** File, Edit, Image, Layer, Select, Filter, View, Window, Help. Menus are organized by user intent, not by implementation module.
3. **Document tab strip:** each tab shows the document name, an unsaved marker, and a recoverable visual indication of save/sync state. Closing a dirty document always prompts to save, discard, or cancel.
4. **Tool rail:** persistent primary tools; flyouts group related tools (for example, several selection tools). The active tool is visually distinct and discoverable by keyboard.
5. **Contextual tool options:** settings for the selected tool, active layer/object, or current operation. It must not become the only location for settings that should persist in Properties.
6. **Canvas viewport:** the largest region. Rulers and scrollbars may be hidden by default but are reachable from View. Document boundaries, transparency, selected-object bounds, selection edges, guides, and transform handles are rendered as overlays, never destructively into source pixels unless the user explicitly requests that.
7. **Right-side docks:** default tabs are Properties, Layers, Adjustments, Color, History, and Assets. Panels can be resized, hidden, moved among supported dock zones, restored to defaults, and reset individually. Advanced panels can be added as they are implemented.
8. **Bottom status bar:** zoom, document pixel dimensions, active color/profile summary where relevant, job progress, and contextual tool hints. Do not show raw engine internals to ordinary users.
9. **Command search:** opens from a consistent shortcut and searches commands, tools, panels, preferences, actions, and help topics. It does not run arbitrary shell commands or plugin code by itself.
10. **Job center:** accessible from progress/status, listing long-running actions such as export, generation of previews, AI model download, cloud upload, and batch jobs, with progress/cancel/retry affordances when safe.

### 42.3 Layout responsiveness and scaling

- Support resizable, maximized, and restored windows; preserve a usable minimum window size.
- Support Windows scaling settings and mixed-DPI monitors without misaligned canvas handles or blurry UI assets.
- Store panel widths/heights in logical units, not raw physical pixels.
- If the available window is too narrow, collapse secondary panels into a tabbed dock or temporary flyout instead of crushing controls to unreadable widths.
- The canvas should use available space after essential chrome, not force fixed sizes on the user's display.
- Remember the last valid workspace layout, but if stored layout state is corrupted or references a missing panel, recover to the default layout without blocking startup.
- Provide a single **Reset Workspace Layout** command.

### 42.4 Visual tokens and theming

VEGA should use named design tokens rather than hard-coded colors in every component. Initial theme is dark neutral with a restrained violet/blue accent, but the visual system must not depend on color alone.

Token groups:

- **Surfaces:** window, panel, raised panel, canvas surround, dialog, menu, tooltip.
- **Text:** primary, secondary, muted, disabled, inverse, link.
- **Borders:** subtle, regular, focused, selected, invalid, warning.
- **Accent:** action, action-hover, selection, focus-ring.
- **Status:** success, warning, error, informational, progress.
- **Artwork overlays:** transform handles, guide lines, selection edge, grid, cursor outline.
- **Spacing:** compact, standard, relaxed spacing scale.
- **Radius:** small, standard, dialog/card.
- **Typography:** UI body, caption, panel header, dialog title, numeric field, monospaced diagnostic value.
- **Motion:** quick feedback and small panel transitions only; respect reduced-motion settings.

Requirements:

- Themes and contrast modes use centrally defined tokens.
- Never rely on hue alone to encode a dangerous state or selection.
- Numeric fields use aligned numerals where practical; units are displayed explicitly.
- Tool icons should share a consistent visual grammar and remain legible at multiple DPI scales.
- Avoid copying protected iconography, branded assets, distinctive proprietary illustrations, or exact product-specific design files from other editors.

### 42.5 States every reusable control must support

Every interactive component needs defined default, hover, pressed, keyboard-focus, selected, disabled, loading, invalid, and high-contrast states as relevant. Pointer hover must not be the only way to reveal important information. Focus indicators must remain visible against both panel and canvas surfaces.

### 42.6 New document and welcome experience

The startup surface is a launcher, not a second editing mode. It contains:

- **New document:** dimensions, units, resolution, orientation, background, color profile/bit depth when supported, and recent presets.
- **Open:** file picker, drag-and-drop, and optional recent files.
- **Recent projects:** thumbnail, name, modified date, location, and missing-file/recovery state. Recent history is local by default.
- **Templates:** local or community-discovered templates, clearly separated by source and trust status. A template opens as an editable copy; it must not silently overwrite the original.
- **Recovery:** clear display of recovered/autosaved documents after an abnormal shutdown.
- **Account/cloud access:** optional and never blocks Open, New, or local editing.
- **Learning help:** short workflow starters and tooltips that users can dismiss. No forced account creation.

No account sign-in or cloud content download is required to use the local editor.

### 42.7 Document tab, save state, and history

- Distinguish **Saved locally**, **Unsaved changes**, **Saving**, **Save failed**, **Cloud upload pending**, **Cloud save complete**, and **Conflict**. Local save and cloud sync are separate operations and must not be conflated.
- Cloud failure cannot mark the local document as unsaved if local saving succeeded; show the two states separately.
- Undo history is per document. Switching tabs does not mix histories.
- Document names are user-controlled, but internal stable document IDs are separate from file paths and names.
- If an external process modifies an open file, detect the change where possible and offer reload/compare/save-as rather than silently overwriting.
- Closing a document with an active non-cancellable operation must explain whether closing will wait, cancel safely, or keep a background task alive.
- History entries need readable labels (for example, “Brush stroke”, “Move layer”, “Exposure adjustment”), not low-level function names.

### 42.8 Canvas behavior

- Fit to screen, 100%, fit width, fit height, zoom around cursor, and smooth pan should behave consistently.
- Zoom and canvas navigation do not alter document pixels and should not pollute content undo history.
- The checkerboard represents transparency and is never baked into image pixels unless explicitly chosen as a background fill.
- Selection edges and guides should remain legible over light and dark artwork. Provide alternate display behavior for difficult backgrounds.
- Transform preview must match committed output within the documented resampling/color tolerance.
- Pan/zoom must stay responsive during ordinary editing; expensive render refreshes are scheduled and cancellable.
- The canvas may show a lower-quality preview while rendering a high-quality result in the background, but it must signal that the preview is provisional when visual differences could mislead.
- Pixel inspection/color sampling should report coordinate, color representation, and color profile/space context as supported.

### 42.9 Properties panel

Properties adapts to the active selection and tool but maintains a stable information architecture. Examples:

- **Raster layer:** position, dimensions, rotation, opacity, blend mode, lock, visibility, resampling choice.
- **Text layer:** content, font family, style, size, tracking, leading, alignment, color/fill, paragraph settings, transform.
- **Shape/path:** geometry, fill, stroke, line width, join/cap, alignment, transform.
- **Mask:** enabled state, density/opacity where supported, inversion, feather, mask-specific painting controls.
- **Adjustment layer:** adjustment-specific parameters, visibility, blending, clipping, and reset.
- **Brush:** size, hardness, opacity/flow, spacing, dynamics and pressure only when a supported input device is present.

Controls use explicit units and sensible defaults. Values should validate at the boundary and communicate range/precision limits. Changing a numeric value should create a well-defined history operation and should not re-render on every keystroke when the user is still entering a value; use live preview with debounce and a clear commit/cancel interaction where appropriate.

### 42.10 Layers panel behavior

Each row can expose, as appropriate:

- Visibility toggle with keyboard access and a tooltip.
- Layer thumbnail plus mask/linked-asset indicators.
- Name, selected state, lock/selection protection, and group nesting.
- Blend/opacity summary when useful without overcrowding the panel.
- Expand/collapse for groups and disclosure for supported layer effects.
- Context menu actions such as rename, duplicate, group, mask, convert, export, and delete when implemented.
- Clear indicators for adjustment, text, vector, linked, missing asset, and unsupported imported content.

Dragging layers must show a target insertion marker. Dragging onto a group must clearly show whether the layer will enter the group or be placed above it. Support keyboard alternatives to drag-and-drop for reorder and grouping. Delete, merge, rasterize, and flatten operations must explain when editability is lost; flatten and merge are undoable where history allows.

### 42.11 Color, adjustment, and tool controls

- Color selection supports a color well, numeric entry, eyedropper, and history/recent colors as those features are implemented.
- Color values must identify their representation (such as RGB/hex/HSV) and color profile where relevant.
- Sliders support keyboard adjustment and numeric entry; users must not be forced to drag to an exact value.
- Histograms and curves show the channel and scale, and expose descriptive accessible labels if graph interaction is supported.
- Presets are named, searchable, previewable, and resettable. Preset application creates editable operations where possible.
- The quick action “Improve lighting” should present and create the same adjustment layers and parameters available in the Adjustments panel; it cannot use a private hidden filter path.
- Expensive filters show progress when relevant and offer cancellation before commit. Cancellation must discard the provisional result and preserve the prior valid document state.

### 42.12 Command search and shortcut policy

- Search tools, commands, panels, preference pages, workflow actions, and help topics.
- Results show action names, short descriptions, categories, and shortcuts if assigned.
- Rank exact command names and recent successful actions above broad matches, but don't conceal alternate results.
- Search never uploads the current image or document contents.
- Commands require normal authorization/permission checks. A plugin may register a command through the defined extension API, but cannot inject arbitrary executable actions outside its sandbox.
- Shortcuts must be discoverable and documented. Conflicts are detected and resolved visibly. User customizations can be introduced after the initial shortcut map is stable.
- Essential functions remain accessible through menus, not shortcut-only.

### 42.13 Dialogs, notifications, and errors

- Use dialogs only for decisions that require interruption or confirmation; use inline status for routine success.
- Error messages state what failed, whether the document remains safe, and the next recovery action.
- Do not show raw exception stacks to regular users. Diagnostics can be copied or exported deliberately.
- Never put authentication tokens, local private paths beyond what is needed, document pixels, or secret keys in error messages or telemetry.
- Destructive dialogs name the impact (for example, “Flattening will combine layers in this copy”) and offer Cancel as the safe default.
- For long work, show progress only when meaningful; don't use fake progress or arbitrary timers.
- Toast notifications cannot be the only record of a failed save or upload; persistent state remains visible until resolved.

### 42.14 Accessibility and internationalization

- All menus, panels, dialogs, toolbars, tree rows, sliders, numeric fields, color controls, and document tabs must be keyboard reachable when interactive.
- Every icon-only action has an accessible name and tooltip.
- Use visible focus indicators, logical tab order, appropriate roles/states, and screen-reader names. Canvas operations that cannot be fully exposed through screen readers need equivalent keyboard workflows and accessible status feedback.
- Do not encode status with color alone; include icons/text/shape differences.
- Respect text scaling where technically possible without breaking canvas interactions.
- Use locale-aware number parsing and formatting in UI while preserving invariant numeric representation in project files.
- Store UI strings in resource catalogs from the start; avoid building the interface out of concatenated English fragments. English is first release language, but string externalization prepares for localization.
- Date/time displays use local formatting; serialized timestamps use UTC/invariant format.
- Provide reduced-motion behavior and avoid flashing effects.

### 42.15 Onboarding, discoverability, and help

- First-run guidance should be skippable and can be reopened from Help.
- Introduce the editor through a few real actions: open, select, adjust, undo, save/export. Don't block the user behind a long tutorial.
- Empty states explain the next useful action and avoid promotional filler.
- Tooltips explain purpose and shortcut; advanced help links to detailed reference pages.
- Command search acts as the universal discovery layer; documentation explains both basic and advanced paths.
- Help pages must match the actual shipped feature set and label beta/experimental features honestly.

### 42.16 UI instrumentation for testing

Use stable automation identifiers for major controls without coupling tests to display text or layout pixels. UI tests should cover launch, New/Open, file dialogs where testable, document tabs, dirty state, panel reset, command search, tool selection, export error handling, and cloud states. Keep screenshot regression tests for stable, deterministic panels and dialogs, but do not make every visual test depend on fragile whole-window pixel equality.

### 42.17 Default workspace map and menu contents

The initial UI should have a predictable default layout rather than presenting all possible panels at once.

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│ VEGA   File Edit Image Layer Select Filter View Window Help      Account ○  │
├──────────────────────────────────────────────────────────────────────────────┤
│ Document tabs:  portrait.vega •     poster.vega                              │
├───────┬───────────────────────────────────────────────┬─────────────────────┤
│ Tools │ Tool options / selection / transform settings │ Properties | Layers │
│       ├───────────────────────────────────────────────┤ Adjustments | Color │
│ Move  │                                               │ History | Assets     │
│ Select│                  CANVAS                       │                     │
│ Crop  │        image + guides + selection             │ selected layer      │
│ Brush │                                               │ thumbnails          │
│ Erase │                                               │ groups and masks    │
│ Text  │                                               │                     │
│ Shape │                                               │                     │
├───────┴───────────────────────────────────────────────┴─────────────────────┤
│ Zoom 66% | 2400 × 1600 px | Rendering… / idle | Job center | Save state      │
└──────────────────────────────────────────────────────────────────────────────┘
```

This diagram defines hierarchy, not exact pixels. Right-dock tabs can be collapsed or rearranged; the canvas takes remaining space. The account control remains discreet and must not advertise network status as a blocker to local work.

Menu responsibilities:

- **File:** New, Open, Open Recent, Save, Save As, Save a Copy, Import/Place, Export, Batch Export when implemented, Document Properties, Close, Exit.
- **Edit:** Undo, Redo, Cut, Copy, Paste, Paste in Place where supported, Duplicate, Delete, Preferences, keyboard shortcuts, and undo-history-related commands.
- **Image:** Image Size, Canvas Size, Rotate/Flip Canvas, Crop, Trim, color profile/bit-depth conversions where supported, metadata.
- **Layer:** New Layer, New Group, Duplicate, Rename, Delete, reorder, link, lock, mask, clipping, merge, flatten, rasterize/convert, layer export.
- **Select:** Select All, Deselect, Reselect, Invert, Feather, Modify, Transform Selection, save/load selection when implemented.
- **Filter:** filter categories and recent filters. Unavailable filters are not presented as active menu items.
- **View:** zoom controls, Fit, 100%, guides, rulers, grid, snapping, transparency checkerboard settings, color proofing, overlays, panel visibility.
- **Window:** open document list, panel list, workspace layouts, Reset Workspace.
- **Help:** searchable help, getting started, supported formats, compatibility report explanation, keyboard map, diagnostics export, privacy/about, security reporting, check updates.

Menu grouping may change following usability testing, but the meaning of each category should remain coherent.

### 42.18 Proposed first-release keyboard map

These are initial defaults to test with users; treat them as a versioned contract and avoid changing established shortcuts casually.

| Action | Proposed shortcut | Notes |
|---|---|---|
| New document | Ctrl+N | Opens validated New Document dialog |
| Open | Ctrl+O | Native file picker |
| Save | Ctrl+S | Saves current project; for a raster source, prompts for a `.vega` project if needed |
| Save As | Ctrl+Shift+S | Does not silently replace original input |
| Export | Ctrl+Shift+E | Opens export workflow |
| Close document | Ctrl+W | Prompts if unsaved |
| Undo / redo | Ctrl+Z / Ctrl+Y | Per-document content history |
| Cut / copy / paste | Ctrl+X / Ctrl+C / Ctrl+V | Must work for supported selected content |
| Select all / deselect | Ctrl+A / Ctrl+D | Context-sensitive only where clearly documented |
| Search commands | Ctrl+K | Search does not execute on typing; Enter invokes selected result |
| Fit canvas | Ctrl+0 | View only; not content history |
| 100% zoom | Ctrl+1 | View only |
| Temporary pan | Space + drag | Space alone must not trigger a tool command |
| Brush | B | When available in the active document context |
| Eraser | E | When available in the active document context |
| Crop | C | When available in the active document context |
| Text | T | When available in the active document context |
| Move | V | When available in the active document context |
| Help | F1 | Opens contextual help or help home |

The initial mapping is not a mandate to copy a competitor's exact shortcut set. It should prioritize conventional expectations and be adjustable later. Shortcut dispatch must respect text-entry focus: typing `B` inside a text object or numeric field must not switch tools.

### 42.19 First-release visible-control policy

The public beta UI contains only controls that are implemented end-to-end. For a feature planned but not shipped, choose one of three behaviors:

1. Do not show it yet.
2. Show it in a clearly labeled roadmap/help area, not as an editor control.
3. Show a disabled item only when discovery has a strong user benefit, with a precise reason such as “Available in a future release”; disabled controls must not look active.

Do not include a decorative “AI” button that produces fixed mock output, a plugin manager that cannot validate/uninstall packages, or a cloud save toggle that stores only local state. Feature flags control availability, not pretend implementation exists.

### 42.20 Design review criteria

For every major screen, review:

- Can a first-time user identify the primary action in under a few seconds without training?
- Are advanced controls available without changing studio/mode?
- Does the view distinguish selected object, active layer, active mask, and active tool?
- Can the user recover from the common wrong action with undo or a safe Cancel path?
- Is there a keyboard and accessibility path for the main task?
- Does the UI remain usable at lower supported window sizes and high DPI?
- Do loading, empty, offline, denied, quota-full, unsupported, error, success, and stale/conflict states exist?
- Does the screen avoid sending local content to cloud/AI without explicit action?
- Can UI automation identify controls without pixel-coordinate clicking?

---

## 43. End-to-end user journeys and acceptance behavior

### 43.1 First launch and local use

1. App opens without an account prompt blocking use.
2. Welcome surface offers New, Open, recent documents, recovery, and optional sign-in.
3. User opens a supported image; the app validates it and shows a render.
4. The user crops or adjusts, uses undo/redo, saves to a local project or exports a raster image.
5. If any step fails, the source file remains intact and the application gives a recoverable error.

### 43.2 Create, edit, and save a project

1. New document validates dimensions, resolution, color setup, and memory budget before allocating large buffers.
2. Commands mutate the document model, and render invalidation is limited to affected regions where practical.
3. Dirty state updates after document-changing commands; view navigation does not mark the document dirty.
4. Save writes to a temporary project, validates it, and atomically replaces the previous file where the OS/filesystem supports that workflow.
5. Save completion is not reported before the operation actually finishes.
6. Reopening the file reproduces the editable tree and expected composite.

### 43.3 Import PSD with unsupported content

1. Validate header, offsets, dimensions, compression, section sizes, and resource limits before expensive processing.
2. Display a progress indicator for large files.
3. Import compatible features and preserve any available composite preview.
4. Show a report of preserved, approximated, flattened, and unsupported components.
5. Keep original PSD untouched.
6. If conversion would lose critical structure, allow cancel or composite-only fallback.
7. Any parser crash/security finding is treated as a bug; test fixture is retained where legally permitted.

### 43.4 Export an image

1. User selects format, dimensions/scale, quality/compression, metadata behavior, transparency behavior, color profile behavior, and destination when relevant.
2. UI defaults to non-destructive output and doesn't change the open project's dimensions unexpectedly.
3. If output cannot preserve transparency, color profile, layers, animation, or metadata, explain it before export.
4. Write to a temp file and finalize safely.
5. Validate output by reopening it with a decoder or tests as appropriate; report success only after finalization.

### 43.5 Private cloud save

1. User explicitly signs in and selects Save to VEGA Cloud.
2. Show account identity, destination, remote quota use, and whether this save will replace or create a version.
3. Worker verifies Firebase ID token, UID, account state, object ownership, file metadata, and quota.
4. Quota is reserved safely before upload and reconciled with actual object size at completion.
5. Cloud save reports saved/failed/conflict separately from local save status.
6. On network failure, preserve local project and provide retry/save-as choices.

### 43.6 Community publishing

1. User selects Publish a Copy; private source project remains private and unchanged.
2. User chooses title, description, tags, preview, supported content type, attribution/license choice, and public vs unlisted visibility.
3. A preflight screen lists included assets and warns about embedded private metadata or linked files.
4. Uploaded data is quarantined until validation and processing finish.
5. Only after validation/finalization does the content become publicly retrievable.
6. Owner can update, unpublish, delete, and view reports/status. A moderation action may remove content independently of owner controls.

### 43.7 Plugin installation

1. User reviews publisher identity, version, supported VEGA API versions, permissions, package hash/signature state, review status, and release notes.
2. Plugin downloads into quarantine and its package/manifest is validated.
3. Installation requires consent to any capabilities beyond default sandbox access.
4. Plugin initializes under resource limits, cannot receive unrestricted file/network/process access, and is isolated from the editor UI process as far as the selected runtime permits.
5. Failure yields a visible error and rollback to the previous compatible version where possible.
6. User can disable/uninstall any plugin. A revoked or known-malicious plugin can be blocked by catalog policy; already downloaded packages are not silently overwritten.

### 43.8 Account stolen or device lost

1. A user can reach account recovery even without access to VEGA's desktop app.
2. After identity recovery, sessions are revoked through a trusted Firebase administrative path, and VEGA installation associations can be removed/revoked server-side.
3. Sensitive account changes require recent authentication where appropriate.
4. The account screen lists devices/installation associations and last-seen information only at a justified level of detail; no precise hidden tracking.
5. The product explains that server revocation blocks future cloud API access after enforcement but cannot erase local files or downloads already made.

---

## 73. Unified UI behavior for advanced capability without clutter

The interface must be designed around predictable command discoverability and low-friction access, not around permanently exposing every setting.

### 73.1 Global navigation

Top-level menus should be stable and keyboard accessible: File, Edit, Image, Layer, Select, Filter, View, Window, Help, with dedicated entries for plugins/automation if the menu architecture warrants them. The menus expose discoverable commands while search provides a faster alternate route. Commands should have consistent names across menus, search, tooltips, and automation IDs.

The left toolbar includes a configurable set of selection, navigation, crop/transform, paint/retouch, vector/text, gradient/fill, and measurement tools. Related tools may be grouped; do not hide essential functions behind unexplained long-press or flyout gestures alone. Every tool needs an accessible label and a discoverable shortcut or command-search entry.

The top tool-options bar changes according to the active tool. It must clearly display the target (pixels/mask/path/shape/selection), parameter values, units, and relevant state. Tool-options controls are keyboard navigable, and a tool change should not silently discard unapplied numeric input.

Dockable panels include Layers, Properties, Adjustments, History, Color/Swatches, Brushes, Channels/Paths, Navigator, Histogram, Info, Actions/Automation, Assets/Resources, Timeline where installed, and Diagnostics/Document Health. Panels can be reset to defaults. Panel visibility and layout preferences are per workspace layout; document state is stored separately.

### 73.2 Contextual properties policy

The Properties panel prioritizes the selected entity's common controls but does not remove access to advanced controls. A clear “More controls” expansion may reveal complex values, yet the complete supported operation remains searchable. Selection order and multi-selection must have defined behavior. For mixed selections, show only shared properties and explain when values differ.

### 73.3 Canvas interaction policy

- Clicking a layer on canvas follows a configurable auto-select behavior; modifier keys provide predictable override.
- Transform handles do not overlap the only visible resize control when a panel layout is narrow; minimum usable dimensions are defined.
- Escape cancels the current transient operation when safe; Enter/Return commits only when the active tool defines it.
- Zoom-to-cursor, spacebar-pan, fit-to-window, 100%, view rotation, guides, and snapping must work consistently across tools.
- Canvas interactions must not unexpectedly change focus to a hidden panel or steal text input.
- The cursor/overlay distinguishes active selection, mask editing, path editing, transform, and eyedropper sampling.
- Long operations run as cancellable tasks and do not freeze menus or window movement.

### 73.4 First-run experience

The first launch must not force account creation or AI setup. It presents an accessible welcome surface with New Document, Open Image/Project, recent local files if permitted, tutorial sample documents, and settings. An online account is available as a clearly optional action. Sample documents must be distributed with compatible asset/font licenses and should teach real workflows rather than just showcase visual effects.

### 73.5 Error and progress UX

Errors need to distinguish user-input problems, unsupported features, recoverable resource failures, internal faults, and service/network failures. A useful error includes what happened, what data is safe, what the user can try, and a copyable diagnostic reference. Never show raw stack traces as the primary user-facing message, but retain redacted diagnostics for developers. Progress UI should indicate stages and cancellation effects without inventing percentage completion when the operation cannot measure it.
