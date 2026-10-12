# VEGA Planning Set v2.1 — Section & Decision Manifest

This is a topic crosswalk from the v1.5 numbered section structure to the categorized planning files. v2.0 briefly proposed a Rust/egui UI; v2.1 supersedes that UI choice with C#/.NET 11 + Avalonia for desktop, Kotlin/Compose for Android, and a shared Rust editing core. Legacy section numbers are historical references rather than a guarantee that every old numbered heading remains verbatim. The authoritative current organization is `plan.md`.

## Product Vision & Decisions (`product-vision.md`)
- Section 1: Executive decision summary
- Section 2: Product definition and target users
- Section 6: Free and proprietary product policy

## UI & User Experience (`ui-ux.md`)
- Section 3: The single unified workspace
- Section 42: Detailed visual design system and UI behavior
- Section 43: End-to-end user journeys and acceptance behavior
- Section 73: Unified UI behavior for advanced capability without clutter

## Editing Capabilities & Feature Inventory (`editing-capabilities.md`)
- Section 4: Product feature map
- Section 58: Capability-parity charter: broad scope, independent implementation
- Section 59: Full capability coverage matrix
- Section 61: Layers, objects, channels, and compositing — expanded requirements
- Section 62: Selection, masks, and edge quality
- Section 63: Brushes, painting, and tablet workflow
- Section 64: Retouching, healing, and photo restoration
- Section 65: Photo development, RAW workflows, and computational photography
- Section 66: Adjustment, filters, color grading, and image effects
- Section 67: Typography, vector content, graphic layout, and multi-artboard design
- Section 69: Animation, timeline, and video capability plan
- Section 81: Additional current feature-reference set
- Section 82: Detailed tool, panel, and command-family inventory
- Section 83: Final expanded product quality principles

## AI, Automation & Recipes (`ai-automation.md`)
- Section 5: AI policy
- Section 46: Optional AI and automation design
- Section 68: Automation, macros, scripts, batch jobs, and APIs
- Section 71: Optional AI and intelligent assistance — expanded safeguards

## Software Architecture (`architecture.md`)
- Section 8: Technical architecture
- Section 9: Proposed repository layout
- Section 29: Expanded repository layout
- Section 44: Data, command, render, and history contracts
- Section 60: Document model and image-science requirements
- Section 76: Architecture changes required for full feature scope

## File Formats & Interoperability (`file-formats.md`)
- Section 7: File formats and data safety
- Section 70: File format and asset interoperability expansion
- Section 75: Interoperability and format-fidelity scorecards

## Backend, Accounts & Cloud Services (`backend.md`)
- Section 18: Online-service scope: basic backend only
- Section 19: Backend architecture and responsibilities
- Section 20: Cloud storage, quotas, and uploads
- Section 21: Device account limit: three accounts per device installation
- Section 22: Public sharing and community library
- Section 24: Initial Worker API contract
- Section 25: D1 data model and consistency rules
- Section 26: Backend threat model and operational requirements
- Section 27: Backend tests and acceptance criteria
- Section 28: Expanded implementation roadmap
- Section 31: Additional official backend references
- Section 47: Backend user experience and service contracts

## Plugins, Extensions & Creative Assets (`plugins-and-assets.md`)
- Section 23: Plugin system and plugin sharing
- Section 45: Asset, preset, font, and resource management
- Section 74: Plugin and extension model — feature coverage expansion

## Security, Privacy & Anti-Theft (`security-privacy.md`)
- Section 11: Security, privacy, and user trust
- Section 32: VEGA Security & Anti-Theft Architecture
- Section 33: Account Theft, Session Security, and Recovery
- Section 34: Protecting User Projects and Cloud Files
- Section 35: Application Integrity, Updates, and Anti-Tampering
- Section 36: Community Upload, Plugin, and Intellectual-Property Protection
- Section 37: Abuse Prevention, Rate Controls, and Service Anti-Theft
- Section 38: Security UX, Privacy, and Incident Response
- Section 39: Security Validation and Go-Live Checklist
- Section 40: Security Implementation Order
- Section 41: Security Reference Sources
- Section 48: Threat model and security engineering matrix
- Section 49: Privacy, data lifecycle, and operational policy
- Section 55: Consolidated security implementation checklist

## Testing, Performance & Engineering Quality (`engineering-quality.md`)
- Section 10: Testing and engineering quality
- Section 50: Performance budgets and benchmark protocol
- Section 51: Quality strategy and release gates
- Section 77: Performance and stability engineering for expanded scope
- Section 78: Expanded verification and acceptance program

## Build, Release & Development Roadmap (`build-release-roadmap.md`)
- Section 12: GitHub Actions and build/distribution
- Section 13: Release plan and exit criteria
- Section 14: Vibe-coding implementation contract
- Section 15: Initial implementation task
- Section 16: Open decisions to resolve during technical spikes
- Section 17: Official references
- Section 30: Final decision register and remaining validation items
- Section 52: Repository, CI/CD, and release operations refinement
- Section 53: Project planning and backlog rules
- Section 54: Consolidated launch checklist
- Section 56: Final decision register
- Section 57: Official references to re-check before implementation
- Section 79: Expanded phased parity roadmap
- Section 80: Program governance and prioritization rules

## VEGA Signature Innovations (`innovations.md`)
- Section 72: VEGA improvements that go beyond parity
- Section 84: VEGA's product identity: original capabilities, not cosmetic imitation
- Section 85: VEGA signature system 1 — Branchbook: real creative branches, not just undo history
- Section 86: VEGA signature system 2 — Edit Atlas: explain what changed, where it changed, and why
- Section 87: VEGA signature system 3 — Design Contracts: tell the document what must remain true
- Section 88: VEGA signature system 4 — Variant Matrix: one creative, many deliverables
- Section 89: VEGA signature system 5 — Change Lens: inspect the real impact of every edit
- Section 90: VEGA signature system 6 — Source Ledger and Rights Passport
- Section 91: VEGA signature system 7 — Recipe Studio: inspectable, testable, reusable editing workflows
- Section 92: VEGA signature system 8 — Output Lab: predict the delivery, not just export a file
- Section 93: VEGA signature system 9 — Semantic Canvas and universal creative search
- Section 94: VEGA signature system 10 — Learn by Replay and shareable project explanations
- Section 95: Innovation integration, data model, and phased delivery
- Section 96: Differentiation review and launch proof


## Architecture reset note

The original section IDs are references to topic scope, not a claim that every historical sentence was retained verbatim. The current desktop app stack is C#/.NET 11 + Avalonia, the shared editing core is Rust, Android uses Kotlin/Jetpack Compose, and the Cloudflare Worker remains TypeScript. See [`architecture.md`](architecture.md), [`dotnet-development.md`](dotnet-development.md), and ADR-0002/ADR-0003 for current decisions.

## New and revised v2.1 material (outside the original numbered sections)

- [`rust-development.md`](rust-development.md): rules for Rust core architecture, safe code, Cargo workflows, interop, and AI-assisted implementation.
- [`decisions/ADR-0001-rust-first-application.md`](decisions/ADR-0001-rust-first-application.md): historical v2.0 decision, superseded by ADR-0003.
- [`decisions/ADR-0003-csharp-avalonia-kotlin-ui.md`](decisions/ADR-0003-csharp-avalonia-kotlin-ui.md): current hybrid UI/core architecture decision.
- [`dotnet-development.md`](dotnet-development.md): C#/.NET 11 and Avalonia coding, testing, and native interop rules.
- [`decisions/ADR-0002-typescript-cloudflare-worker.md`](decisions/ADR-0002-typescript-cloudflare-worker.md): retains the previously requested TypeScript backend as a deliberate exception.

- [`android-development.md`](android-development.md): Kotlin/Compose, lifecycle, native bridge, touch UI, and Android test requirements for the deferred mobile phase.
