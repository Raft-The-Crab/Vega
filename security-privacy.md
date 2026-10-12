# Security, Privacy & Anti-Theft

> VEGA Planning Set v2.1 · Category document
> Threat models, identity and session security, private files, app integrity, plugin/upload safety, and incident response.

[← Back to `plan.md`](plan.md) · [Architecture](architecture.md) · [UI/UX](ui-ux.md) · [Capabilities](editing-capabilities.md) · [Backend](backend.md) · [Security](security-privacy.md)

## Contents

- [11. Security, privacy, and user trust](#11-security-privacy-and-user-trust)
- [32. VEGA Security & Anti-Theft Architecture](#32-vega-security-anti-theft-architecture)
- [33. Account Theft, Session Security, and Recovery](#33-account-theft-session-security-and-recovery)
- [34. Protecting User Projects and Cloud Files](#34-protecting-user-projects-and-cloud-files)
- [35. Application Integrity, Updates, and Anti-Tampering](#35-application-integrity-updates-and-anti-tampering)
- [36. Community Upload, Plugin, and Intellectual-Property Protection](#36-community-upload-plugin-and-intellectual-property-protection)
- [37. Abuse Prevention, Rate Controls, and Service Anti-Theft](#37-abuse-prevention-rate-controls-and-service-anti-theft)
- [38. Security UX, Privacy, and Incident Response](#38-security-ux-privacy-and-incident-response)
- [39. Security Validation and Go-Live Checklist](#39-security-validation-and-go-live-checklist)
- [40. Security Implementation Order](#40-security-implementation-order)
- [41. Security Reference Sources](#41-security-reference-sources)
- [48. Threat model and security engineering matrix](#48-threat-model-and-security-engineering-matrix)
- [49. Privacy, data lifecycle, and operational policy](#49-privacy-data-lifecycle-and-operational-policy)
- [55. Consolidated security implementation checklist](#55-consolidated-security-implementation-checklist)

---

## 11. Security, privacy, and user trust

- Opening any image/project file is untrusted-input handling. Validate lengths, offsets, dimensions, compression, recursion depth, and decompressed data limits.
- Never execute code embedded in project files or presets.
- Keep update signatures and release credentials out of source code and untrusted pull-request workflows.
- Store settings and logs locally with data minimization. Do not put document content or sensitive file paths into diagnostics by default.
- No telemetry by default in the initial release. If diagnostics are ever added, obtain clear opt-in consent and make it possible to see what is sent.
- Do not silently use the network for AI, crash uploads, analytics, font downloads, or cloud sync. Account, cloud, sharing, and plugin requests are user-initiated and separately disclosed.
- Online operations must verify Firebase ID tokens server-side; a client-supplied user ID is never proof of identity.
- Private project bytes must not be served from a public bucket or a guessable raw object key.
- Online quota is enforced server-side with reservations/finalization and reconciliation to handle concurrent uploads.
- Public uploads must have a content-report/takedown path, safe preview/download handling, and aggregate cost/abuse monitoring before open registration.
- Never execute native DLLs, .NET assemblies, or arbitrary scripts from community plugins; plugin execution requires a sandbox and explicit capabilities.
- Store Windows refresh tokens and installation private keys using Windows-protected storage; never place secrets in plaintext configuration files or logs.
- Ship signed release artifacts and a signed update manifest; reject tampered, unsigned, wrong-channel, or unauthorized downgrade packages.
- Provide recent-session visibility, session/device revocation, account recovery, and a lost-device workflow before public cloud launch.
- Consider client-side encryption for private cloud projects as a security milestone; distinguish this from provider-managed encryption at rest and document recovery/key-loss consequences.
- Local AI model weights and fonts require their own license/source attribution review.
- Provide a security contact/process before public distribution and maintain a response plan for serious parser vulnerabilities.
- Anti-theft controls must be proportionate and transparent: no keylogging, stealth persistence, undisclosed file scanning, covert hardware fingerprinting, or interference with security software.

---

## 32. VEGA Security & Anti-Theft Architecture

### 32.1 Goal and scope

VEGA must protect five distinct things:

1. **User accounts and sessions** from credential theft, stolen refresh tokens, unauthorized device access, and account recovery abuse.
2. **User artwork and project files** from accidental exposure, unauthorized cloud access, local credential theft, unsafe parsing, and preventable loss.
3. **VEGA's identity and software distribution** from counterfeit installers, tampered updates, and supply-chain compromise.
4. **The plugin and community ecosystem** from malicious packages, impersonation, spam, infringing uploads, and unsafe file previews.
5. **VEGA's service and operator resources** from abuse, quota bypass, cost attacks, denial of service, and unauthorized administration.

Security is layered: client protections reduce risk but do not replace server authorization; opaque IDs do not replace access control; encryption at rest does not equal end-to-end encryption; code signing establishes publisher/package integrity but does not prove the code is bug-free; and no normal desktop app can guarantee that local files will never be copied by a person who can read them.

### 32.2 Threat model and trust boundaries

Treat all of the following as untrusted until validated: image/document files, PSD/PSB containers, archives, fonts, LUTs, presets, plugin packages, API parameters, client-reported file size, client-supplied account IDs, local cache contents, downloaded previews, and public metadata.

Trust boundaries:

- UI actions must enter through validated Rust operation APIs. Core and renderer modules independently validate dimensions, indices, offsets, parameters, and allocation sizes; safe Rust does not replace input validation.
- The desktop client is not authoritative for cloud ownership, quota, public visibility, device-account associations, plugin approval state, or user permissions.
- The TypeScript Worker is the public API authorization boundary. It verifies identity and resource ownership on every protected request.
- D1 stores metadata and state; R2 stores bytes. Their operations are not one cross-service transaction. Use explicit state machines, idempotency, and reconciliation.
- Public resources are hostile inputs even if marked available. Validation and sandboxing reduce risk but do not prove a plugin is harmless.
- CI pull requests and build logs are not trusted with release secrets. Production signing/deployment must be isolated from untrusted contribution workflows.

Before substantial implementation, maintain a lightweight threat register with: asset, actor/capability, attack path, impact, mitigation, residual risk, owner, and verification test. Prioritize by severity and exploitability rather than claiming every risk is eliminated.

### 32.3 Security levels and release gates

**P0 — required before any cloud account beta:** token verification, per-resource authorization, secret separation, secure local token storage, private/public object separation, safe upload pipeline, deletion and account recovery behavior, rate limiting, and security audit events.

**P1 — required before public uploads/plugins:** malware/format handling policy, moderation and takedown, publisher impersonation controls, upload abuse controls, signed release/update design, plugin permission/sandbox implementation, security contact, cost alerts, incident response, and a rollback/pause mechanism.

**P2 — high-priority security improvements:** optional client-side encrypted private vault, hardware-backed installation key when supported, stronger account verification/MFA where supported, signed plugin publishers, reproducible build improvements, and external security review as the service grows.

Do not make release claims such as “unhackable,” “impossible to steal,” or “military-grade.” Document what controls are present and their limitations.

---

## 33. Account Theft, Session Security, and Recovery

### 33.1 Credentials and token handling

- Firebase Authentication owns passwords and credential verification. VEGA never receives or stores a user's password in D1, R2, logs, telemetry, or crash reports.
- Use Firebase ID tokens for short-lived API authentication. The Worker validates signature, issuer, audience, expiry, issued-at time, and subject using a maintained verifier and trusted signing-key cache.
- Refresh tokens are more sensitive than ID tokens. On Windows, store them through Windows Credential Manager or a reviewed DPAPI-backed storage implementation scoped to the current user. Do not write them to plaintext JSON, registry values, command-line arguments, crash dumps, or logs. For macOS and Android, use each platform's secure credential storage when those clients are implemented.
- Do not place Firebase service-account credentials, Cloudflare API tokens, R2 secret keys, signing keys, or moderation credentials in the desktop application. Public Firebase client configuration is not a substitute for server authorization.
- Clear secrets from temporary buffers when practical and avoid unnecessary token copies. Recognize that code running under the same compromised OS user may still attack a desktop process; client storage is risk reduction, not a protection against a fully compromised device.
- Use HTTPS for every online request. Never put bearer tokens, password-reset codes, signed URLs, or other secrets in application logs or URLs used for general navigation.

### 33.2 Session and device management

Provide an Account Security screen with:

- Current session/device installation and a last-used timestamp.
- Registered VEGA installations with a descriptive user-supplied label where possible; avoid exposing detailed hardware identifiers.
- “Sign out this device,” “Remove this installation,” and “Sign out all sessions” flows where supported.
- A confirmation and recent-authentication requirement for sensitive operations such as email changes, account deletion, and destructive account linking.
- A clear explanation that revoking a cloud session cannot erase a file already downloaded to a previously authorized or compromised device.

Maintain a VEGA-side session/device state in D1 for service authorization and revocation. Every authenticated call checks that the account is active and that the device association/session has not been revoked. For actions with higher risk, use a revocation-aware Firebase validation path or reauthentication. Because Firebase ID tokens remain valid until expiry unless revocation is checked, a stolen token must not be assumed invalid merely because the client pressed “sign out.” Design a secure administrative pathway to revoke Firebase refresh tokens using the supported Firebase Admin SDK from a trusted environment; never put admin credentials in the Worker client bundle.

Do not bind accounts rigidly to a single IP address. IPs can change legitimately. Use IP/network signals only as one risk input with minimal retention, and provide recovery when a legitimate user trips a risk control.

### 33.3 Account recovery and stolen-account response

- Account recovery uses Firebase's supported password-reset and email verification flows.
- A user who reports account theft gets a documented path: reset password, revoke Firebase refresh sessions through the trusted administration workflow, revoke VEGA device/session records, review published items and recent account-security events, and contact support if ownership or publication was changed.
- When possible, send security notifications for password/email changes, new device association, suspicious publishing, and account deletion requests. Do not include secrets or full project names in email notifications.
- Rate-limit sign-in, password reset, verification resend, account creation, device registration, and sensitive account operations. Avoid revealing whether an email address is registered in unauthenticated responses.
- Add multifactor authentication only through a supported, maintainable identity configuration. Do not invent a custom MFA cryptographic protocol. If not supported in the first release, state that clearly and prioritize strong password practices, session review, and account recovery.
- Support reports of compromised accounts and preserve only the security evidence necessary to investigate, with documented retention.

### 33.4 Three-account-per-installation policy

- Keep the existing limit of three unique Firebase UIDs associated with a registered VEGA installation.
- Use an installation ID plus a public key whose private key is protected with Windows-protected storage; where compatible hardware is available, offer TPM-backed key storage. Microsoft documents the Platform Crypto Provider for TPM-backed keys, but not every machine will expose identical attestation behavior.
- Use server challenge/signature verification for actions that link or remove account associations. Database writes must be race-safe.
- Do not collect raw serial numbers, MAC addresses, disk IDs, advertising IDs, or broad hardware fingerprints as a shortcut. Do not create a hidden, permanent hardware fingerprint.
- The installation identity is an abuse-control signal, not proof of a unique physical PC. Reinstalling, cloning a virtual machine, restoring profile data, or tampering may bypass it. Offer a recovery process for legitimate reinstallation and avoid locking a user out of local editing.
- If a stronger physical-device guarantee becomes a hard product requirement, run a separate platform-attestation feasibility review; do not block normal editing on attestation support.

---

## 34. Protecting User Projects and Cloud Files

### 34.1 Private-by-default rules

- Local projects remain local unless the user explicitly chooses Cloud Save, Publish, Share, or Upload Plugin.
- A new cloud file is private by default. Publishing is a separate action with a clear visibility choice and a pre-publication confirmation.
- Public and unlisted share objects use separate metadata and storage access paths from private cloud objects. No API accepts a raw R2 key from the client as authorization.
- Private download URLs, where used, are short-lived bearer credentials with narrowly scoped method/object access. Prefer the shortest practical expiry (proposed initial target: 5 minutes), never log the URL, and disclose that anyone obtaining it during its validity window may use it.
- An opaque file ID is not a secret and never grants access by itself. Check owner UID, object state, active account status, and permission for each operation.
- Use server-generated object keys, strict object-key validation, safe Content-Disposition headers, private cache controls for sensitive responses, and no public CDN route to private objects.
- Avoid exposing private cloud filenames, local paths, email addresses, raw bucket keys, user IDs, or detailed account storage inventories in public responses.

### 34.2 Encryption policy

R2 documents encryption at rest and TLS-protected transfers. Those are useful baseline controls, but provider-managed at-rest encryption does not mean VEGA operators can never access plaintext while processing a file. The product must use precise language.

Recommended plan:

1. **Backend MVP:** TLS in transit, provider encryption at rest, private bucket/object permissions, strict Worker authorization, limited logs, safe deletion, and no public exposure of private project content.
2. **Private Vault milestone before broad adoption:** design client-side encryption for private cloud project bytes, so the Worker/storage service receives encrypted content and cannot decrypt the user's project. Use a maintained cryptographic library and an established streaming AEAD construction; do not design a custom cipher or improvise chunk cryptography.
3. **Key recovery:** clearly explain the recovery trade-off. If VEGA never has the encryption key, losing the user's passphrase/recovery key may make cloud files irrecoverable. Offer a user-held recovery key generated and displayed once, with clear confirmation that VEGA cannot reconstruct it. Any server-held recovery escrow is a separate opt-in architecture with different trust claims.
4. **Public sharing:** publication necessarily makes a shareable representation available to recipients. Warn the user before copying a formerly private item into the public pool. Do not claim a published plaintext file remains confidential or can be recalled from people who already downloaded it.
5. **Previews and metadata:** generated previews can leak project content. For encrypted private files, generate previews locally and either store them encrypted or let the client decide whether to upload a separate preview. Encrypt sensitive names/metadata where the format permits; at minimum, use opaque keys and minimize metadata.

Do not ship custom client-side encryption until the format, chunk authentication, nonce uniqueness, key derivation, migration, corruption handling, recovery flow, and test vectors receive expert review. If it is not included in a given release, the UI and privacy documentation must say so plainly rather than imply end-to-end encryption.

### 34.3 Local project protection and recovery

- Autosave/recovery files are stored in a per-user application data directory with restrictive file permissions where supported.
- Offer an optional “encrypt this project with a password” capability only after using a reviewed format and secure key derivation scheme. Do not invent encryption formats just to add a checkbox.
- A user who enables project encryption must be warned that VEGA cannot recover a forgotten password unless they created a recovery key.
- Never silently delete original files, overwrite local work after a cloud conflict, or replace a project with an older copy.
- “Delete from cloud” removes the server's logical reference and starts object cleanup; the UI must explain if immutable backups, provider retention, or recovery windows mean physical deletion is not instantaneous.
- Add project file checksums and integrity checks for corruption detection. A checksum is not encryption and does not authenticate the publisher unless verified against a trusted signature.
- Recovery testing must include interrupted writes, full disk, corrupt autosave, failed download, device loss, and restoration on a second user-authorized device.

---

## 35. Application Integrity, Updates, and Anti-Tampering

### 35.1 Authentic distribution

- Pick and document the initial channel (for example, Microsoft Store/MSIX or a signed direct download) before release. Compare cost, auto-update, signing, support burden, and install UX.
- Sign production Windows artifacts and installer/package when the chosen distribution channel requires or supports it. Verify signatures in release tests and publish checksums and release notes. Microsoft documents the trade-offs among Store, MSIX, and direct-download distribution paths.
- Keep signing credentials outside source control and never provide them to untrusted pull requests. Prefer a protected key vault or managed signing service with tightly restricted CI identity; use separate development and production credentials.
- Produce a Software Bill of Materials (SBOM), third-party notices, source/version pinning, and dependency/license/security scans for every public release.
- Establish a stable update manifest that identifies app version, platform, architecture, package hash, signing identity, release channel, minimum supported version, and a signature over the manifest. Verify manifest signature and package signature/hash before installing an update.
- Reject modified, mismatched, expired/unauthorized, or wrong-channel updates. Do not auto-install arbitrary binaries downloaded from a plugin link or from community storage.
- Prevent downgrade to a known vulnerable version except through a signed, documented rollback procedure. Preserve user projects during upgrades/downgrades and test migrations before shipping.

### 35.2 Anti-tamper policy

VEGA may validate its own release signature, packaged resource hashes, and update channel when starting or updating. A failed integrity check should display a clear warning, preserve user documents, and direct users to an authentic installer/recovery path.

Do not implement invasive anti-tamper behavior such as keylogging, covert persistence, hiding processes, deleting user files, disabling Windows Defender, blocking task manager/debuggers globally, or silently uploading environment details. The product is free, so aggressive anti-piracy DRM is unnecessary and would undermine reliability. Code signing and update validation aim to prevent counterfeit/trojanized VEGA distributions, not to make reverse engineering impossible.

### 35.3 Build and supply-chain safeguards

- Pin SDKs, packages, action versions, and native toolchains where practical; update intentionally.
- Use least-privilege GitHub Actions permissions and explicit workflow permissions. Untrusted pull-request jobs have no release or production secrets.
- Protect release branches and tags, require CI success, and ensure release artifacts are built from a reviewed commit.
- Use dependency advisories/SBOM scanning and a documented patch process. A scan is not proof of absence of vulnerabilities.
- Keep development, staging, and production Firebase projects, D1 databases, R2 buckets, and secrets isolated.
- Audit administrative access and rotate credentials after personnel or incident changes.
- Store signing keys and API credentials only in platform secrets/key vaults. Do not write secrets into artifacts, build cache, logs, generated source files, or symbols distributed publicly.

---

## 36. Community Upload, Plugin, and Intellectual-Property Protection

### 36.1 Upload pipeline

Use an explicit upload state machine, such as `initiated → uploaded → validating → available` or `rejected/withdrawn`. Public metadata must not show an item as available until validation and finalization complete.

- Allowlist file/resource categories and supported extensions, but never trust the extension or browser/client MIME alone.
- Check actual byte length, signatures/magic bytes, container structure, archive entry count, total decompressed size, path traversal, symlinks where applicable, nesting depth, dimensions, parser time, memory, and decompression ratios.
- Use separate processing limits for images, `.vega` containers, resource packs, and `.vegaplug` plugin packages. Reject unsupported files safely.
- Generate previews using a controlled decoder or sandboxed processing context. Do not render uploaded HTML or active SVG as trusted UI content.
- Scan packages with available safety tools where practical, but do not represent a clean malware scan as a guarantee that a plugin is benign. The primary plugin protection remains sandbox isolation and capability limits.
- Sanitize display names, tags, descriptions, markdown, filenames, and links. Do not insert user data directly into HTML, SQL, shell commands, logs, or filesystem paths.
- Enforce platform/provider request limits and per-object technical size bounds. These are separate from the product promise of no per-user cumulative storage quota or item-count cap for public shares.

### 36.2 Plugin identity and signing

- Give each plugin a stable identifier and immutable version records. Publisher display names are not unique identities.
- Preserve who uploaded/published each version and when; allow withdrawal and moderation without mutating the byte content of an already published immutable version.
- A signed plugin release should bind the canonical manifest, plugin ID, version, API compatibility, permission declaration, and content hash. Signature verification establishes origin/integrity only if the signer key is trusted; it does not prove safety.
- Clearly distinguish built-in VEGA plugins, verified publisher identity (if introduced), community-unverified packages, and removed/revoked packages. Do not use “verified safe” for a signature check alone.
- Deny network, filesystem, OS process, environment, and arbitrary document export access by default. Users approve sensitive capabilities, and the host enforces them even if the plugin lies in its manifest.
- Plugin output is treated as untrusted. Validate all document commands, lengths, object IDs, returned buffers, and resource usage. Run plugin operations in a way that prevents crashes or malformed output from corrupting the document.
- Provide a global plugin kill switch and a remote catalog takedown/revocation path. On startup, VEGA should refresh revocation metadata when online, but offline users need clear behavior and must not be tricked into thinking stale revocation data is current.
- Plugin uninstall must remove its package and prevent further execution. Document what happens to existing documents that contain objects created by a plugin; do not silently destroy the objects.

### 36.3 Stolen artwork, ownership, and redistribution limits

- Private is the default. Public or unlisted publishing requires explicit user action and a preview of the exact visibility and metadata.
- Let publishers state license/usage terms and report ownership disputes, but do not treat a self-declared license as proof of ownership.
- Keep an abuse/copyright reporting path, case IDs, moderation outcomes, appeal/reinstatement process, and records of material actions.
- Support unpublishing and deletion, but explain that unlisted/public URLs and copies already downloaded by third parties may persist beyond VEGA's control.
- Optional watermarking of gallery previews can deter casual reuse but must not be presented as DRM and must not permanently watermark the user's original project.
- Do not expose original high-resolution private files when only a preview is required. Separate preview and download authorization where useful.
- Do not collect biometric, device contact, clipboard, keystroke, or unrelated file data to enforce ownership. Copyright and abuse controls must be transparent and proportionate.

---

## 37. Abuse Prevention, Rate Controls, and Service Anti-Theft

The no-per-user-total-quota policy for public uploads remains a product requirement, but a public service cannot promise unlimited instantaneous resources. Keep distinct concepts separate:

- **No account-level total-storage quota:** no user is assigned a 250 MB-style cumulative limit for public shares.
- **Technical upload boundaries:** each single upload has a declared format/size ceiling, request rates are limited, and unsafe formats are rejected.
- **Global service protection:** the operator may pause new uploads or temporarily limit abusive routes during an incident or cost attack. Existing lawful shares should remain readable when safe and financially possible.

Controls:

- Route-specific rate limits for sign-up, reset, token refresh, device registration, upload initiation, upload completion, publishing, downloads, search, and reports.
- Account, installation, network, and global limits as overlapping signals; avoid relying on a single IP or fingerprint.
- Idempotency keys for upload creation, finalization, deletion, and publish/update requests.
- Quota reservations for the private 250,000,000-byte storage pool, actual-object-size verification, reconciliation, and cleanup of abandoned multipart uploads.
- Per-object limits, checksum verification, short-lived signed URLs, and protection against replay/reuse outside the intended key/method scope.
- Budget alerts for Workers, D1, R2 storage and operations, and any moderation/scan provider. Define a global emergency pause threshold before opening uploads. No user-facing “unlimited” wording should imply unlimited file size, throughput, or guaranteed service capacity.
- Suspicious behavior scoring with explainable reasons and appeal/recovery paths. Avoid silently suspending legitimate accounts based only on IP changes or a single automated signal.
- Turnstile or similar browser challenge where practical for registration or suspicious browser-based activity; it is not an authentication system and may not fit the native desktop flow directly.
- Admin endpoints require a separate high-trust identity, MFA where available, least privilege, audit trail, and no shared default credentials. Administrative actions must not be exposed through ordinary user API routes.

---

## 38. Security UX, Privacy, and Incident Response

### 38.1 User-facing security controls

Add an Account & Security section containing:

- Email verification and account recovery status.
- Registered device/installation list and last-used information.
- Revoke this device and sign out all VEGA sessions.
- Public shares and plugins published by the account, including status and quick unpublish/delete actions.
- Recently relevant security events, such as new installation linking or email/password change, without exposing sensitive token data.
- Download personal account data and request account deletion.
- Optional private-vault encryption status and recovery-key status, once that feature exists.

Confirmation dialogs must describe the actual consequences. Deleting a published resource should state whether it removes the catalog listing, object, previews, and/or cached copies; never promise that an external copy can be erased.

### 38.2 Privacy defaults

- No ad tracking, keylogging, hidden device fingerprinting, or data sale.
- No telemetry by default for the desktop editor. Essential security logs are narrowly scoped, disclosed, access-controlled, and retained only as long as a documented purpose requires.
- Proposed default retention for routine security event metadata: 30 days, subject to privacy/legal review and incident needs. The final retention schedule must say what fields are kept, who can access them, and how deletion works. Never store access tokens, passwords, reset codes, signed URLs, image pixels, or full project contents in routine logs.
- Minimize IP retention. If IPs or derived network signals are temporarily used for abuse prevention, state the purpose, restrict access, and define deletion/aggregation periods. Do not publish them in community profiles.
- Online actions are visible and user-triggered. The client clearly indicates whether a file is local, uploaded privately, or published.
- Publish a privacy notice, community/content rules, plugin warning policy, and contact path before public beta.

### 38.3 Incident response

Before public cloud launch, write a small incident runbook with named owner/contact placeholders and procedures for:

1. Leaked API or signing credential: disable/rotate credential, assess exposure, invalidate affected release path, rebuild/re-sign if needed, and document the incident.
2. Account/session theft: suspend VEGA session access, revoke relevant Firebase refresh sessions through trusted administration, assist recovery, and review public content changes.
3. Private data exposure: close the exposure path, preserve minimum evidence, determine affected objects/users, notify affected users as legally/contractually required, and prevent recurrence.
4. Malicious plugin: revoke catalog availability/signature trust, block new installs, display a warning, preserve a safe uninstall/disable path, and determine whether affected projects or files require recovery.
5. Malformed-file exploit: disable or gate the affected parser path, publish a patched release and advisory, and add a regression fixture after safely sanitizing the reproduction.
6. Cost/abuse attack: pause new uploads or specific routes, protect private downloads and account access where possible, communicate service limits, and resume after controls are tested.
7. Data corruption/deletion: stop destructive jobs, restore from tested recovery material where available, reconcile D1 and R2, and report known data loss honestly.

Maintain a security contact and vulnerability-reporting instructions. A vulnerability report is not automatically a bug bounty; do not promise compensation until a program is explicitly funded and defined. Acknowledge reports, triage severity, coordinate disclosure, and avoid punishing good-faith testing that follows a published policy.

---

## 39. Security Validation and Go-Live Checklist

A public cloud beta must not launch until the high-priority tests below are automated or have documented manual evidence.

### Identity and authorization

- Reject missing, malformed, expired, wrong-project, incorrectly signed, and otherwise invalid Firebase ID tokens.
- Verify that user A cannot list, rename, download, delete, publish, or access user B's private object by changing IDs or request fields.
- Verify revoked/disabled VEGA sessions cannot mutate protected resources, and test the trusted Firebase token-revocation procedure.
- Verify passwords, tokens, service secrets, signed URLs, and reset codes never appear in logs or crash artifacts.
- Test password reset, email verification, account deletion, reauthentication, and lost-device recovery without account-enumeration leaks.
- Attempt a fourth distinct account association from the same installation; test simultaneous link requests, legitimate reinstall recovery, and documented bypass limitations.

### Storage, upload, and privacy

- Test quota values at 249,999,999; 250,000,000; and 250,000,001 bytes, including multiple parallel pending uploads and actual-size mismatch.
- Verify private object keys cannot be fetched via public routes, share IDs cannot address private objects, and guessed identifiers alone confer no access.
- Test expired/replayed upload URLs, wrong method/key, checksum mismatch, abandoned multipart upload, repeated completion, and delete/reconcile races.
- Test malformed files, fake MIME types, path traversal, duplicate names, oversized images, decompression bombs, nested containers, invalid dimensions, and parser timeouts.
- Verify local project data is not uploaded during startup, autosave, AI-disabled use, crash, or normal editing.
- Verify the user can distinguish private, public, and unlisted states, and that unpublishing does not imply deletion of external copies.

### Application integrity and plugins

- Modify a release package or update manifest and confirm signature/hash validation blocks installation.
- Confirm untrusted pull-request workflows cannot access release keys or production credentials.
- Verify the app recovers safely from an interrupted update and prevents downgrade to a blocked vulnerable version outside the documented rollback process.
- Attempt plugin file/network/process access without permission; verify it is denied at the host boundary.
- Test malicious manifests, oversized outputs, timeouts, memory exhaustion, crash loops, signature failure, revoked versions, and rollback.
- Verify users can disable/uninstall a plugin and still open documents containing plugin-created objects without silent data loss.

### Operations and response

- Exercise upload pause/kill switch, plugin takedown, credential rotation, session revocation, backup restoration, and D1/R2 reconciliation in staging.
- Confirm alerts exist for unusual request rates, failed authorization spikes, quota drift, storage/operation growth, and unexpected spending.
- Confirm staging cannot access production data; migrations and deploys require reviewed changes and appropriate permissions.
- Verify security, privacy, and community policy documentation is available before collecting public uploads.

**Go-live rule:** a clean demo is not enough. If a required security gate is not implemented, either postpone the exposed feature (especially public plugins/uploads) or narrow the release scope and disclose the limitation. Do not waive a protection silently to hit a date.

---

## 40. Security Implementation Order

1. **Local app baseline:** safe parsers, native boundary validation, dependency review, protected local settings, and no unnecessary telemetry.
2. **Authentic Windows distribution:** signing/release plan, update manifest verification, secure CI, SBOM and dependency checks.
3. **Private backend beta:** Firebase token validation, D1 ownership checks, private R2 isolation, quotas, session revocation, error/log scrubbing.
4. **Lost-device and recovery workflows:** device/session listing, revoke/sign-out actions, password-reset guidance, account deletion and data cleanup.
5. **Private Vault design and security review:** choose established streaming encryption and key recovery semantics before enabling client-side encryption.
6. **Public sharing beta:** strict upload validation, visibility controls, cost guards, report/takedown workflows, abuse queue and emergency pause.
7. **Plugin trust model:** manifest validation, sandbox runtime, capability enforcement, resource budgets, publisher identity/signatures, revocation and rollback.
8. **Security review before wider release:** threat-model refresh, test suite, restore drill, abuse simulation, privacy/legal review, and external review proportional to usage and risk.

Never build all security features in one enormous vibe-coded change. Each stage should have threat scenarios, testable acceptance criteria, and a reviewed implementation contract.

---

## 41. Security Reference Sources

These primary references should be revisited during implementation; platform details can change:

- Cloudflare R2 data security (encryption at rest/in transit): https://developers.cloudflare.com/r2/reference/data-security/
- Cloudflare R2 upload methods and presigned URLs: https://developers.cloudflare.com/r2/objects/upload-objects/
- Cloudflare Workers Rate Limiting API: https://developers.cloudflare.com/workers/runtime-apis/bindings/rate-limit/
- Cloudflare Workers current limits: https://developers.cloudflare.com/workers/platform/limits/
- Cloudflare D1 transactions/batch behavior: https://developers.cloudflare.com/d1/worker-api/d1-database/
- Firebase Authentication session management and refresh-token revocation: https://firebase.google.com/docs/auth/admin/manage-sessions
- Firebase App Check custom provider (possible anti-abuse enhancement; not a replacement for authentication/authorization): https://firebase.google.com/docs/app-check/custom-provider
- Windows CNG key storage providers, including the TPM-backed Platform Crypto Provider: https://learn.microsoft.com/en-us/windows/win32/seccertenroll/cng-key-storage-providers
- Windows DPAPI reference for Rust platform wrappers: https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata
- Windows app packaging, signing, and distribution choices: https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/choose-distribution-path
- OWASP File Upload Cheat Sheet: https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html

---

**Specification version:** 1.4 — expanded capability and product-design master specification (security, anti-theft, recovery, and software integrity expansion)
**Status:** Product direction and backend scope are defined. Security decisions requiring prototypes or legal/operational review are marked as validation items. The next step is technical spikes and threat-model review, not starting all features in parallel. Local editing must remain buildable while the TypeScript backend is developed as a separate subsystem.

---

## 48. Threat model and security engineering matrix

Security work must map to specific assets, attackers, controls, and tests. “Anti-theft” is not a single switch.

| Asset | Example threat | Core control | Required test/evidence |
|---|---|---|---|
| User account | Stolen password or refresh token | Firebase recovery, session revocation, safe token storage | Revoked/expired sessions lose cloud access within declared enforcement window |
| Private project | ID guessing or object-key leakage | Server owner authorization, private bucket, non-cacheable responses | Cross-account access tests cover list/read/write/rename/delete |
| Local project | Accidental overwrite or corruption | Atomic save, backups, journaling/recovery | Forced termination and disk-full tests preserve prior valid file |
| Cloud quota | Concurrent uploads race past limit | Reservation ledger, finalize/reconcile, actual-size check | Parallel boundary tests remain within quota |
| Public catalog | Malicious file, CSAM, copyright abuse, parser crash | Quarantine, policy, reports, bounded decoding, takedown | Invalid uploads cannot become public; operations runbook tested |
| Plugin runtime | Infinite loop or unauthorized file access | WASM sandbox, capabilities, memory/fuel/time caps | Adversarial suite can't escape or damage host document |
| Application binary | Fake installer or compromised update | Signing, hashes, signed metadata, protected release pipeline | Modified package/manifest fails verification |
| CI secrets | Pull request steals production key | Environment protection, scoped OIDC/secrets, no secrets on forks | Untrusted PR can neither deploy production nor access signing secret |
| User privacy | Logs contain tokens or artwork names | Structured redaction, minimal telemetry, opt-in diagnostics | Automated log scans and tests find no secrets/private payloads |
| Service costs | Upload or download abuse | Rate limits, alerts, emergency switches, monitoring | Simulated abuse triggers alert and route/publishing pause |

### 48.1 Authentication and token storage

- Use Firebase-supported email/password flows with email verification and password reset. Add stronger authentication options later if needed.
- Do not store passwords in VEGA. Keep bearer/refresh credentials in a protected OS credential store, not plain configuration files.
- Use HTTPS only for API traffic; reject insecure endpoints outside local development.
- Do not include tokens in URLs, telemetry, crash dumps, exception messages, or ordinary logs.
- Reauthenticate for especially sensitive actions where supported: changing email/password, deleting an account, export of sensitive account data, and certain security settings.
- Rate-limit abuse-prone endpoints; do not reveal whether an arbitrary email belongs to an account in a way that enables account enumeration.
- Provide clear logout and revoke-session behavior. Local files remain accessible unless the user separately protects them with OS/account encryption.

### 48.2 Local files, encryption, and theft claims

- Recommend OS full-disk/device encryption for users with sensitive work, and explain that VEGA is not an OS lock-screen replacement.
- Protect application secrets with Windows-supported protected storage. Do not invent custom cryptography or rely on a hard-coded encryption key in the executable.
- If implementing end-to-end encrypted private cloud files, use an established audited cryptographic library and a reviewed streaming AEAD/container format. Document recovery consequences before launch.
- If an end-to-end encrypted vault is not implemented, clearly state that cloud private objects rely on provider-managed encryption plus application-level access controls; do not market this as zero-knowledge encryption.
- Do not automatically encrypt or rewrite user files without consent and a recovery plan.
- Cloud session revocation cannot erase previously downloaded/cached files. Explain that limitation in lost-device UI.

### 48.3 Product authenticity and anti-counterfeit protection

- Choose the release channel before production updater implementation: Microsoft Store/MSIX, signed direct distribution, or both. Each has distinct release operations.
- Sign shipped installers/packages and validate the trusted signer at release; sign the update manifest or metadata and bind package hash, version, architecture, release channel, and minimum supported version.
- Prevent unsigned downgrade, wrong-channel update, tampered package, or mismatched hash. Provide a documented recovery path if a signing key is rotated/compromised.
- Keep signing secrets out of source, app binaries, client configs, build artifacts, untrusted pull-request runners, and developer logs.
- Generate a software bill of materials and third-party notices. Protect workflow files and release branches with review/branch controls.
- Security checks must not conceal malware-like behavior, disable antivirus, persist secretly, or frustrate a user's uninstall.

Official Windows signing reference: https://learn.microsoft.com/en-us/windows/msix/package/sign-msix-package-guide

### 48.4 Three-account-per-installation limit

- Define precisely what counts as a registered installation (installation key backed by OS protected storage, not raw hardware serial enumeration).
- Enforce a maximum of three distinct Firebase UIDs linked to an installation in D1, atomically under concurrent requests.
- Users can unlink an account/device through authorized UI or support recovery, with abuse safeguards.
- Reinstalling/resetting protected data may create a new installation identity; do not claim this policy is a foolproof physical-computer limit.
- Avoid collecting invasive hardware fingerprint components. Do not use covert tracking to compensate for inherent client-control limitations.
- Log only the minimum association metadata and retain it for a declared period.

### 48.5 Upload and parser hardening

- Check file signatures/magic bytes as well as extension and declared MIME type; metadata from clients is untrusted.
- Use server-generated object keys and keep object paths separate from user-supplied display names.
- Enforce content-type allowlists for each category, request size and decoder resource limits, image dimension/pixel-count limits, compressed/decompressed size limits for archive-like packages, and time budgets.
- Validate PSD/PSB, archives, fonts, SVG-like formats, presets, and plugin packages with parsers that do not execute embedded content.
- Prefer generating thumbnails and previews in a sandboxed or isolated bounded worker/process. Strip or avoid serving active content inline from the same trusted origin as the API.
- Quarantine first, scan/parse, generate preview, reconcile checksum/size, then publish. Failed validation never becomes publicly available.
- Keep private and public R2 objects logically separated by key prefix and preferably separate buckets/environments; authorization policy is still mandatory even if object keys are unguessable.
- Set safe content-disposition/cache headers, use short-lived signatures when direct URLs are required, and prevent public CDN caching of private objects.

Reference: https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html

### 48.6 Plugin trust model

- Plugin signature establishes package integrity/publisher continuity; it does not prove that the plugin is safe or benign.
- Catalog moderation/review status, user reports, publisher identity, release version, and permissions are separate metadata fields.
- Default permissions: no arbitrary filesystem access, no network, no process execution, no environment secrets, and no unrestricted document access.
- Every capability call is checked by the host, with schema validation and user consent for sensitive operations.
- Bound CPU/fuel, wall-clock execution, memory, output payload sizes, host calls, recursion/call depth where available, and concurrent jobs.
- Do not allow plugins to intercept passwords, auth tokens, raw cloud credentials, or unrestricted project paths.
- Provide disable-all-plugins safe mode if a plugin prevents startup. Failed plugin migration must not corrupt documents.
- Record which plugin version touched a document where that helps reproducibility, but don't claim the original artwork can be restored if the plugin performed a destructive action outside undo history.

### 48.7 Copyright and content ownership

- Publishing UI should ask for an explicit license/permission choice and remind publishers to share only material they own or are authorized to redistribute.
- Include report, takedown, counter-notice/appeal, repeat-abuser, and ownership-appeal processes appropriate to the actual jurisdictions where VEGA is operated.
- Preserve only evidence needed for abuse/security operations and deletion/audit obligations. Define content retention in a published policy before public release.
- A public listing should show author/publisher, version, description, preview, size, compatibility, license/attribution declaration, moderation status, and download count if tracked.
- Provide an option to disable public-profile discovery if desired; don't publish private documents by default.
- VEGA cannot guarantee that downloaded/captured content will not be copied. Optional creator attribution/watermarking can be offered, not forced into editable documents.

### 48.8 Rate limiting and service abuse

Use multiple dimensions where justified: IP/network, account UID, installation association, endpoint, object, and short-term burst. Do not rely on any one dimension because IP addresses can be shared or changed. Make limits transparent and tuneable. Use stronger checks for high-cost or abuse-prone operations such as upload session creation, publishing, plugin package submission, large downloads, password reset, and account linking. Use rate-limiter bindings or a compatible storage-backed approach only after checking their semantics and deployment environment.

No per-user public cumulative storage quota is part of the intended product model. Safety limits include request rates, a documented per-object maximum, platform/runtime constraints, validation constraints, legal policy, emergency service pause, and infrastructure budget controls. These are not to be represented as a paid-tier system.

### 48.9 Audit, telemetry, and incident response

- Audit security-relevant events: authentication failures in aggregate, new installation association, revocation, private-file permission denials, quota anomalies, publish/unpublish, plugin reports/revocations, admin moderation, and destructive account deletion.
- Never log file bytes, passwords, bearer tokens, private document contents, full plugin source by default, or unnecessary precise local paths.
- Restrict access to operational logs; define retention and deletion windows.
- Provide alerting for unusual storage growth, error spikes, abusive request patterns, failed validation surges, costs, and production configuration changes.
- Keep runbooks for compromised user accounts, compromised signing key, malicious plugin, private-data exposure, provider outage, quota corruption, and takedown request.
- Each runbook defines an incident owner, containment step, user communication, evidence retention, recovery, and post-incident follow-up.
- A production upload kill-switch must pause new public uploads/publishing without deleting data or disabling ordinary local editing.

---

## 49. Privacy, data lifecycle, and operational policy

### 49.1 Data inventory

Before public beta, maintain a data inventory that states for each field/object: purpose, location, owner, access, retention period, deletion behavior, backup behavior, and whether the data is optional. Include account profile, installation association, private cloud metadata, project objects, public-share metadata, plugin packages, reports, moderation audit data, logs, and model downloads.

### 49.2 Account deletion

- Deletion flow identifies which cloud objects, public posts, plugin packages, metadata, and account associations will be deleted, transferred, unpublished, or retained for legal/safety obligations.
- Require reauthentication or a confirmation step appropriate to the risk.
- Make local files independent of account deletion and explain that local files are not remotely removed.
- Design deletion jobs as idempotent state machines with retries and visible status; don't mark deletion complete before the documented deletion tasks finish.
- Backups may retain deleted records until their published expiration window, where necessary for safe recovery; this limitation is disclosed.
- Public content may be unpublished/deleted in response to account deletion subject to published community policy and records required for abuse response.

### 49.3 Minimal telemetry

- Default to local diagnostics and structured error messages without uploading artwork.
- If optional telemetry is later added, clearly describe data categories and purpose, allow opt-in/opt-out where appropriate, and never collect image pixels or document text by default.
- Crash reports must be reviewed for secret leakage before upload; offer a preview and a way to remove local paths or user content.
- Use aggregate events for performance/feature analysis rather than per-keystroke or canvas-interaction surveillance.

### 49.4 Service availability and graceful degradation

- Local open/edit/save/export must work if Firebase, Cloudflare, or the network is unavailable.
- Cloud screens show offline state and retry options; they don't freeze the editor.
- Service outages must never clear a local file or replace a local document with stale remote content.
- Backend errors are logged with request/correlation IDs that are safe to expose. Client shows a support reference but not internal data.
- Database migrations must be backward-compatible with the currently deployed Worker during rollout when feasible; deploy expand/migrate/contract steps rather than a single incompatible cutover.


### 49.5 Sustainable operation without charging users

VEGA's product policy is free-to-use with no subscriptions, paid features, advertisements, or account-tier paywalls. The operator must still budget for object storage, requests, Worker/D1 activity, support, moderation, signing/distribution, incident response, and legal compliance. Candidate funding sources such as owner funding, voluntary donations, grants, or non-exclusive sponsorship can be evaluated separately; none may silently change the product's free core or introduce tracking-based monetization. Do not promise limitless infrastructure capacity. If safety or cost requires temporarily pausing public intake, preserve existing user files, publish a status notice, explain the reason, and keep local editing functional. The no-per-user cumulative public-storage quota remains the intended product policy unless the owner explicitly changes it.

---

## 55. Consolidated security implementation checklist

This checklist incorporates the separate security plan into the master specification; maintain it in this document or in a generated issue tracker, not as a competing source of truth.

### Before authenticated cloud beta

- [ ] Firebase email/password sign-up, verification, reset, logout.
- [ ] Worker-side JWT signature/issuer/audience/expiry/subject validation.
- [ ] Owner authorization/account status check on every private object route.
- [ ] OS-protected storage for credentials and installation key material.
- [ ] Private/public storage separation and environment isolation.
- [ ] Race-safe quota reservation, size verification, idempotent finalization, abandoned-upload cleanup, reconciliation.
- [ ] Device/session view, unlink/revoke, account recovery path.
- [ ] Rate limits for sign-in-adjacent routes, upload start/finalize, delete, publish, and device association.
- [ ] Server-generated object keys, safe cache/content-disposition, short-lived authorized URLs.
- [ ] Account/content deletion and backup-restore tests.

### Before opening public uploads

- [ ] Real type/signature validation rather than trusting extension/MIME.
- [ ] Documented object-size and parser limits.
- [ ] Safe previews that do not render active user content in a trusted origin.
- [ ] Report, takedown, appeal, and moderation state machine.
- [ ] Cost alert and public upload/publish emergency pause.
- [ ] Private/public object separation, versioning, deletion, and restoration tests.
- [ ] Legal/content policy and operator contact ready.

### Before enabling executable plugins

- [ ] Resource-pack path shipped first.
- [ ] Versioned package manifest and canonical package hash.
- [ ] Signature/publisher identity and review status separated clearly.
- [ ] Host-enforced default-deny capabilities and validated host calls.
- [ ] CPU/time/memory/output/network/filesystem limits.
- [ ] Install-time permission display and consent.
- [ ] Disable, uninstall, rollback, report, and catalog revocation.
- [ ] Adversarial tests for malformed packages, infinite loops, excessive output, invalid signatures, and revoked versions.

### Before production release

- [ ] Signed installer/package and signed/verified updater metadata.
- [ ] SBOM and required third-party notices.
- [ ] Protected release branch/workflow and isolated signing secrets.
- [ ] Backup restore, quota reconciliation, deletion jobs, incident response, key rotation, and release rollback drilled.
- [ ] Privacy notice, terms/community rules, support contact, vulnerability-reporting path, and takedown policy published.
