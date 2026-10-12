# Backend

This directory contains the TypeScript Cloudflare Worker backend for VEGA's online services.

## Structure

- **worker/** - Main Cloudflare Worker application
- **contracts/** - API contracts, OpenAPI specs, and shared type definitions

## Backend Stack

- **Runtime:** Cloudflare Workers (TypeScript)
- **Database:** Cloudflare D1 (SQLite-compatible)
- **Storage:** Cloudflare R2 (S3-compatible object storage)
- **Authentication:** Firebase Authentication

## Services

- Account management and authentication
- Private cloud storage (250MB per account)
- Public/unlisted sharing
- Plugin catalog and distribution
- Content moderation

## Build

```bash
cd backend/worker
npm install
npm run build
npm test
```

## Documentation

See `../docs/reference/backend.md` for detailed backend architecture and API documentation.
