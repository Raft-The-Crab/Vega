# VEGA Planning Set v2.1

Start with [`plan.md`](plan.md). This is the hybrid UI/core plan for VEGA: C#/.NET 11 + Avalonia for the Windows-first/macOS-later desktop UI, Rust for the shared document and editing engine, Kotlin + Jetpack Compose for Android 11+, and TypeScript for the Cloudflare Worker backend.

Read [`architecture.md`](architecture.md), [`dotnet-development.md`](dotnet-development.md), and [`rust-development.md`](rust-development.md) before changing the application foundation. .NET 11/Avalonia compatibility must pass the documented Phase 0 gate before feature development scales up.
