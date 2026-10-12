# ADR-0002: Keep the Cloudflare Worker backend in TypeScript

- **Status:** Accepted
- **Date:** 2026-10-12

## Context

VEGA's product owner previously specified a basic backend on Cloudflare Workers using TypeScript, Cloudflare R2 for object storage, D1 for metadata, and Firebase Authentication. The client stack is changing to Rust; that does not require the separately deployed backend to change language.

## Decision

- Keep the Worker written in TypeScript and managed as a separate deployment.
- Verify Firebase ID tokens server-side, and enforce all private-file access, storage quotas, account policy, sharing permissions, moderation, and plugin publication rules on the backend.
- Keep the API contract language-neutral (OpenAPI or versioned JSON schemas) so Rust clients and any future platform use the same semantics.
- Generate Worker binding types from the selected Wrangler config and use separate dev/staging/production resources and credentials.
- Do not embed Firebase service credentials, Cloudflare secrets, or administrative keys in the desktop client.

## Consequences

The project intentionally has two principal first-party language ecosystems: Rust for the application and image editor; TypeScript for the online service. This is a bounded separation between two independently deployed systems, not a cross-language boundary inside every local editing operation.

## References

- Cloudflare Workers TypeScript: https://developers.cloudflare.com/workers/languages/typescript/
- Cloudflare Workers Rust support (not selected for the initial backend): https://developers.cloudflare.com/workers/languages/rust/
