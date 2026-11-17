# Veyra Media Viewer

High-performance native Rust desktop application for viewing images (and videos in future).

## Architecture

Modular Cargo Workspace:

| Crate | Responsibility | Status |
|-------|----------------|--------|
| `desktop` | Slint UI, application entry, state management | Image viewer logic implemented |
| `media-core` | Shared media types | Basic types done |
| `image-engine` | Decoding to RGBA, info, rotation, format checks | Functional + tested |
| `filesystem` | Directory scanning for images/videos | Functional |
| `cache-engine` | LRU memory cache | Basic implementation |
| `video-engine` | Video playback abstraction | Stub (no libmpv) |
| `thumbnail-engine` | Progressive thumbnails | Stub |
| `task-manager` | Background workers | Stub |
| `metadata-engine` | EXIF / metadata | Stub |

## Milestone 1: Image Viewer — Progress

**Implemented in code:**
- Background image decoding (`decode_to_rgba`) off UI thread via `std::thread`.
- Conversion of decoded RGBA into `slint::SharedPixelBuffer` + `Image`.
- AppState managing current file list, index, zoom, rotation.
- Callbacks wired for: Open Folder, Select from gallery, Zoom In/Out, Fit, 100%, Rotate L/R, Prev/Next.
- Gallery list with selection highlight.
- Error handling for invalid images.
- Status bar updates with dimensions and filename.
- Unit tests for supported extensions and format checks.

**UI features present:**
- Dark theme, sidebar gallery, toolbar, image area, status bar.
- TouchArea on gallery items for selection.
- Image component bound to dynamic `current-image` property.

**Environment limitations (sandbox):**
- Cannot install `pkg-config` or `libfontconfig` → Slint software renderer build fails at `yeslogic-fontconfig-sys`.
- Cannot install `libmpv` → video remains stub.
- Full interactive pan/fullscreen and pixel-accurate rotation of displayed image require a successful Slint link.
- Tested: `cargo test -p image-engine` passes.

## Build & Run

On Ubuntu:

```bash
sudo apt install pkg-config libfontconfig1-dev libxkbcommon-dev
cd ~/Documents/veyra
cargo clean
cargo check --workspace
cargo build --release -p desktop
cargo test --workspace
```

**Important:** After any change to `desktop/ui/main.slint`, run `cargo clean -p desktop` (or full `cargo clean`) so that `slint-build` regenerates the Rust bindings. Stale generated code in `target/` causes private method errors even after the `.slint` file is fixed.

## Known Limitations

- UI binary cannot be produced in this sandbox due to missing fontconfig.
- Pan by mouse drag and true fullscreen are not yet implemented (code structure ready for them).
- Rotation currently updates state; pixel buffer re-rotation on demand is prepared but not fully applied to display.
- Thumbnail generation and persistent settings are future milestones.

## Roadmap

- [x] Image decoding pipeline + background loading
- [x] Gallery selection + navigation
- [x] Zoom state and basic controls
- [ ] Mouse pan
- [ ] Fullscreen
- [ ] Thumbnail engine
- [ ] Settings persistence
- [ ] Video backend (when libmpv available)
- [ ] Benchmarks

## Testing

```bash
cargo test -p image-engine
# Additional integration tests can be added once UI builds.
```
