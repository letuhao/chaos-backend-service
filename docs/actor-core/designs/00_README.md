# Actor Core v3 — Professional Design (English, **Merged**)
**Updated:** 2025-09-08 00:15

This is the **single, authoritative design pack** for Actor Core v3 (Aggregator‑only).
It merges the base design with **Realm‑Scoped Caps** and **Flexible Multi‑Layer Caps** (REALM/WORLD/EVENT/TOTAL)
using an explicit **CapLayerRegistry** and deterministic **across‑layer reduction**.

## 🚀 Performance Optimizations

For games with many subsystems where performance is critical, see **[23_Performance_Optimizations.md](23_Performance_Optimizations.md)** for detailed optimization strategies including:
- Intelligent caching (10-100x performance gain)
- Lazy evaluation & incremental updates (5-20x performance gain)
- Parallel processing (2-8x performance gain)
- Memory optimization & zero allocations
- Real-time performance monitoring

## 📚 Additional Guides

- **[24_Subsystem_Development_Guide.md](24_Subsystem_Development_Guide.md)** - Comprehensive guide for developing robust, performant subsystems
- **[25_Production_Deployment_Guide.md](25_Production_Deployment_Guide.md)** - Production deployment, monitoring, and operations guide

> Docs only — no implementation code. A separate **Cursor AI Execution Guide** will be delivered **after** you sign off this pack.
