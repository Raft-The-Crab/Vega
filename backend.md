# Backend, Accounts & Cloud Services

> VEGA Planning Set v2.1 · Category document
> Firebase identity, Cloudflare Workers/D1/R2, quotas, sharing APIs, and backend validation.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [18. Online-service scope: basic backend only](#18-online-service-scope-basic-backend-only)
- [19. Backend architecture and responsibilities](#19-backend-architecture-and-responsibilities)
- [20. Cloud storage, quotas, and uploads](#20-cloud-storage-quotas-and-uploads)
- [21. Device account limit: three accounts per device installation](#21-device-account-limit-three-accounts-per-device-installation)
- [22. Public sharing and community library](#22-public-sharing-and-community-library)
- [24. Initial Worker API contract](#24-initial-worker-api-contract)
- [25. D1 data model and consistency rules](#25-d1-data-model-and-consistency-rules)
- [26. Backend threat model and operational requirements](#26-backend-threat-model-and-operational-requirements)
- [27. Backend tests and acceptance criteria](#27-backend-tests-and-acceptance-criteria)
- [28. Expanded implementation roadmap](#28-expanded-implementation-roadmap)
- [31. Additional official backend references](#31-additional-official-backend-references)
- [47. Backend user experience and service contracts](#47-backend-user-experience-and-service-contracts)

---

## 18. Online-service scope: basic backend only

VEGA will add an optional online service after the local editing foundation and native project save/open behavior are dependable. The online service is a companion to the desktop app, not a requirement for editing.

### In scope for the first backend release

- Create accounts and sign in with Firebase Authentication email/password authentication.
- Verify email, refresh/renew authentication through Firebase, request password reset, sign out, and delete an account through a documented flow.
- View basic account profile and private cloud-storage usage.
- Explicitly upload, list, download, rename, and delete private cloud project files, subject to a **250 MB per-account quota**.
- Publish files/resources to public or unlisted share links without those copies consuming the private cloud quota.
- Browse and search a basic public community library, open share details, download shared resources, and report abusive or infringing content.
- Upload, browse, version, install, disable, and remove supported VEGA plugins through an initial plugin catalog.
- Associate up to three distinct VEGA accounts with one registered application installation, subject to the device-identity limitation described below.
- Provide account, share, and plugin deletion/reporting workflows and operational logs that avoid unnecessary personal data.

### Explicitly out of scope

- Real-time collaboration, shared cursors, live co-editing, team workspaces, shared editable sessions, comments, direct messages, and presence indicators.
- Cloud rendering, cloud AI inference, mandatory cloud saves, mandatory logins, paid tiers, subscriptions, ads, or usage-based charges to VEGA users.
- A general-purpose file-hosting service for arbitrary executable programs.
- A complex recommendation algorithm, social network, follower system, or engagement feed.
- Syncing a currently open document in real time across devices. Users may manually save/upload and download/reopen files on another device; concurrent changes must never silently overwrite one another.

### Network behavior

- The app starts and performs normal local editing while offline and while signed out.
- Cloud save, community sharing, plugin discovery/download, and account management each have clear, separate user actions.
- Do not upload local files, thumbnails, recent-file paths, font lists, clipboard contents, or crash data automatically.
- Account screens disclose which item is private, public, or unlisted before publishing.
- Cloud errors must not discard local edits. A failed upload remains a retryable operation; a successful local save remains valid regardless of cloud service availability.

---

## 19. Backend architecture and responsibilities

### 19.1 Service layout

Use a TypeScript Cloudflare Worker as the single public API boundary for VEGA's first backend. Bind it to:

- **Cloudflare D1** for normalized metadata: app profile, device registrations, cloud file records and quota accounting, share listings, plugin manifests/versions, reports, and audit events.
- **Cloudflare R2** for bytes: private project files, public share objects and their generated previews, and plugin packages. Use separate buckets or equivalently strong environment/access separation for private files, public shares, and plugins; development and production must never share buckets accidentally.
- **Firebase Authentication** as the identity provider. Firebase is authoritative for credential verification and its UID is the user's stable identity. Do not store passwords in D1.
- **Cloudflare rate limiting / edge controls** for abusive request patterns and **Turnstile** where a browser-based challenge is practical (for example, account registration or suspicious upload activity). These are abuse controls, not authentication substitutes.

A small typed HTTP router and schema validation library may be used after license, size, and maintenance review. Avoid adding a queue, cache, Durable Object, vector database, or other service until a requirement actually needs it. D1 plus R2 plus Workers is sufficient for the initial capabilities.

### 19.2 Responsibilities by layer

- **Firebase Auth:** sign-up/sign-in, password reset, email verification, token and session lifecycle.
- **Worker API:** verify identity tokens; enforce authorization, visibility, device association, quota, file allowlists, rate limits, upload state transitions, and server-side validation; issue short-lived upload/download authorization where appropriate.
- **D1:** store small structured records and maintain indexes; never store full project bytes or plugin binaries as base64.
- **R2:** store binary objects. Buckets remain private at the storage-provider layer; the Worker mediates authorized reads, or uses short-lived scoped upload/download URLs as appropriate.
- **VEGA client:** initiate uploads and downloads, display quota and publish visibility, retain local copies, and never decide authoritative storage usage or permissions.

Never expose R2 access credentials, service-account secrets, signing secrets, or other server credentials in the desktop executable, checked-in configuration, or logs. A Firebase client API key is a client configuration value rather than a substitute for server authentication; secure API operations by verified ID tokens and server-side authorization.

### 19.3 Firebase identity verification

The client uses Firebase's supported Authentication REST flow or a platform-appropriate client integration. Once signed in, it sends the Firebase ID token to the Worker over HTTPS as a bearer token. The Worker validates the signature using Google's published signing keys and checks the expected Firebase project audience, issuer, expiration, issued-at time, and non-empty subject/UID. Cache keys according to the upstream cache headers and use a maintained JWT implementation compatible with Workers. Never trust a UID merely because the client submitted it in JSON, and never treat a decoded-but-unverified token as authentication.

Verify email for account capabilities that can publish public content or register plugins. Local editing and public read-only browsing may work without signing in. Reset and verification links must be handled without logging their tokens. Firebase refresh tokens remain in the client credential store and are never sent to or persisted by the VEGA API.

### 19.4 Data ownership and deletion

Firebase UID is the account key. The Worker stores only the profile fields required by VEGA (for example, display name, public handle if implemented, creation time, and preference flags), not copies of passwords or unnecessary identity data. Public share metadata must not reveal email addresses or private cloud filenames.

Provide an account deletion workflow that explains the consequences, requires recent authentication where appropriate, deletes private cloud objects and profile data, and offers the user a clear choice about published shares and plugin packages. Shared content that remains public must have an explicit retained owner/publisher state and moderation contact path; it cannot silently be orphaned. Define a short, documented deletion/tombstone period only where needed for reliable object deletion, abuse prevention, or legal requirements.

---

## 20. Cloud storage, quotas, and uploads

### 20.1 Private account storage: 250 MB

Product definition: **250 MB means 250,000,000 bytes per Firebase account** for private online storage. The UI may display a rounded usage figure but the server uses exact byte counts. This is a storage allowance, not a maximum total count of local projects. Local files are not counted.

Private account storage includes every object that VEGA retains for that account in the private cloud pool: uploaded `.vega` projects, associated private assets, retained revisions or recovery copies, and separate stored previews. The implementation should minimize duplicate storage and state exactly how retained revision bytes are accounted for. Publicly published copies live in a separate pool and do not count against the 250 MB private allowance. If the same content exists in both pools, the two stored copies are accounted for independently in the operator's infrastructure metrics.

Use a server-authoritative quota reservation and finalization protocol:

1. Authenticated client requests an upload session with purpose, intended length, content category, and checksum where available.
2. Worker checks the account, device association if applicable, allowable content type, current usage, outstanding reservations, and available quota.
3. Worker atomically reserves the declared bytes in D1 (including outstanding sessions) before granting upload authorization.
4. Client uploads directly to R2 using a short-lived, object-specific pre-signed URL or appropriate multipart session; avoid proxying large binary bodies through the Worker when a direct R2 upload is suitable.
5. Client calls completion. Worker verifies that the object exists, its actual size matches the reservation, and its checksum/metadata is consistent.
6. Worker commits actual usage and finalizes the file record. If validation fails, it marks the upload failed and queues/executes object cleanup.
7. Expired abandoned sessions release reservations and remove incomplete uploads; a scheduled cleanup task reconciles stale records and usage.

Do not trust a client-supplied size as proof of uploaded size. Make completion and deletion idempotent. A delete operation should update logical visibility and quota safely, and the UI must not show free quota until the deletion/accounting state is authoritative. Protect against parallel uploads racing past the 250 MB limit.

### 20.2 Public sharing: no per-user quota

The product policy is that users do not have a per-account total-storage allowance or upload-count cap for published public/unlisted shares. These uploads do not use private cloud quota. This is a deliberate business promise, with real infrastructure and abuse implications.

“Unlimited” does not eliminate unavoidable technical and safety boundaries:

- Cloudflare plan/request limits, object-size constraints, timeouts, and multipart constraints still apply; expose understandable errors and resumable behavior where supported.
- Validate each file's actual bytes/type, extension, declared purpose, length, checksum, and parser result. Do not trust MIME type or filename alone.
- Set per-request, burst, per-IP, per-installation, and automated-abuse controls. These protect the service from floods; they must not quietly become a published per-user storage quota.
- Keep public objects in a public-share bucket and never allow a public share URL to read private account keys by path manipulation.
- Apply content policy, report/takedown handling, owner deletion, moderation state, and a way to disable a malicious publisher.
- Monitor aggregate stored bytes, operations, failed uploads, downloads, and spend. Create provider billing alerts and an emergency service-protection playbook. Cloudflare R2 currently bills for stored data and operations even though egress charges are waived, so unrestricted use has a cost to VEGA's operator.
- Be transparent in the service terms that harmful, infringing, abusive, or illegal content can be removed. Keep no-per-user quota separate from the right to reject an unsafe file or throttle an attack.

Public read access can be anonymous. Publishing requires a verified account. A share record has an opaque ID, owner UID, title, description, type, tags, visibility, object key, byte count, checksum, content status, timestamps, download statistics at coarse granularity if needed, and version. Public responses never expose the bucket's internal key layout.

Visibility states:

- **Public:** discoverable through the community listing and downloadable through VEGA.
- **Unlisted:** not indexed/discoverable, but anyone who obtains the share link can access it. Do not describe this as private or confidential.
- **Private:** used only for account storage and owner-authorized access; it is not a community listing visibility state.

Publishing should create a separately managed shared object or immutable share version. Editing or deleting the private cloud copy must not silently change or destroy a published version without a clear confirmation.

### 20.3 Upload categories and safe file handling

The initial supported upload categories should be declared in code and documentation, for example:

- VEGA documents and templates (`.vega` plus any later explicit template extension).
- VEGA resource packages: brushes, gradients, palettes, presets, LUTs, and supported asset packs.
- Image assets for which VEGA has a tested decoder and safe preview path.
- VEGA plugins (`.vegaplug`) containing a manifest and sandboxed WebAssembly module under the plugin rules below.
- Other binary attachments may be offered as download-only files only if the operator accepts the moderation/security tradeoff. They must never execute or render active content in the app, Worker, or gallery preview.

Reject arbitrary executables, native libraries, installers, and scripts as plugins. Do not render untrusted SVG/HTML in an application WebView. Use safe generated thumbnails rather than serving potentially active user content inline. Where preview generation is needed, process files with size/time/resource bounds and fail closed if parsing is unsafe. Keep content as an attachment with a safe content disposition where appropriate.

Uploads do not run synchronously through a long parser inside an API request. If deeper scanning is added, introduce an explicit pending/rejected/approved state and a tested processing service; do not advertise content as safe before that step is complete.

---

## 21. Device account limit: three accounts per device installation

Product target: a VEGA installation can register up to **three distinct account UIDs**. This is a limit on accounts associated with the application installation, not a limit of three active HTTP sessions. The initial UX can keep up to three account profiles available for switching while one account is active at a time.

### Suggested implementation

- On first run, generate a cryptographic installation key pair and a random installation ID. Protect the private key using Windows DPAPI/Credential Manager; store only a public-key fingerprint/server-issued install ID in the local app profile.
- The Worker registers a public key for the installation and stores its registration in D1.
- Requests that mutate device-linked account state prove key possession with a short-lived server challenge or signed request where warranted. Do not accept a client-chosen `deviceId` string as authoritative proof.
- When a Firebase UID is first linked, use a transaction/conditional database operation to count distinct active account associations for that installation. Reject registration once three distinct UIDs are registered. Repeated logins to an already-linked UID do not consume another slot.
- Define how a genuinely deleted account, compromised installation, or lost profile is handled. Do not let ordinary sign-out reset the historical association and trivialize the cap.
- Rate-limit account/device registration; email verification and suspicious-activity challenges provide additional controls.

**Important technical limitation:** a normal Windows desktop application cannot prove physical hardware identity with absolute reliability merely by reading a machine ID. Reinstalling, deleting local state, running a VM, or spoofing identifiers can evade an installation-only limit. The initial release therefore enforces three accounts per registered installation and uses abuse signals to raise the cost of bypassing it; it must not claim a tamper-proof three-accounts-per-physical-device guarantee. Hardware-backed key storage/attestation may strengthen this on supported systems, while Android can later evaluate Play Integrity. If the strict physical-device cap is non-negotiable, validate a suitable attestation design before promising it publicly.

---

## 22. Public sharing and community library

The community library is a simple directory of user-published assets, not a social network or collaboration platform.

### Community capabilities

- Browse newest and recently updated public items.
- Search by title and tag; filter by supported resource type.
- View publisher's public display name, item type, compatibility range, version, file size, preview, license/permissions statement, and update date.
- Download/import a resource and show an import/compatibility confirmation before adding it to a document.
- Copy a public/unlisted share link, update published metadata, publish a new version, unpublish, or delete one's own item.
- Report malware, malicious plugins, copyright concerns, impersonation, spam, or other policy violations.
- Maintain moderation states such as `pending`, `available`, `hidden`, `rejected`, and `removed`; only available items appear in browse/search.

No likes, comments, private messages, followers, teams, invitations, shared live documents, or collaborative editing are required for the first backend. Anonymous users may browse and download available public content; an authenticated verified account is required to publish and to manage one's own content.

Every upload should ask the publisher to confirm they have the rights needed to share it. The license field is informational and user-provided; VEGA must not imply that a user-uploaded asset is legally cleared simply because it is hosted on the service. Add a takedown/contact process and a repeat-abuser response policy before public launch.

---

## 24. Initial Worker API contract

Use a versioned JSON API under `/v1`. Exact request/response schemas should be committed as TypeScript types plus runtime validation before client integration. Avoid putting account identity or secrets in query strings.

| Method and route | Access | Purpose |
|---|---|---|
| `GET /v1/health` | Public | Liveness/basic service health; no secret configuration dump |
| `GET /v1/me` | Authenticated | Return minimal VEGA profile and private storage usage |
| `DELETE /v1/me` | Recent auth | Begin confirmed account/data deletion workflow |
| `POST /v1/devices/register` | Authenticated | Register/prove installation and enforce account association limit |
| `GET /v1/cloud/files` | Authenticated | Page through private cloud files |
| `POST /v1/cloud/uploads` | Authenticated | Reserve quota and issue a private upload session |
| `POST /v1/cloud/uploads/{id}/complete` | Authenticated | Validate R2 object and finalize quota/accounting |
| `GET /v1/cloud/files/{id}/download` | Authenticated owner | Issue authorized download or stream object |
| `PATCH /v1/cloud/files/{id}` | Authenticated owner | Rename or update metadata using optimistic concurrency |
| `DELETE /v1/cloud/files/{id}` | Authenticated owner | Delete/private-tombstone and reconcile usage |
| `POST /v1/shares/uploads` | Verified user | Initiate public/unlisted share upload without using private quota |
| `POST /v1/shares/uploads/{id}/complete` | Verified owner | Validate upload and make share available or pending review |
| `GET /v1/shares/{shareId}` | Public if available | Return safe share metadata |
| `GET /v1/discover/shares` | Public | Search/page public shares with validated filters and bounded page size |
| `GET /v1/shares/{shareId}/download` | Public if available | Return safe download redirect/stream with visibility check |
| `PATCH /v1/shares/{shareId}` | Owner | Update metadata, visibility, or publish a version |
| `DELETE /v1/shares/{shareId}` | Owner/moderation | Unpublish/delete the share and schedule object cleanup |
| `POST /v1/shares/{shareId}/reports` | Public/auth policy | Report potentially harmful or infringing content |
| `POST /v1/plugins/uploads` | Verified publisher | Begin package upload and reserve technical upload resources |
| `POST /v1/plugins/uploads/{id}/complete` | Verified publisher | Validate package/manifest and publish pending catalog version |
| `GET /v1/plugins` | Public | Browse/search available plugins |
| `GET /v1/plugins/{pluginId}` | Public | Plugin metadata and available versions |
| `GET /v1/plugins/{pluginId}/versions/{version}/download` | Public if available | Download a validated available version |
| `POST /v1/plugins/{pluginId}/reports` | Public/auth policy | Report a plugin/security concern |
| `DELETE /v1/plugins/{pluginId}/versions/{version}` | Owner/moderation | Withdraw an owned version or remove a moderated version |

Apply bounded pagination, input limits, consistent error codes, request IDs, idempotency keys for upload completion/deletion, and explicit authorization checks to every route. Never accept arbitrary R2 bucket names or object keys from the client. The server resolves an opaque database ID to its stored object key after checking ownership/visibility.

### Standard error contract

Return an appropriate HTTP status and a small stable JSON error such as `{ "error": { "code": "QUOTA_EXCEEDED", "message": "Private cloud storage is full.", "requestId": "..." } }`. Error codes should include at least `AUTH_REQUIRED`, `EMAIL_VERIFICATION_REQUIRED`, `FORBIDDEN`, `NOT_FOUND`, `QUOTA_EXCEEDED`, `UPLOAD_EXPIRED`, `INVALID_FILE`, `UNSUPPORTED_FORMAT`, `RATE_LIMITED`, `VERSION_CONFLICT`, and `SERVICE_UNAVAILABLE`. Do not return SQL, stack traces, bucket keys, signing details, token contents, or secret environment values.

---

## 25. D1 data model and consistency rules

The exact D1 schema is created through version-controlled migrations. Proposed logical tables:

- `user_profiles`: Firebase UID primary key, safe public handle/display name, creation/updated timestamps, account status, minimal preference fields.
- `device_installations`: server-issued installation ID, public-key thumbprint, platform/app version, registered time, last-seen time, status. Do not store raw OS hardware IDs as an attempted security substitute.
- `device_accounts`: installation ID, Firebase UID, linked/removed status, timestamps; unique constraint on installation + UID; enforce at most three qualifying unique UIDs through a race-safe transaction/conditional write pattern.
- `cloud_files`: opaque file ID, owner UID, safe display name, logical storage category, R2 object key, byte size, checksum, content type from validation, created/updated time, object state, version/ETag.
- `cloud_quota`: owner UID, committed byte usage, reserved bytes, version/update time. Treat as a cache/accounting row backed by reconciliation, not an unverified client claim.
- `upload_sessions`: session ID, owner UID, purpose/pool, reserved size, expected checksum, fixed object key, expiration, status, multipart metadata, timestamps.
- `shares`: share ID, owner UID, object key, title, description, tags, type, visibility, moderation state, size/checksum, preview key, published version, timestamps, compatibility and license metadata.
- `share_versions`: immutable version number, share ID, object key, manifest/checksum, created time, release notes, availability state.
- `plugins`: stable plugin ID, owner UID, metadata, moderation status, current version and timestamps.
- `plugin_versions`: plugin ID, version, package object key, manifest JSON validated against schema, checksum, API compatibility, permission declarations, file size, publication status.
- `content_reports`: report ID, target type/ID/version, reason code, user narrative with length limits, reporter reference if signed in, status, timestamps, moderator action.
- `audit_events`: limited security-relevant events such as account linking, publish/unpublish, quota inconsistencies, moderation, and deletion. Do not log access tokens or full document names by default.

Use indexes that match owner listing, public search/filtering, published status, IDs, and cleanup tasks. Use parameterized statements for every query. Use database uniqueness and conditional state transitions to prevent double-complete, double-delete, quota races, and duplicate plugin versions. D1 and R2 are not one transactional database, so represent multi-step operations with durable states (`pending`, `complete`, `failed`, `deleting`) and a reconciliation worker rather than pretending cross-service atomicity exists.

---

## 26. Backend threat model and operational requirements

Threats to handle before public access include account token theft, IDOR (access to another user's file by guessing IDs), quota race attacks, malicious/corrupt project packages, path/key manipulation, MIME spoofing, oversized/decompression-bomb files, malicious plugin behavior, spam/abuse uploads, copyright abuse, denial-of-service, leaked pre-signed URLs, fraudulent publishers, compromised release artifacts, and accidental exposure of private assets. Section 32 onwards defines the full security and anti-theft system; the following controls are the minimum baseline for the backend.

Required controls:

- HTTPS only and short-lived, narrowly scoped upload URLs with fixed object keys and bounded methods/headers.
- Verify Firebase tokens server-side and re-check authorization for each resource. UUIDs/opaque IDs reduce guessing but are not authorization.
- Server-generated R2 object keys; strict separation of private, public, and plugin buckets/prefixes and separate dev/prod credentials.
- Rate limits and payload caps; public share quota remains unmetered per user, but requests are not unbounded per second and attacks can be throttled.
- Validate file signatures, actual object size, archive entry counts, decompressed total size, path traversal, nested archive depth, manifest schema, and supported parsers.
- No evaluation of user-supplied JS or native code. Plugins run only in the approved sandbox and permissions model.
- CORS is not authentication; native desktop callers are not protected by browser origin checks. Every protected route verifies its token and authorization regardless of `Origin`.
- Avoid storing email, IP, or user agent longer than needed. If IP-derived abuse signals are retained, document purpose and retention; do not expose them publicly.
- Secrets are stored as platform secrets, rotated if exposed, and kept out of local sample configs. Test environments use separate Firebase project, D1 database, and R2 buckets.
- Backups/migrations, service error monitoring, R2/D1 usage metrics, and a documented restore procedure exist before general availability.
- Content reporting, takedown handling, repeat-abuser controls, and an emergency disable/pause-upload mechanism exist before opening public uploads to everyone.
- Add a privacy policy and service terms covering public uploads, moderation, deletion, retention, account abuse, and the fact that unlisted links are not confidential.

---

## 27. Backend tests and acceptance criteria

### Authentication and accounts

- Missing, malformed, expired, wrong-project, wrongly signed, and wrong-issuer tokens are rejected.
- Client-supplied UID or owner IDs cannot access another user's objects.
- Unverified accounts cannot publish or upload plugins if verification is required by policy.
- Password reset and email verification flows do not leak tokens through VEGA logs.
- Three distinct accounts can be associated with one installation; the fourth is rejected; relogin for an existing account is allowed; parallel registration cannot exceed the cap.
- Documentation and UI do not claim the Windows installation association is impossible to reset or spoof.

### Private cloud storage

- Account with room can upload, list, download, rename, and delete a file.
- Uploads crossing the 250,000,000-byte allowance are rejected before authorization; parallel outstanding sessions cannot overbook quota.
- Actual R2 object size, not just client metadata, decides final quota use.
- Failed, expired, duplicate-completed, and abandoned uploads reconcile safely.
- Private object downloads require owner authorization; another user cannot access them even with a known opaque file ID.
- A failed cloud save never changes or removes the local project.
- Conflicting remote versions return `VERSION_CONFLICT` and allow downloading or saving as a new copy; never silently last-write-wins.

### Public sharing and plugins

- Public/unlisted uploads do not subtract from private quota, and public shares do not become visible before successful validation/finalization.
- Owner can publish, update, unpublish, delete, and create versions; moderation can hide or remove available content.
- Public share IDs cannot access private bucket records. Unlisted links are not displayed in public search.
- Disallowed or malformed file types are rejected. Active content is not rendered inline.
- Plugin package schema and compatibility are validated, permissions appear before installation, plugins do not receive undeclared capabilities, and disabling/uninstalling a plugin prevents later calls.
- Plugin exceptions/timeouts/resource exhaustion do not crash or corrupt the main editor document.
- A report can be submitted and moves the item into an operator-visible moderation queue.

### Operations

- Tests run against isolated local/test resources and cannot mutate production buckets/database.
- Migrations work from an empty database and from the prior released schema.
- Usage reconciliation detects deliberate quota-record drift.
- Upload/download, error rates, object counts, stored bytes, and estimated provider costs are observable without recording content or credentials.
- CI verifies TypeScript types, lint/format, unit/integration tests, migration validation, and dependency/license checks.
- Error responses are consistent and contain no secrets or internal stack traces.

---

## 28. Expanded implementation roadmap

The Windows local editor still comes first. The backend can be built as a separate TypeScript project without coupling the native rendering engine to network code.

### Backend phase A — contract and local emulator/tests

Write the OpenAPI or equivalent route contract, token/identity boundary, D1 migrations, R2 key strategy, quota definition, device association policy, content policy, and test fixtures. Add a local development configuration with a separate Firebase test project and local/test D1/R2 resources. No public uploads yet.

**Exit:** schemas and authorization rules are reviewable; no production secrets or resources are needed for local tests.

### Backend phase B — Firebase Auth and profile

Implement email/password sign-up/sign-in client flow, email verification, password reset, logout, `/v1/me`, server-side Firebase ID-token verification, basic profile, and account deletion plan. Core editing remains signed-out capable.

**Exit:** invalid tokens fail; authenticated test users can reach only their own profile; secrets are absent from the client.

### Backend phase C — private cloud storage

Implement quota reservation/finalization, direct or multipart R2 upload, listing, download, rename, delete, usage display, conflict detection, abandoned-upload cleanup, and reconciliation. UI calls this only on explicit user actions.

**Exit:** quota/race/ownership tests pass, including the exact 250,000,000-byte limit and failed upload recovery.

### Backend phase D — device account cap

Implement protected installation-key registration and the maximum-three-account policy, with diagnostics and a recovery path. Test concurrent links and installation reset limitations.

**Exit:** enforceable installation-level behavior is documented accurately; no unsupported claim of tamper-proof physical-device identity.

### Backend phase E — public shares

Implement upload sessions, public/unlisted items, listing/search, previews, downloads, publisher actions, reports/moderation status, safe file policy, and provider cost alerts. Keep no per-user public storage quota while maintaining technical safety boundaries.

**Exit:** public data never leaks private object metadata; publish/report/remove flows work; public read and upload abuse controls are enabled.

### Backend phase F — plugin packages and sandbox

Define the manifest and built-in extension boundaries; ship declarative presets/resource packs first. Implement WASM sandbox and permission system before executable public plugins, followed by the catalog, package upload, version pinning, install/disable/rollback, and reports.

**Exit:** permission and isolation tests pass; plugins cannot access arbitrary files/network/processes; package upload and catalog removal work.

### Backend phase G — private/public beta

Run security review, migration rehearsal, deletion and restore tests, cost simulations, abuse drills, legal/content policy review, and a limited invite-only upload pilot. Expand publication only after review.

**Exit:** documented operational owner, alerting, incident response, quotas, recovery, and takedown workflows are ready. Do not open unlimited public uploads to an unmoderated audience merely because the happy-path demo works.

### Platform integration after the backend

Keep API DTOs and endpoint semantics platform-neutral. Windows launches first; macOS reuses the Rust client service layer. Android should reuse the Rust core and call the same HTTPS API; a thin Kotlin bridge is permitted only for platform lifecycle/OS integration if the Android Rust UI prototype requires it. The image/document core must not know Firebase, D1, or R2 implementation details.

---

## 31. Additional official backend references

Recheck these official references immediately before implementation because platform limits and APIs can change:

- Cloudflare Workers TypeScript: https://developers.cloudflare.com/workers/languages/typescript/
- Cloudflare R2 Workers API/bindings: https://developers.cloudflare.com/r2/api/workers/workers-api-reference/
- Cloudflare R2 upload methods: https://developers.cloudflare.com/r2/objects/upload-objects/
- Cloudflare R2 pricing: https://developers.cloudflare.com/r2/pricing/
- Cloudflare D1 getting started: https://developers.cloudflare.com/d1/get-started/
- Cloudflare Workers limits: https://developers.cloudflare.com/workers/platform/limits/
- Cloudflare Turnstile server-side validation: https://developers.cloudflare.com/turnstile/get-started/
- Firebase Authentication REST API: https://firebase.google.com/docs/reference/rest/auth
- Firebase ID token verification: https://firebase.google.com/docs/auth/admin/verify-id-tokens
- Firebase App Check custom providers (possible future desktop hardening): https://firebase.google.com/docs/app-check/custom-provider

---

---

## 47. Backend user experience and service contracts

### 47.1 Account center

The Account panel should display email/verification state, sign-in method, private storage used/available, registered installations, active sessions if supported, and actions for sign out, revoke an installation, account recovery, export personal data, and delete account. Local editing must remain available while signed out. Do not display device geolocation or invasive hardware diagnostics.

### 47.2 Storage display and quota semantics

- Define the private quota as **250,000,000 bytes**, shown as approximately 250 MB using decimal units to avoid ambiguity. If a binary UI unit is displayed, disclose that it is MiB and show exact bytes in details.
- Quota applies to private online project objects and any stored private file versions/retained previews identified by policy. Avoid charging the same byte twice unless there are separately stored copies that actually consume storage; document the accounting rule.
- Local files, transient upload parts that are cleaned up promptly, public shares, and public plugin/resource packages do not consume the private quota.
- A replace operation must account for old/new versions and retention policy clearly. No user should unexpectedly exceed quota because hidden versions are retained without disclosure.
- The server uses actual object length/checksum and an authoritative usage ledger. The UI's meter is informative, never authoritative.
- When quota is exceeded, preserve local saves and offer delete older cloud copies, save as a new local file, or cancel.

### 47.3 Public sharing policy details

“No per-user limit” means no per-account cumulative byte quota and no per-account item-count cap imposed as a normal product tier. It does not mean arbitrary unbounded single-object uploads, unlimited request frequency, bypass of provider/runtime limits, or permission to publish illegal/unsafe content. The API still enforces documented per-object technical limits, file-type rules, anti-abuse controls, rate limits, and emergency service protection. When a limit affects a user, explain the reason and provide a retry or support path where practical.

### 47.4 Public, unlisted, and private visibility

- **Private:** owner-only; never discoverable publicly.
- **Unlisted:** accessible by high-entropy link/opaque ID, omitted from search and profile listings; explicitly explain that anyone with the link can view/download it unless authentication is required by product policy.
- **Public:** may appear in search, categories, public profile, and recommendations.
- **Removed/quarantined:** inaccessible to the general public until approved/restored; owner sees the state and appeal/report path when appropriate.

Unlisted is not a cryptographic access-control replacement for sensitive files. Private content must use authenticated owner authorization, regardless of how obscure an ID is.

### 47.5 API consistency and client resilience

- Version API routes under `/v1` and keep request/response schemas in shared contracts.
- Use stable error codes; distinguish authentication failure, authorization failure, validation errors, quota exceeded, stale version, rate limited, unavailable service, and retryable storage failure.
- Mutating requests use idempotency keys where duplicate execution could charge quota, publish twice, or delete unintended data.
- Use optimistic concurrency/version fields for private project updates; never silently overwrite newer remote data.
- Use short-lived upload/download authorization and bind it to the expected object, method, and expiration.
- Client clock and client-provided owner ID are not trusted for permissions or enforcement.
- Keep upload initiation, upload finalization, publication, and moderation state separate so unvalidated content is never public by default.

### 47.6 Authentication token validation in Cloudflare Workers

Firebase ID tokens are short-lived bearer credentials. The Worker should validate the JWT signature and the expected Firebase project issuer, audience, expiry, issued-at time, subject, and allowed algorithm using a vetted implementation and cached public key rotation rules. Do not assume an available Firebase Admin SDK works in the Workers runtime; verify runtime compatibility or use a reviewed JWT verification implementation. Use server-side Firebase administrative credentials only in server secrets with least privilege; never ship them in the desktop client. The API must have a deliberate strategy for disabled/deleted accounts and token revocation, including how quickly revoked sessions stop being accepted.

Official reference: https://firebase.google.com/docs/auth/admin/verify-id-tokens and https://firebase.google.com/docs/auth/admin/manage-sessions
