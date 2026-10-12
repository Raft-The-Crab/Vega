# AI, Automation & Recipes

> VEGA Planning Set v2.1 · Category document
> Optional AI policy, automations, scripts, batch work, and safeguards.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [5. AI policy](#5-ai-policy)
- [46. Optional AI and automation design](#46-optional-ai-and-automation-design)
- [68. Automation, macros, scripts, batch jobs, and APIs](#68-automation-macros-scripts-batch-jobs-and-apis)
- [71. Optional AI and intelligent assistance — expanded safeguards](#71-optional-ai-and-intelligent-assistance-expanded-safeguards)

---

## 5. AI policy

AI is an **optional enhancement**, not a foundation of the editor. Every ordinary edit must remain available without AI and without a network connection.

### Initial policy

- No required account, subscription, remote AI service, or network dependency.
- First AI candidates: subject/background selection assistance, masking refinement, and optional image cleanup, but only after non-AI editing is stable.
- Prefer optional local inference where license, model size, performance, and hardware requirements are acceptable.
- Do not bundle a large model by default without evaluating installer size, device compatibility, license terms, and update strategy. Optional model downloads must be visible and initiated by the user.
- Do not silently upload images, prompts, masks, metadata, or thumbnails.
- Every AI result must be inspectable, undoable, and preferably represented as a layer, mask, or editable operation.
- If a future online provider is considered, it requires a separate explicit product decision, a clear data flow/consent screen, cost sustainability plan, and privacy documentation. It must not make core editing dependent on the service.
- Keep AI integrations behind a provider interface so VEGA can ship without them and support optional backends later.

---

## 46. Optional AI and automation design

AI is not a requirement for basic functionality and is not part of the initial backend MVP.

### 46.1 AI principles

- Explicit opt-in per feature; no background upload of artwork.
- Explain whether a model runs locally or remotely before the operation begins.
- Prefer local processing when quality, license, size, latency, and hardware constraints make it viable.
- Never send local documents to a server merely because the user is signed in.
- If remote inference is introduced later, show the destination/provider, what data is transmitted, retention behavior, and cost/limits before submission.
- AI output is a layer/mask/asset or another reviewable result whenever technically possible, rather than a silent destructive replacement.
- Support cancellation, undo, retry, and failure without corrupting the source document.
- Model downloads are opt-in, versioned, license-checked, integrity-verified, and removable.

### 46.2 Non-AI automation first

Prioritize repeatable batch actions and presets before generative AI: batch resize/export, naming patterns, applying adjustments, watermarking when requested, preset application, and recorded action sequences with a defined schema. Any macro/action feature must be a constrained command list, not arbitrary downloaded code.

---

## 68. Automation, macros, scripts, batch jobs, and APIs

Automation is a first-class professional requirement and a major opportunity to make VEGA easier than a manually repetitive workflow.

### 68.1 Recordable actions

An action recorder should capture supported commands as a versioned list with parameter values and optional prompt points. Users can inspect, rename, reorder, group, enable/disable, and edit captured steps. Playback reports the failing step and offers continue/stop decisions. Commands with destructive consequences must be explicit in the recording and support validation before batch execution.

### 68.2 Conditional actions and batch queue

Plan for conditions based on dimensions, aspect ratio, color mode, file type, layer presence, metadata, and supported feature state. The batch queue supports source filters, output paths, naming templates, per-file logs, concurrency limits, retry, cancellation, skip/replace policies, and dry-run preview. Queue operations must not overwrite inputs unless the user explicitly selects that behavior.

### 68.3 Scripts and automation SDK

Expose a versioned automation API for opening/creating documents, inspecting metadata, selecting layers, applying supported commands, setting properties, exporting, and responding to documented events. It should be possible to write scripts in a supported language or use a built-in node/step workflow later, but the first scripting runtime must prioritize sandboxing and repeatability.

Scripts must not inherit arbitrary filesystem, network, process, credential, or plugin permissions by default. Require explicit capability grants and visible user consent for operations that reach beyond the open document. Separate trusted developer mode from normal user mode; don't let a script silently disable security checks or call raw native pointers.

### 68.4 API design rules

- Use stable command IDs, typed parameter schemas, validation errors, cancellation tokens, and structured logs.
- Version the API independently from the app release.
- Make command behavior consistent between UI invocation, action playback, scripts, and batch processing.
- Support dry-run and validation where side effects occur.
- Keep the UI responsive and expose progress for operations that can take measurable time.
- Never ship secrets, privileged backend credentials, or unrestricted cloud administrative capabilities in the desktop API.
- Publish sample scripts, a command reference, migration notes, and compatibility support windows.

---

## 71. Optional AI and intelligent assistance — expanded safeguards

AI functions are optional product capabilities. Users must be able to open, edit, save, export, and use core tools without account sign-in, AI model downloads, or a network connection.

### 71.1 Potential AI capabilities

- Selection assistance: subject/object/sky/people/body-part segmentation and edge refinement.
- Generative fill/remove/expand and background generation or replacement.
- Content-aware object removal and image extension.
- Upscaling, deblur, denoise, restoration, face-aware enhancement where lawful and appropriate, and super-resolution.
- Style/lighting/scene harmonization and shadow/reflection suggestion.
- Semantic retouch recommendations, composition/crop suggestions, and image quality warnings.
- Image search/tagging on-device, alt-text suggestions, and project/layer naming assistance.
- Variant generation as separate layers/documents, with compare and discard controls.

### 71.2 Model sourcing and runtime

Do not assume that a model is free to bundle, redistribute, modify, or use commercially just because weights are downloadable. Review the model license, dataset provenance claims, distribution size, hardware requirements, runtime dependencies, and applicable policy before inclusion. Model runtime should be an optional module with an explicit capabilities matrix. Unsupported hardware should result in a useful unavailable state rather than degraded core editing.

### 71.3 User control and privacy

Before any remote AI operation, state what data is sent, to which service, for what purpose, and whether the provider may retain/process it under their terms. Never upload the full document when the user requested a local basic adjustment. Use the minimal selected input/region where technically possible and clearly state when context requires more pixels. Give a preview/consent step and retain a local copy of results.

For local AI, expose downloads as optional components with size estimates and supported hardware. Provide a disable setting, model removal, and data processing disclosure. AI output must be an ordinary editable result (layer, mask, object, or raster result) with a record of the tool and model version where appropriate. Users can reject outputs without replacing the source.

### 71.4 Safety and rights guardrails

The AI feature set must have policies for consent-sensitive biometric/identity manipulation, deceptive editing, harmful content, copyrighted input/output questions, and upload restrictions. Provide a visible content-origin note where warranted and do not make a false claim that provenance metadata guarantees authenticity. Any content credential/watermark system must be assessed independently for legal, technical, and interoperability constraints.
