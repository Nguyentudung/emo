# Veyra — Architecture

## Overview

Veyra uses a Cargo Workspace with clear separation between UI and media processing.

```
veyra/
├── desktop/          # Slint UI, main loop, callbacks, AppState
├── media-core/       # Shared types (MediaFile, MediaType)
├── image-engine/     # Image load, decode, info, transform helpers
├── filesystem/       # Directory scanning
├── cache-engine/     # LRU cache
├── video-engine/     # Video playback (planned)
├── thumbnail-engine/ # Thumbnail generation (planned)
├── metadata-engine/  # EXIF / media info (planned)
└── task-manager/     # Async task orchestration (planned)
```

## Principles

- UI (`desktop`) must not contain heavy media processing logic.
- Engines are independent and reusable.
- Long-running work runs off the UI thread.
- Errors are explicit (`anyhow` currently; prefer typed errors later).
- No circular dependencies.

## Current Data Flow (Image Viewer)

1. User clicks "Open Folder" → `rfd` dialog → `filesystem::scan_directory`.
2. File list is set on the Slint `files` property.
3. On selection / navigation → background thread calls `image_engine::decode_to_rgba`.
4. RGBA buffer is converted to `slint::SharedPixelBuffer` on the UI thread and set as `current-image`.
5. Zoom / rotation state is kept in both Rust `AppState` and Slint properties.

## Window Resizing

Slint `Window` is resizable by default on the winit desktop backend.  
Setting `width` / `height` only defines the initial preferred size.  
Layout uses `HorizontalBox` / `VerticalBox` so the main viewing area expands.  
Minimum size should be set via `min-width` / `min-height` on the Window.

## Future Improvements

- Introduce a proper TaskManager for cancellation and progress.
- Use `image-engine` for all transforms (resize, convert, crop) and expose them through a clean API.
- ThumbnailEngine should depend on CacheEngine and ImageEngine.
- MetadataEngine can wrap `kamadak-exif` or similar later.
- VideoEngine should hide the concrete backend (mpv/GStreamer) behind a trait.

## Dependencies (high-level)

- UI: `slint`
- Images: `image`
- Files: `walkdir`, `rfd`
- Async: `tokio` (available), currently using `std::thread`
- Cache: `lru`
