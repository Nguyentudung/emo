# Veyra — Project Status

**Last updated:** 2026-11-16  
**Version:** 0.1.0 (pre-Alpha)  
**Target:** Functional Alpha Release

## Executive Summary

Veyra is a modular Rust desktop media application using Slint for UI. The image viewing pipeline is partially functional. Most other features (video, thumbnails, metadata, batch processing, task management) are stubs or missing. The desktop binary builds and runs on a properly configured Linux system after `cargo clean` when Slint files change.

## Crate Status

| Crate | Status | Notes |
|-------|--------|-------|
| `desktop` | Partial | Image viewer UI + logic implemented. Window resizable by default. Missing pan, proper aspect-ratio zoom, fullscreen, keyboard shortcuts, responsive improvements, image processing UI. |
| `image-engine` | Functional | Decode to RGBA, info extraction, rotation helpers, supported format checks. Has unit tests. |
| `filesystem` | Functional | Recursive directory scan (max depth 2) for images/videos. |
| `media-core` | Basic | Shared `MediaFile` type and traits. |
| `cache-engine` | Basic | Thread-safe LRU cache. |
| `video-engine` | Stub | Empty. |
| `thumbnail-engine` | Stub | Empty. |
| `task-manager` | Stub | Empty. |
| `metadata-engine` | Stub | Empty. |

## Implemented Features

- Open folder and list supported images in sidebar.
- Background decoding of images to RGBA and display in Slint.
- Previous / Next navigation.
- Zoom in / out / fit / 100% with aspect-ratio preserving layout.
- Drag-to-pan.
- Rotate left / right (visual transform).
- Error display for failed loads.
- Status bar with dimensions + filename.
- Basic dark-themed layout with minimum window size (800×600).
- Keyboard shortcuts for navigation and zoom.
- Image-engine helpers for resize and format conversion (backend ready).

## Critical Gaps for Alpha

1. **Native window resizing** — supported (Slint Window + min-size set). Layout expands correctly.
2. **Proper image viewing** — aspect-ratio fit and pan implemented. Fullscreen still stub.
3. **Image processing tools** — backend helpers added; UI and batch processing still missing.
4. **Video player** — not started.
5. **Thumbnails & metadata** — not started.
6. **Task manager / cancellation** — missing.
7. **Polish** — tooltips, better empty states, consistent dialogs.

## Build Status

- `cargo check --workspace`: Succeeds on developer machine after clean.
- Sandbox: May fail due to missing `pkg-config` / `libfontconfig1-dev`.
- Tests: `image-engine` tests pass. Others have no tests.

## Next Immediate Actions

1. Improve main.slint for responsive layout and proper image fitting.
2. Implement mouse pan and better zoom/fit logic.
3. Add keyboard shortcuts.
4. Expand image-engine with resize/convert helpers.
5. Begin thumbnail-engine and metadata-engine.
6. Document architecture and Alpha roadmap.
