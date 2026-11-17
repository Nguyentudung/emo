# Veyra — Alpha Roadmap

**Goal:** Produce a genuinely usable Alpha that can open, view, and perform basic processing on images, with a resizable native window, responsive UI, and solid error handling. Video and advanced library features are secondary if time-constrained.

## Phase 1: Audit & Documentation (Done)
- Inspect all crates.
- Create/update PROJECT_STATUS.md, ARCHITECTURE.md, ALPHA_ROADMAP.md, CHANGELOG.md.

## Phase 2: Core Infrastructure Fixes
- Ensure clean `cargo check` / `cargo test`.
- Confirm Slint window is freely resizable (native edges/corners).
- Set minimum window size.
- Support maximized / restored states.

## Phase 3: Responsive Desktop UI + Image Viewer Polish
- Responsive sidebar (collapsible or adaptive width).
- Aspect-ratio preserving image display.
- Mouse pan (drag to move).
- Proper Fit / Actual Size / Zoom In-Out.
- Pixel rotation applied to displayed image (or transform).
- Fullscreen toggle.
- Keyboard shortcuts (Ctrl+O, arrows, +/-, 0, 1, F11, Esc).
- Tooltips, better empty/error/loading states.
- Status bar with more info (file size, format).

## Phase 4: Image Processing Tools
- Resize (custom size, percentage, preserve aspect).
- Format conversion (JPEG, PNG, WebP, etc.).
- Quality / compression controls.
- Export to chosen directory (never overwrite original silently).
- Basic crop.
- Batch processing with progress and cancellation.

## Phase 5: Media Library & Thumbnails
- Improved directory browsing.
- Thumbnail generation + caching.
- Grid view option.
- Sorting / filtering.

## Phase 6: Metadata
- Extract and display width/height, format, file size, EXIF (where available).

## Phase 7: Video Player (Stretch)
- Backend evaluation (libmpv or GStreamer).
- Basic play/pause/seek/volume if backend available.
- Document exact system dependencies if not.

## Phase 8: Performance & Stability
- Async task manager with cancellation.
- Bounded concurrency.
- Memory-efficient decoding.
- Clippy clean where practical.
- Error handling for all user-facing paths.

## Phase 9: Alpha Packaging
- Linux build instructions.
- Known limitations list.
- CHANGELOG updated.
- Final verification: launch, resize, image open/navigate, basic process.

## Priority Order
1. Resizable + responsive UI
2. Solid Image Viewer (fit, pan, zoom, rotate, nav, shortcuts)
3. Basic image processing + export
4. Thumbnails + metadata
5. Video (if feasible)
6. Polish & docs
