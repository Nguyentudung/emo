# Changelog

## [Unreleased] — Toward Alpha

### Added
- PROJECT_STATUS.md, ALPHA_ROADMAP.md, ARCHITECTURE.md, CHANGELOG.md
- Initial audit of all crates
- Image engine: `resize_image`, `resize_preserve_aspect`, `save_image`, `convert_and_save` helpers
- UI: min-width/min-height for resizable window
- UI: aspect-ratio preserving display size calculation
- UI: drag-to-pan support
- UI: basic keyboard shortcuts (arrows, +/-, 0, 1, F11)
- AppState tracks original image dimensions

### Changed
- Full GNOME Adwaita-style redesign
  - Native-looking headerbar
  - Nautilus-inspired sidebar
  - Adwaita symbolic icons (from adwaita-icon-theme)
  - Adwaita dark color palette
- Image viewer keeps fit-scale + clamped pan (image cannot leave viewport)

### Fixed
- Slint type errors in main.slint (length/float conversion, preferred-width/height instead of width+min-width, mouse cursor)
- Window remains freely resizable with min size constraints

## [0.1.0] — Initial skeleton

- Cargo workspace with modular crates
- Basic image decoding pipeline
- Slint UI with sidebar, toolbar, status bar
- Folder scanning and image display
- Unit tests in image-engine
